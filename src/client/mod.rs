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
//! use harmony_rust_sdk::client::{Client, Session, api::core::create_guild};
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

pub use crate::api::foundation::Session;
pub use error::*;

use crate::api::{
    core::core_service_client::CoreServiceClient,
    foundation::foundation_service_client::FoundationServiceClient,
    profile::profile_service_client::ProfileServiceClient,
};

use tonic::transport::Channel;

use http::Uri;
#[cfg(feature = "use_parking_lot")]
use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;
#[cfg(not(feature = "use_parking_lot"))]
use std::sync::{Mutex, MutexGuard};

type FoundationService = FoundationServiceClient<Channel>;
type CoreService = CoreServiceClient<Channel>;
type ProfileService = ProfileServiceClient<Channel>;

#[derive(Debug)]
pub(self) struct ClientData {
    pub(self) homeserver_url: Uri,
    pub(self) session: Mutex<Option<Session>>,
    pub(self) core: Mutex<CoreService>,
    pub(self) foundation: Mutex<FoundationService>,
    pub(self) profile: Mutex<ProfileService>,
}

/// Client implementation for Harmony.
#[derive(Clone, Debug)]
pub struct Client {
    pub(self) data: Arc<ClientData>,
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

        let foundation = FoundationService::connect(homeserver_url.clone()).await?;
        let core = CoreService::connect(homeserver_url.clone()).await?;
        let profile = ProfileService::connect(homeserver_url.clone()).await?;

        let data = ClientData {
            homeserver_url,
            session: Mutex::new(session),
            core: Mutex::new(core),
            foundation: Mutex::new(foundation),
            profile: Mutex::new(profile),
        };

        Ok(Self {
            data: Arc::new(data),
        })
    }

    pub(self) fn core_lock(&self) -> MutexGuard<CoreService> {
        let lock = self.data.core.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("core service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    pub(self) fn foundation_lock(&self) -> MutexGuard<FoundationService> {
        let lock = self.data.foundation.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("foundation service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    pub(self) fn profile_lock(&self) -> MutexGuard<ProfileService> {
        let lock = self.data.profile.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("profile service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    pub(self) fn session_lock(&self) -> MutexGuard<Option<Session>> {
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
    pub fn homeserver_url(&self) -> Uri {
        self.data.homeserver_url.clone()
    }

    /// Send a [`api::foundation::login`] request to the server and store the returned session.
    pub async fn login(&self, email: impl ToString, password: impl ToString) -> ClientResult<()> {
        let session = api::foundation::login(self, email.to_string(), password.to_string()).await?;
        *self.session_lock() = Some(session);

        Ok(())
    }

    /// Send a [`api::foundation::register`] request to the server and store the returned session.
    pub async fn register(
        &self,
        email: impl ToString,
        username: impl ToString,
        password: impl ToString,
    ) -> ClientResult<()> {
        let session = api::foundation::register(
            self,
            email.to_string(),
            username.to_string(),
            password.to_string(),
        )
        .await?;
        *self.session_lock() = Some(session);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn client_new() {
        init();
        let url: Uri = "http://127.0.0.1".parse().unwrap();
        let client = Client::new(url.clone(), None).await.unwrap();
        assert_eq!(
            client.homeserver_url(),
            "http://127.0.0.1:2289".parse::<Uri>().unwrap()
        );
    }

    #[tokio::test]
    async fn client_new_with_session() {
        init();
        let url: Uri = "http://127.0.0.1".parse().unwrap();
        let session = Session {
            session_token: String::from("secret"),
            user_id: 123456789,
        };
        let client = Client::new(url.clone(), Some(session.clone()))
            .await
            .unwrap();
        assert_eq!(client.session(), Some(session))
    }
}
