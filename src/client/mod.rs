//! Rust client implementation for Harmony, powered by [`tonic`].
//!
//! See the `examples` directory in the repository on how to use this.

/// [`Client`] API implementations.
pub mod api;
/// Error related code used by [`Client`].
pub mod error;

pub use crate::api::auth::Session;
use assign::assign;
pub use error::*;

/// Some crates exported for user convenience.
pub mod exports {
    pub use futures_util;
    pub use reqwest;
}

use crate::api::{
    auth::auth_service_client::AuthServiceClient, chat::chat_service_client::ChatServiceClient,
    mediaproxy::media_proxy_service_client::MediaProxyServiceClient,
};
use api::{auth::*, chat::EventSource};

use futures_util::{future, stream::Stream, StreamExt, TryStreamExt};
use http::{uri::PathAndQuery, Uri};
#[cfg(feature = "use_parking_lot")]
use parking_lot::{Mutex, MutexGuard};
use reqwest::Client as HttpClient;
use std::sync::Arc;
#[cfg(not(feature = "use_parking_lot"))]
use std::sync::{Mutex, MutexGuard};
use tonic::transport::Channel;

type AuthService = AuthServiceClient<Channel>;
type ChatService = ChatServiceClient<Channel>;
type MediaProxyService = MediaProxyServiceClient<Channel>;

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
    pub fn session(&self) -> Option<&Session> {
        match self {
            AuthStatus::None => None,
            AuthStatus::InProgress(_) => None,
            AuthStatus::Complete(session) => Some(session),
        }
    }

    /// Checks whetever authentication is complete or not.
    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthStatus::Complete(_))
    }
}

#[derive(Debug)]
struct ClientData {
    homeserver_url: Uri,
    auth_status: Mutex<AuthStatus>,
    chat: Mutex<ChatService>,
    auth: Mutex<AuthService>,
    mediaproxy: Mutex<MediaProxyService>,
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
    /// ```no_run
    /// # use harmony_rust_sdk::client::*;
    /// # #[tokio::main]
    /// # async fn main() -> ClientResult<()> {
    /// let client = Client::new("https://example.org".parse().unwrap(), None).await?;
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
            chat: Mutex::new(chat),
            auth: Mutex::new(auth),
            mediaproxy: Mutex::new(mediaproxy),
            http,
        };

        Ok(Self {
            data: Arc::new(data),
        })
    }

    fn chat_lock(&self) -> MutexGuard<ChatService> {
        #[cfg(not(feature = "use_parking_lot"))]
        return self
            .data
            .chat
            .lock()
            .expect("chat service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        self.data.chat.lock()
    }

    fn auth_lock(&self) -> MutexGuard<AuthService> {
        #[cfg(not(feature = "use_parking_lot"))]
        return self
            .data
            .auth
            .lock()
            .expect("auth service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        self.data.auth.lock()
    }

    fn mediaproxy_lock(&self) -> MutexGuard<MediaProxyService> {
        #[cfg(not(feature = "use_parking_lot"))]
        return self
            .data
            .mediaproxy
            .lock()
            .expect("media proxy service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        self.data.mediaproxy.lock()
    }

    fn auth_status_lock(&self) -> MutexGuard<AuthStatus> {
        #[cfg(not(feature = "use_parking_lot"))]
        return self
            .data
            .auth_status
            .lock()
            .expect("auth status mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        self.data.auth_status.lock()
    }

    /// Get the current auth status.
    pub fn auth_status(&self) -> AuthStatus {
        self.auth_status_lock().clone()
    }

    /// Get the stored homeserver URL.
    pub fn homeserver_url(&self) -> &Uri {
        &self.data.homeserver_url
    }

    /// Start an authentication session.
    pub async fn begin_auth(&self) -> ClientResult<()> {
        let auth_id = api::auth::begin_auth(self).await?.auth_id;
        *self.auth_status_lock() = AuthStatus::InProgress(auth_id);
        Ok(())
    }

    /// Request the next authentication step from the server.
    ///
    /// Returns `Ok(None)` if authentication was completed.
    /// Returns `Ok(Some(AuthStep))` if extra step is requested from the server.
    pub async fn next_auth_step(
        &self,
        response: AuthStepResponse,
    ) -> ClientResult<Option<AuthStep>> {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            let step = api::auth::next_step(self, auth_id, response.into()).await?;

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
    pub async fn prev_auth_step(&self) -> ClientResult<AuthStep> {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            api::auth::step_back(self, auth_id).await
        } else {
            Err(ClientError::NoAuthId)
        }
    }

    /// Begin an authentication steps stream for the current authentication session.
    pub async fn auth_stream(
        &self,
    ) -> ClientResult<impl Stream<Item = ClientResult<AuthStep>> + Send + Sync> {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            api::auth::stream_steps(self, auth_id)
                .await
                .map(|stream| stream.map_err(Into::into))
        } else {
            Err(ClientError::NoAuthId)
        }
    }

    /// Subscribe to events coming from specified event sources.
    pub async fn subscribe_events(
        &self,
        subscriptions: Vec<EventSource>,
    ) -> ClientResult<impl Stream<Item = ClientResult<api::chat::event::Event>> + Send + Sync> {
        let sub = api::chat::stream_events(self, futures_util::stream::iter(subscriptions)).await?;

        Ok(sub
            .map_err(Into::into)
            .map_ok(|outer_event| outer_event.event)
            .filter_map(|result| {
                // Remove items which dont have an event
                future::ready(match result {
                    Ok(maybe_event) => maybe_event.map(Ok),
                    Err(err) => Some(Err(err)),
                })
            }))
    }
}
