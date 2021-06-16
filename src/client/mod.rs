//! Rust client implementation for Harmony, powered by [`hrpc`].
//!
//! See the `examples` directory in the repository on how to use this.

/// [`Client`] API implementations.
pub mod api;
/// Error related code used by [`Client`].
pub mod error;

/// Some crates exported for user convenience.
pub mod exports {
    pub use reqwest;
}

use api::{auth::*, chat::EventSource, Hmc, HmcFromStrError};
use error::*;

use std::sync::Arc;

use hrpc::{bytes::Bytes, url::Url};
use parking_lot::{Mutex, MutexGuard};
use reqwest::Client as HttpClient;
use tokio::sync::Mutex as AsyncMutex;

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
    token_bytes: Mutex<Bytes>,
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

        tracing::debug!(
            "Using homeserver URL {} with session {:?} to create a `Client`",
            homeserver_url,
            session
        );

        let auth = AuthService::new(http.clone(), homeserver_url.clone())?;
        let chat = ChatService::new(http.clone(), homeserver_url.clone())?;
        let mediaproxy = MediaProxyService::new(http.clone(), homeserver_url.clone())?;

        let session = session.map_or(AuthStatus::None, AuthStatus::Complete);
        let data = ClientData {
            homeserver_url,
            token_bytes: Mutex::new(session.session().map_or_else(Bytes::new, |s| {
                Bytes::copy_from_slice(s.session_token.as_bytes())
            })),
            auth_status: Mutex::new(session),
            chat: AsyncMutex::new(chat),
            auth: AsyncMutex::new(auth),
            mediaproxy: AsyncMutex::new(mediaproxy),
            http,
        };

        Ok(Self {
            data: Arc::new(data),
        })
    }

    /// Consumes self and starts an event loop with the given handler.
    ///
    /// All socket errors will be logged with tracing. If the handler
    /// function returns `Ok(true)` or `Err(err)`, the function will
    /// return, so if you don't want it to return, return `Ok(false)`.
    pub async fn event_loop<'a, Fut, Hndlr>(
        &'a self,
        subs: Vec<EventSource>,
        handler: Hndlr,
    ) -> Result<(), ClientError>
    where
        Fut: std::future::Future<Output = ClientResult<bool>> + 'a,
        Hndlr: Fn(&'a Client, crate::api::chat::event::Event) -> Fut,
    {
        let mut sock = self.subscribe_events(subs).await?;
        loop {
            if let Some(res) = sock.get_event().await {
                match res {
                    Ok(event) => {
                        if handler(&self, event).await? {
                            return Ok(());
                        }
                    }
                    Err(err) => tracing::error!("{}", err),
                }
            }
        }
    }

    #[inline(always)]
    async fn chat(&self) -> tokio::sync::MutexGuard<'_, ChatService> {
        self.data.chat.lock().await
    }

    #[inline(always)]
    async fn auth(&self) -> tokio::sync::MutexGuard<'_, AuthService> {
        self.data.auth.lock().await
    }

    #[inline(always)]
    async fn mediaproxy(&self) -> tokio::sync::MutexGuard<'_, MediaProxyService> {
        self.data.mediaproxy.lock().await
    }

    #[inline(always)]
    fn auth_status_lock(&self) -> MutexGuard<AuthStatus> {
        self.data.auth_status.lock()
    }

    #[inline(always)]
    pub(crate) async fn generic_api_fn<'a, Resp, Req, IntoReq, Handler, HnldrOut>(
        &'a self,
        handler: Handler,
        request: IntoReq,
    ) -> ClientResult<Resp>
    where
        Req: std::fmt::Debug,
        Resp: std::fmt::Debug,
        IntoReq: Into<Req> + std::fmt::Debug,
        Handler: FnOnce(&'a Client, hrpc::Request<Req>) -> HnldrOut,
        HnldrOut: std::future::Future<Output = Result<Resp, InternalClientError>> + 'a,
    {
        use hrpc::IntoRequest;

        let mut request = request.into().into_request();
        if self.data.auth_status.lock().is_authenticated() {
            request = request.header(
                http::header::AUTHORIZATION,
                // This is safe on the assumption that servers will never send session tokens
                // with invalid-byte(s). If they do, they aren't respecting the protocol
                unsafe {
                    http::HeaderValue::from_maybe_shared_unchecked(
                        self.data.token_bytes.lock().clone(),
                    )
                },
            );
        }

        tracing::debug!("Sending request: {:?}", request);
        let response = handler(&self, request).await;
        tracing::debug!("Received response: {:?}", response);

        response.map_err(Into::into)
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
    /// assert_eq!(client.make_hmc("404").unwrap(), Hmc::new("chat.harmonyapp.io:2289", "404").unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn make_hmc(&self, id: impl std::fmt::Display) -> Result<Hmc, HmcFromStrError> {
        let url = &self.data.homeserver_url;
        Hmc::new(
            format!("{}:{}", url.host().unwrap(), url.port().unwrap()),
            id,
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
                *self.data.token_bytes.lock() =
                    Bytes::copy_from_slice(session.session_token.as_bytes());
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
    pub async fn auth_stream(&self) -> ClientResult<AuthSocket> {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            let inner = api::auth::stream_steps(self, AuthId::new(auth_id)).await?;
            Ok(AuthSocket { inner })
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
#[derive(Debug, Clone)]
pub struct EventsSocket {
    inner: hrpc::client::socket::Socket<
        crate::api::chat::StreamEventsRequest,
        crate::api::chat::Event,
    >,
}

impl EventsSocket {
    /// Get an event.
    pub async fn get_event(&mut self) -> Option<ClientResult<crate::api::chat::event::Event>> {
        let res = self.inner.get_message().await?;
        match res {
            Ok(ev) => ev.event.map(Ok),
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

    /// Close this socket.
    pub async fn close(self) -> ClientResult<()> {
        self.inner.close().await.map_err(Into::into)
    }
}

/// Auth steps subscription socket.
#[derive(Debug, Clone)]
pub struct AuthSocket {
    inner: hrpc::client::socket::ReadSocket<
        crate::api::auth::StreamStepsRequest,
        crate::api::auth::AuthStep,
    >,
}

impl AuthSocket {
    /// Get an auth step.
    pub async fn get_step(&mut self) -> Option<ClientResult<crate::api::auth::AuthStep>> {
        let res = self.inner.get_message().await?;
        Some(res.map_err(Into::into))
    }

    /// Close this socket.
    pub async fn close(self) -> ClientResult<()> {
        self.inner.close().await.map_err(Into::into)
    }
}
