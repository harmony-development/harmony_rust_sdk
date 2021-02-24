//! Rust client implementation for Harmony, powered by [`hrpc`].
//!
//! See the `examples` directory in the repository on how to use this.

/// [`Client`] API implementations.
pub mod api;
/// Error related code used by [`Client`].
pub mod error;

/// Some crates exported for user convenience.
pub mod exports {
    pub use futures;
    pub use reqwest;
}

#[cfg(feature = "request_method")]
use api::ClientRequest;
use api::{auth::*, chat::EventSource, Hmc};
use error::*;

use std::sync::Arc;
#[cfg(not(feature = "parking_lot"))]
use std::sync::{Mutex, MutexGuard};

use async_mutex::Mutex as AsyncMutex;
use hrpc::url::Url;
#[cfg(feature = "parking_lot")]
use parking_lot::{Mutex, MutexGuard};
use reqwest::Client as HttpClient;

type AuthService = crate::api::auth::auth_service_client::AuthServiceClient;
type ChatService = crate::api::chat::chat_service_client::ChatServiceClient;
type MediaProxyService =
    crate::api::mediaproxy::media_proxy_service_client::MediaProxyServiceClient;

/// Represents an authentication state in which a [`Client`] can be.
#[derive(Debug, Clone)]
pub enum AuthStatus {
    /// [`Client`] is not currently authenticated.
    None,
    /// [`Client`] is in the progress of authenticating.
    InProgress(String),
    /// [`Client`] completed an authentication session and is now authenticated.
    Complete(Session),
}

impl AuthStatus {
    /// Gets the session, if authentication is completed.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::client::*;
    /// let auth_status = AuthStatus::None;
    /// assert!(auth_status.session().is_none());
    /// ```
    pub fn session(&self) -> Option<&Session> {
        match self {
            AuthStatus::None => None,
            AuthStatus::InProgress(_) => None,
            AuthStatus::Complete(session) => Some(session),
        }
    }

    /// Checks whetever authentication is complete or not.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::client::*;
    /// let auth_status = AuthStatus::None;
    /// assert!(!auth_status.is_authenticated());
    /// ```
    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthStatus::Complete(_))
    }
}

#[derive(Debug)]
struct ClientData {
    homeserver_url: Url,
    auth_status: Mutex<AuthStatus>,
    chat: AsyncMutex<ChatService>,
    auth: AsyncMutex<AuthService>,
    mediaproxy: AsyncMutex<MediaProxyService>,
    http: HttpClient,
}

/// Client implementation for Harmony.
#[derive(Clone, Debug)]
pub struct Client {
    data: Arc<ClientData>,
}

impl Client {
    /// Create a new [`Client`] from a homeserver [`Url`] (URL) and an (optional) session.
    ///
    /// If port is not specified in the URL, this will add the default port `2289` to it.
    /// If scheme is not specified, this will assume the scheme is `https`.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::client::*;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// let client = Client::new("https://chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(mut homeserver_url: Url, session: Option<Session>) -> ClientResult<Self> {
        // Add the default scheme if not specified
        if !matches!(homeserver_url.scheme(), "http" | "https") {
            homeserver_url.set_scheme("https").unwrap();
        }

        let http = HttpClient::builder().build()?;

        // If no port specified, attempt to name res
        if homeserver_url.port().is_none() {
            use serde::Deserialize;

            #[derive(Deserialize)]
            struct Server {
                #[serde(rename(deserialize = "h.server"))]
                server: String,
            }

            let url = homeserver_url.join("/_harmony/server").unwrap();

            if let Ok(response) = http
                .get(&url.to_string())
                .send()
                .await?
                .json::<Server>()
                .await
            {
                let host: Url = response.server.parse().unwrap();
                homeserver_url = host;
            }
        };

        // Add the default port if not specified
        if homeserver_url.port().is_none() {
            // These unwraps are safe since we use `Url` everywhere and we are sure that our `new_authority` is
            // indeed a correct authority.
            homeserver_url.set_port(Some(2289)).unwrap();
        }

