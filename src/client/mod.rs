//! Rust client implementation for Harmony, powered by [`tonic`].
//!
//! # Usage
//!
//! Begin by creating a [`Client`] type, then either log in or register:
//!
//! ```no_run
//! use harmony_rust_sdk::client::Client;
//!
//! let work = async {
//!     let homeserver_url = "https://example.org".parse().unwrap();
//!     let client = Client::new(homeserver_url, None).await?;
//!
//!     // Login:
//!     let session = client
//!         .login("example@example.com", "password")
//!         .await?;
//!
//!     // Or register:
//!     let session = client
//!         .register("example@example.com", "example", "password")
//!         .await?;
//!
//!     // You're now logged in / registered!
//!     // Write the session (and homeserver URL!) to a file if you want to restore it later.
//!
//!     // make calls to the API here
//! # harmony_rust_sdk::client::ClientResult::Ok(())
//! };
//! ```
//!
//! You can also pass an existing session to the [`Client`] constructor to restore a previous session
//! rather than calling [`Client::login`]:
//!
//! ```no_run
//! use harmony_rust_sdk::client::{Client, Session};
//!
//! let work = async {
//!     let homeserver_url = "https://example.org".parse().unwrap();
//!     let session = Session {
//!         session_token: "secret".to_string(),
//!         user_id: 123456789,
//!     };
//!     let client = Client::new(homeserver_url, Some(session));
//!
//!     // make calls to the API here
//! # harmony_rust_sdk::client::ClientResult::Ok(())
//! };
//! ```
//!
//! You can also use the API methods in the [`api`] module
//! (note that you *won't* be able to store a [`Session`] that you got from these APIs inside a [`Client`]):
//! ```no_run
//! use harmony_rust_sdk::client::{Client, Session, api::chat::create_guild};
//!
//! let work = async {
//!     let homeserver_url = "https://example.org".parse().unwrap();
//!     let client = Client::new(homeserver_url, None).await?;
//!
//!     // Auth here
//!
//!     // Create a guild and get the guild_id from the response
//!     let created_guild_id = create_guild(&client, String::from("Example Guild"), None).await?.guild_id;
//!
//!     // make more API calls
//! # harmony_rust_sdk::client::ClientResult::Ok(())
//! };
//! ```

/// [`Client`] API implementations.
pub mod api;
/// Error related code used by [`Client`].
pub mod error;

pub use crate::api::auth::Session;
pub use error::*;
pub use prost::Message;

use crate::api::{
    auth::auth_service_client::AuthServiceClient, chat::chat_service_client::ChatServiceClient,
};

use futures_util::{
    future,
    stream::{self, Stream},
    StreamExt, TryStreamExt,
};
use http::Uri;
#[cfg(feature = "use_parking_lot")]
use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;
#[cfg(not(feature = "use_parking_lot"))]
use std::sync::{Mutex, MutexGuard};
use tonic::transport::Channel;

type AuthService = AuthServiceClient<Channel>;
type ChatService = ChatServiceClient<Channel>;

#[derive(Debug)]
struct ClientData {
    homeserver_url: Uri,
    session: Mutex<Option<Session>>,
    chat: Mutex<ChatService>,
    auth: Mutex<AuthService>,
}

/// Client implementation for Harmony.
#[derive(Clone, Debug)]
pub struct Client {
    data: Arc<ClientData>,
}

impl Client {
    /// Create a new [`Client`] from a homeserver [`Uri`] (URL) and an (optional) session.
    ///
    /// If port is not specified in the URL, this will add the default port `2289` to it.
    pub async fn new(homeserver_url: Uri, session: Option<Session>) -> ClientResult<Self> {
        // Add the default port if not specified
        let homeserver_url = if let (None, Some(authority)) =
            (homeserver_url.port(), homeserver_url.authority())
        {
            let new_authority = format!("{}:2289", authority);

            // These unwraps are safe since we use `Uri` everywhere and we are sure that our `new_authority` is
            // indeed a correct authority.
            Uri::from_parts(
                assign::assign!(homeserver_url.into_parts(), { authority: Some(new_authority.parse().unwrap()) }),
            ).unwrap()
        } else {
            homeserver_url
        };

        log::debug!(
            "Using homeserver URL {} with session {:?} to create a `Client`",
            homeserver_url,
            session
        );

        let foundation = AuthService::connect(homeserver_url.clone()).await?;
        let core = ChatService::connect(homeserver_url.clone()).await?;

        let data = ClientData {
            homeserver_url,
            session: Mutex::new(session),
            chat: Mutex::new(core),
            auth: Mutex::new(foundation),
        };

        Ok(Self {
            data: Arc::new(data),
        })
    }

