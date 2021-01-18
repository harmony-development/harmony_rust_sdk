//! Rust client implementation for Harmony, powered by [`tonic`].
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

use assign::assign;
use async_mutex::Mutex as AsyncMutex;
use futures::prelude::*;
use http::{uri::PathAndQuery, Uri};
#[cfg(feature = "parking_lot")]
use parking_lot::{Mutex, MutexGuard};
use reqwest::Client as HttpClient;
use tonic::transport::Channel;

type AuthService = crate::api::auth::auth_service_client::AuthServiceClient<Channel>;
type ChatService = crate::api::chat::chat_service_client::ChatServiceClient<Channel>;
type MediaProxyService =
    crate::api::mediaproxy::media_proxy_service_client::MediaProxyServiceClient<Channel>;

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
    homeserver_url: Uri,
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
    /// Create a new [`Client`] from a homeserver [`Uri`] (URL) and an (optional) session.
    ///
    /// If port is not specified in the URL, this will add the default port `2289` to it.
    /// If scheme is not specified, this will assume the scheme is `https`.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::client::*;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(mut homeserver_url: Uri, session: Option<Session>) -> ClientResult<Self> {
        // Add the default scheme if not specified
        if homeserver_url.scheme().is_none() {
            let parts = homeserver_url.into_parts();

            homeserver_url = Uri::builder()
                .scheme("https")
                .authority(parts.authority.unwrap())
                .path_and_query(
                    parts
                        .path_and_query
                        .unwrap_or_else(|| PathAndQuery::from_static("")),
                )
                .build()
                .unwrap();
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

            let uri = Uri::from_parts(assign!(
                homeserver_url.clone().into_parts(),
                {
                    path_and_query: Some(PathAndQuery::from_static("/_harmony/server"))
                }
            ))
            .unwrap();

            if let Ok(response) = http
                .get(&uri.to_string())
                .send()
                .await?
                .json::<Server>()
                .await
            {
                let host: Uri = response.server.parse().unwrap();
                homeserver_url = host;
            }
        };

        // Add the default port if not specified
        if let (None, Some(authority)) = (homeserver_url.port(), homeserver_url.authority()) {
            let new_authority = format!("{}:2289", authority);

            // These unwraps are safe since we use `Uri` everywhere and we are sure that our `new_authority` is
            // indeed a correct authority.
            homeserver_url = Uri::from_parts(
                assign!(homeserver_url.into_parts(), { authority: Some(new_authority.parse().unwrap()) }),
            )
            .unwrap();
        }

        log::debug!(
            "Using homeserver URL {} with session {:?} to create a `Client`",
            homeserver_url,
            session
        );

        let mut endpoint = Channel::builder(homeserver_url.clone());

        // Use tls if scheme is https
        if homeserver_url.scheme_str().unwrap() == "https" {
            endpoint = endpoint.tls_config(tonic::transport::ClientTlsConfig::new())?;
        }

        let channel = endpoint.connect().await?;

        let auth = AuthService::new(channel.clone());
        let chat = ChatService::new(channel.clone());
        let mediaproxy = MediaProxyService::new(channel);

        let data = ClientData {
            homeserver_url,
            auth_status: Mutex::new(AuthStatus::None),
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
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
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
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// assert_eq!(&client.homeserver_url().to_string(), "https://chat.harmonyapp.io:2289/");
    /// # Ok(())
    /// # }
    /// ```
    pub fn homeserver_url(&self) -> &Uri {
        &self.data.homeserver_url
    }

    /// Makes an HMC with homeserver's authority and the given ID.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::{api::Hmc, client::*};
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> error::ClientResult<()> {
    /// # let client = Client::new("chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// assert_eq!(client.make_hmc("404"), Hmc::new("chat.harmonyapp.io:2289".parse().unwrap(), "404"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn make_hmc(&self, id: impl ToString) -> Hmc {
        Hmc::new(
            self.data.homeserver_url.authority().unwrap().clone(),
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

    /// Begin an authentication steps stream for the current authentication session.
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
    }

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
    ) -> ClientResult<(
        impl Stream<Item = ClientResult<crate::api::chat::event::Event>> + Send + Sync,
        impl Sink<EventSource, Error = impl std::fmt::Debug> + Send + Sync,
    )> {
        let (tx, rx) = flume::unbounded();
        for sub in subscriptions {
            tx.send(sub).unwrap();
        }

        let sub = api::chat::stream_events(self, rx.into_stream()).await?;

        Ok((
            sub.map_err(Into::into)
                .map_ok(|outer_event| outer_event.event)
                .filter_map(|result| {
                    // Remove items which dont have an event
                    future::ready(match result {
                        Ok(maybe_event) => maybe_event.map(Ok),
                        Err(err) => Some(Err(err)),
                    })
                }),
            tx.into_sink(),
        ))
    }
}