        log::debug!(
            "Using homeserver URL {} with session {:?} to create a `Client`",
            homeserver_url,
            session
        );

        let auth = AuthService::new(http.clone(), homeserver_url.clone())?;
        let mut chat = ChatService::new(http.clone(), homeserver_url.clone())?;
        let mut mediaproxy = MediaProxyService::new(http.clone(), homeserver_url.clone())?;

        if let Some(session) = &session {
            chat.set_auth_token(Some(session.session_token.clone()));
            mediaproxy.set_auth_token(Some(session.session_token.clone()));
        }

        let data = ClientData {
            homeserver_url,
            auth_status: Mutex::new(session.map_or(AuthStatus::None, AuthStatus::Complete)),
            chat: AsyncMutex::new(chat),
            auth: AsyncMutex::new(auth),
            mediaproxy: AsyncMutex::new(mediaproxy),
            http,
        };

        Ok(Self {
            data: Arc::new(data),
        })
    }

    async fn chat_lock(&self) -> async_mutex::MutexGuard<'_, ChatService> {
        self.data.chat.lock().await
    }

    async fn auth_lock(&self) -> async_mutex::MutexGuard<'_, AuthService> {
        self.data.auth.lock().await
    }

    async fn mediaproxy_lock(&self) -> async_mutex::MutexGuard<'_, MediaProxyService> {
        self.data.mediaproxy.lock().await
    }

    fn auth_status_lock(&self) -> MutexGuard<AuthStatus> {
        #[cfg(not(feature = "parking_lot"))]
        return self
            .data
            .auth_status
            .lock()
            .expect("auth status mutex was poisoned");
        #[cfg(feature = "parking_lot")]
        self.data.auth_status.lock()
    }

    /// Send a request.
    ///
    /// # Example
    /// ```no_run
    /// # use harmony_rust_sdk::client::{*, api::{harmonytypes::UserStatus, chat::profile::{ProfileUpdateRequest, ProfileUpdate}}};
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// client
    ///  .request::<ProfileUpdateRequest, _, _>(
    ///    ProfileUpdate::default()
    ///        .new_status(UserStatus::OnlineUnspecified)
    ///        .new_is_bot(true),
    ///   )
    ///   .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "request_method")]
    pub async fn request<Req: ClientRequest<Resp>, Resp, IntoReq: Into<Req>>(
        &self,
        request: IntoReq,
    ) -> ClientResult<Resp> {
        request.into().request(self).await
    }

    /// Get the current auth status.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::client::*;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("https://chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// assert!(!client.auth_status().is_authenticated());
    /// # Ok(())
    /// # }
    /// ```
    pub fn auth_status(&self) -> AuthStatus {
        self.auth_status_lock().clone()
    }

    /// Get the stored homeserver URL.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::client::*;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("https://chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// assert_eq!(&client.homeserver_url().to_string(), "https://chat.harmonyapp.io:2289/");
    /// # Ok(())
    /// # }
    /// ```
    pub fn homeserver_url(&self) -> &Url {
        &self.data.homeserver_url
    }

    /// Makes an HMC with homeserver's authority and the given ID.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::{api::Hmc, client::*};
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("https://chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// assert_eq!(client.make_hmc("404"), Hmc::new("chat.harmonyapp.io:2289", "404"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn make_hmc(&self, id: impl ToString) -> Hmc {
        Hmc::new(
            format!(
                "{}:{}",
                self.data.homeserver_url.host_str().unwrap(),
                self.data.homeserver_url.port().unwrap()
            ),
            id.to_string(),
        )
    }

    /// Start an authentication session.
    ///
    /// # Example
    /// ```no_run
    /// # use harmony_rust_sdk::client::*;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// client.begin_auth().await?;
    /// // Do auth stuff here
    /// # Ok(())
    /// # }
    /// ```
    pub async fn begin_auth(&self) -> ClientResult<()> {
        let auth_id = api::auth::begin_auth(self, ()).await?.auth_id;
        *self.auth_status_lock() = AuthStatus::InProgress(auth_id);
        Ok(())
    }

    /// Request the next authentication step from the server.
    ///
    /// Returns `Ok(None)` if authentication was completed.
    /// Returns `Ok(Some(AuthStep))` if extra step is requested from the server.
    ///
    /// # Example
    /// ```no_run
    /// # use harmony_rust_sdk::client::{*, api::auth::*};
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// client.begin_auth().await?;
    /// let next_step = client.next_auth_step(AuthStepResponse::Initial).await?;
    /// // Do more auth stuff here
    /// # Ok(())
    /// # }
    /// ```
    pub async fn next_auth_step(
        &self,
        response: AuthStepResponse,
    ) -> ClientResult<Option<AuthStep>> {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            let step = api::auth::next_step(self, AuthResponse::new(auth_id, response)).await?;

            Ok(if let Some(auth_step::Step::Session(session)) = step.step {
                self.chat_lock()
                    .await
                    .set_auth_token(Some(session.session_token.clone()));
                self.mediaproxy_lock()
                    .await
                    .set_auth_token(Some(session.session_token.clone()));
                *self.auth_status_lock() = AuthStatus::Complete(session);
                None
            } else {
                Some(step)
            })
        } else {
            Err(ClientError::NoAuthId)
        }
    }

    /// Go back to the previous authentication step.
    ///
    /// # Example
    /// ```no_run
    /// # use harmony_rust_sdk::client::{*, api::auth::*};
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// client.begin_auth().await?;
    /// // Call next step and whatnot here
    /// // Oops, user wants to do something else, lets go back
    /// let prev_step = client.prev_auth_step().await?;
    /// // Do more auth stuff here
    /// # Ok(())
    /// # }
    /// ```
    pub async fn prev_auth_step(&self) -> ClientResult<AuthStep> {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            api::auth::step_back(self, AuthId::new(auth_id)).await
        } else {
            Err(ClientError::NoAuthId)
        }
    }

    /*/// Begin an authentication steps stream for the current authentication session.
    ///
    /// # Example
    /// ```no_run
    /// # use harmony_rust_sdk::client::*;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// client.begin_auth().await?;
    /// let auth_steps_stream = client.auth_stream().await?;
    /// // Do auth stuff here
    /// # Ok(())
    /// # }
    /// ```
    pub async fn auth_stream(
        &self,
    ) -> ClientResult<impl Stream<Item = ClientResult<AuthStep>> + Send + Sync> {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            api::auth::stream_steps(self, AuthId::new(auth_id))
                .await
                .map(|stream| stream.map_err(Into::into))
        } else {
            Err(ClientError::NoAuthId)
        }
    }*/

    /// Subscribe to events coming from specified event sources.
    ///
    /// # Example
    /// ```no_run
    /// # use harmony_rust_sdk::client::{*, api::chat::EventSource};
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// // Auth here
    /// let event_stream = client.subscribe_events(vec![EventSource::Homeserver, EventSource::Action]).await?;
    /// // Do more auth stuff here
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subscribe_events(
        &self,
        subscriptions: Vec<EventSource>,
    ) -> ClientResult<EventsSocket> {
        let inner = api::chat::stream_events(self).await?;
        let mut socket = EventsSocket { inner };
        for source in subscriptions {
            socket.add_source(source).await?;
        }

        Ok(socket)
    }
}

/// Event subscription socket.
#[derive(Debug)]
pub struct EventsSocket {
    inner: hrpc::client::Socket<crate::api::chat::StreamEventsRequest, crate::api::chat::Event>,
}

impl EventsSocket {
    /// Get an event.
    pub async fn get_event(&mut self) -> Option<ClientResult<crate::api::chat::event::Event>> {
        let res = self.inner.get_message().await?;
        match res {
            Ok(ev) => {
                if let Some(event) = ev.event {
                    Some(Ok(event))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err.into())),
        }
    }

    /// Add a new event source.
    pub async fn add_source(&mut self, source: EventSource) -> ClientResult<()> {
        self.inner
            .send_message(source.into())
            .await
            .map_err(Into::into)
    }
}