    fn chat_lock(&self) -> MutexGuard<ChatService> {
        let lock = self.data.chat.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("core service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    fn auth_lock(&self) -> MutexGuard<AuthService> {
        let lock = self.data.auth.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("foundation service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    fn session_lock(&self) -> MutexGuard<Option<Session>> {
        let lock = self.data.session.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("session mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    /// Get the stored session.
    pub fn session(&self) -> Option<Session> {
        self.session_lock().clone()
    }

    /// Get the stored homeserver URL.
    pub fn homeserver_url(&self) -> &Uri {
        &self.data.homeserver_url
    }

    /// Send a [`api::auth::login`] request to the server and store the returned session.
    pub async fn login(&self, email: impl ToString, password: impl ToString) -> ClientResult<()> {
        let session = api::auth::login(self, email.to_string(), password.to_string()).await?;
        *self.session_lock() = Some(session);

        Ok(())
    }

    /// Send a [`api::auth::register`] request to the server and store the returned session.
    pub async fn register(
        &self,
        email: impl ToString,
        username: impl ToString,
        password: impl ToString,
    ) -> ClientResult<()> {
        let session = api::auth::register(
            self,
            email.to_string(),
            username.to_string(),
            password.to_string(),
        )
        .await?;
        *self.session_lock() = Some(session);

        Ok(())
    }

    pub async fn subscribe_events(
        &self,
        guilds: Vec<u64>,
        actions: bool,
        homeserver: bool,
    ) -> ClientResult<impl Stream<Item = ClientResult<api::chat::event::Event>>> {
        use api::chat::{stream_events, stream_events_request::*};

        let mut requests = guilds
            .into_iter()
            .map(|guild_id| Request::SubscribeToGuild(SubscribeToGuild { guild_id }))
            .collect::<Vec<_>>();

        if actions {
            requests.push(Request::SubscribeToActions(SubscribeToActions {}));
        };

        if homeserver {
            requests.push(Request::SubscribeToHomeserverEvents(
                SubscribeToHomeserverEvents {},
            ));
        };

        stream_events(self, stream::iter(requests))
            .await
            .map(|stream| {
                stream
                    .map_err(Into::into)
                    .map_ok(|outer_event| outer_event.event)
                    .filter_map(|result| {
                        // Remove items which dont have an event
                        future::ready(match result {
                            Ok(maybe_event) => maybe_event.map_or(None, |event| Some(Ok(event))),
                            Err(err) => Some(Err(err)),
                        })
                    })
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    async fn make_client() -> ClientResult<Client> {
        Client::new("http://127.0.0.1".parse().unwrap(), None).await
    }

    #[tokio::test]
    async fn new() -> ClientResult<()> {
        init();
        let _client = make_client().await?;
        Ok(())
    }

    #[tokio::test]
    async fn login() -> ClientResult<()> {
        init();

        let client = make_client().await?;
        client.login("example@example.org", "123456789").await?;

        Ok(())
    }

    #[tokio::test]
    async fn register() -> ClientResult<()> {
        use std::time::{SystemTime, UNIX_EPOCH};
        init();

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let client = make_client().await?;
        client
            .register(
                format!("example{}@example.org", current_time),
                format!("example{}", current_time),
                "123456789",
            )
            .await?;

        Ok(())
    }

    async fn client_sub(guilds: Vec<u64>, actions: bool, homeserver: bool) -> ClientResult<()> {
        let client = make_client().await?;
        client.login("example@example.org", "123456789").await?;
        let _ = client.subscribe_events(guilds, actions, homeserver).await?;

        Ok(())
    }

    #[tokio::test]
    async fn subscribe_nothing() -> ClientResult<()> {
        init();
        client_sub(Vec::new(), false, false).await?;

        Ok(())
    }

    #[tokio::test]
    async fn subscribe_homeserver() -> ClientResult<()> {
        init();
        client_sub(Vec::new(), false, true).await?;

        Ok(())
    }

    #[tokio::test]
    async fn subscribe_actions() -> ClientResult<()> {
        init();
        client_sub(Vec::new(), true, false).await?;

        Ok(())
    }

    #[tokio::test]
    async fn subscribe_actions_and_homeserver() -> ClientResult<()> {
        init();
        client_sub(Vec::new(), true, true).await?;

        Ok(())
    }
}
