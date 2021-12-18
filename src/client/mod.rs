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

use crate::api::{
    auth::{BeginAuthRequest, NextStepResponse, StepBackResponse},
    Endpoint,
};
use api::{auth::*, chat::EventSource, Hmc, HmcFromStrError};
use error::*;

use std::{
    convert::TryFrom,
    fmt::{self, Debug, Formatter},
    future::{self, Future},
    sync::Arc,
};

use hrpc::{
    client::transport::{self as client_transport},
    encode::encode_protobuf_message,
    exports::{
        bytes::{Bytes, BytesMut},
        futures_util::{future::Either, TryFutureExt},
        tower::Service,
    },
    request::BoxRequest,
    Response,
};
use http::Uri;
use parking_lot::{Mutex, MutexGuard, RwLock, RwLockReadGuard};
use reqwest::Client as HttpClient;

#[cfg(feature = "client_web")]
type GenericClientTransport = client_transport::http::Wasm;
#[cfg(all(feature = "client_native", not(feature = "client_web")))]
type GenericClientTransport = client_transport::http::Hyper;
type GenericClient = AddAuth<GenericClientTransport>;

type AuthService = crate::api::auth::auth_service_client::AuthServiceClient<GenericClient>;
type ChatService = crate::api::chat::chat_service_client::ChatServiceClient<GenericClient>;
type MediaProxyService =
    crate::api::mediaproxy::media_proxy_service_client::MediaProxyServiceClient<GenericClient>;
type ProfileService =
    crate::api::profile::profile_service_client::ProfileServiceClient<GenericClient>;
type EmoteService = crate::api::emote::emote_service_client::EmoteServiceClient<GenericClient>;
type BatchService = crate::api::batch::batch_service_client::BatchServiceClient<GenericClient>;

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
    auth_status: Arc<RwLock<(AuthStatus, Bytes)>>,
    chat: Mutex<ChatService>,
    auth: Mutex<AuthService>,
    mediaproxy: Mutex<MediaProxyService>,
    profile: Mutex<ProfileService>,
    emote: Mutex<EmoteService>,
    batch: Mutex<BatchService>,
    http: HttpClient,
}

/// Client implementation for Harmony.
#[derive(Clone, Debug)]
pub struct Client {
    data: Arc<ClientData>,
}

impl Client {
    /// Create a new [`Client`] from a homeserver [`Uri`] and an (optional) session.
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
    pub async fn new(mut homeserver_url: Uri, session: Option<Session>) -> ClientResult<Self> {
        // Add the default scheme if not specified
        if !matches!(homeserver_url.scheme_str(), Some("http" | "https")) {
            homeserver_url = {
                let mut parts = homeserver_url.into_parts();
                parts.scheme = Some("https".parse().unwrap());
                Uri::from_parts(parts).unwrap()
            };
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

            let url = {
                let mut parts = homeserver_url.clone().into_parts();
                parts.path_and_query = Some("/_harmony/server".parse().unwrap());
                Uri::from_parts(parts).unwrap()
            };

            if let Ok(response) = http
                .get(&url.to_string())
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
        if homeserver_url.port().is_none() {
            homeserver_url = {
                let mut parts = homeserver_url.into_parts();
                parts.authority = Some(
                    format!("{}:2289", parts.authority.unwrap().as_str())
                        .parse()
                        .unwrap(),
                );
                Uri::from_parts(parts).unwrap()
            }
        }

        #[cfg(debug_assertions)]
        tracing::debug!(
            "Using homeserver URL {} with session {:?} to create a `Client`",
            homeserver_url,
            session
        );

        let session = session.map_or(AuthStatus::None, AuthStatus::Complete);
        let token_bytes = session.session().map_or_else(Bytes::new, |s| {
            Bytes::copy_from_slice(s.session_token.as_bytes())
        });
        let auth_status = Arc::new(RwLock::new((session, token_bytes)));

        let transport = GenericClientTransport::new(homeserver_url.clone())
            .map_err(|err| ClientError::Internal(InternalClientError::Transport(err)))?;
        let transport = AddAuth {
            inner: transport,
            auth_status: auth_status.clone(),
        };
        let inner = hrpc::client::Client::new(transport);

        let auth = AuthService::new_inner(inner.clone());
        let chat = ChatService::new_inner(inner.clone());
        let mediaproxy = MediaProxyService::new_inner(inner.clone());
        let profile = ProfileService::new_inner(inner.clone());
        let emote = EmoteService::new_inner(inner.clone());
        let batch = BatchService::new_inner(inner);

        let data = ClientData {
            homeserver_url,
            auth_status,
            chat: Mutex::new(chat),
            auth: Mutex::new(auth),
            mediaproxy: Mutex::new(mediaproxy),
            profile: Mutex::new(profile),
            emote: Mutex::new(emote),
            batch: Mutex::new(batch),
            http,
        };

        Ok(Self {
            data: Arc::new(data),
        })
    }

    /// Starts an event loop with the given handler.
    ///
    /// All socket errors will be logged with tracing. If the handler
    /// function returns `Ok(true)` or `Err(err)`, the function will
    /// return, so if you don't want it to return, return `Ok(false)`.
    pub async fn event_loop<'a, Fut, Hndlr>(
        &'a self,
        subs: Vec<EventSource>,
        mut handler: Hndlr,
    ) -> Result<(), ClientError>
    where
        Fut: Future<Output = ClientResult<bool>>,
        Hndlr: FnMut(&'a Client, crate::api::chat::Event) -> Fut + Send + 'a,
    {
        let mut sock = self.subscribe_events(subs).await?;
        loop {
            match sock.get_event().await {
                Ok(Some(ev)) => {
                    let fut = handler(self, ev);
                    if fut.await? {
                        return Ok(());
                    }
                }
                Err(err) => tracing::error!("{}", err),
                _ => std::hint::spin_loop(),
            }
        }
    }

    #[inline(always)]
    /// Get a mutex guard to the chat service.
    pub fn chat(&self) -> MutexGuard<'_, ChatService> {
        self.data.chat.lock()
    }

    #[inline(always)]
    /// Get a mutex guard to the auth service.
    pub fn auth(&self) -> MutexGuard<'_, AuthService> {
        self.data.auth.lock()
    }

    #[inline(always)]
    /// Get a mutex guard to the mediaproxy service.
    pub fn mediaproxy(&self) -> MutexGuard<'_, MediaProxyService> {
        self.data.mediaproxy.lock()
    }

    #[inline(always)]
    /// Get a mutex guard to the profile service.
    pub fn profile(&self) -> MutexGuard<'_, ProfileService> {
        self.data.profile.lock()
    }

    #[inline(always)]
    /// Get a mutex guard to the emote service.
    pub fn emote(&self) -> MutexGuard<'_, EmoteService> {
        self.data.emote.lock()
    }

    #[inline(always)]
    /// Get a mutex guard to the batch service.
    pub fn batch(&self) -> MutexGuard<'_, BatchService> {
        self.data.batch.lock()
    }

    /// Execute the given request.
    pub fn call<Req>(
        &self,
        request: Req,
    ) -> impl Future<Output = ClientResult<Req::Response>> + Send + 'static
    where
        Req: Endpoint,
        Req::Response: prost::Message + Default + 'static,
    {
        let fut = request.call_with(self);
        async move { Ok(fut.await?.into_message().await?) }
    }

    /// Execute the given request, but return a [`hrpc::Response`].
    pub fn call_response<Req>(
        &self,
        request: Req,
    ) -> impl Future<Output = ClientResult<Response<Req::Response>>> + Send + 'static
    where
        Req: Endpoint,
        Req::Response: 'static,
    {
        request.call_with(self)
    }

    /// Execute the given requests a batch (same) request.
    ///
    /// Note that this does not support the convenience types defined in the [`api`] module.
    /// You will need to convert them to the corresponding request type with `Request::from`.
    pub fn batch_call<Req>(
        &self,
        requests: Vec<Req>,
    ) -> impl Future<Output = ClientResult<Vec<<Req as Endpoint>::Response>>> + Send + 'static
    where
        Req: Endpoint + prost::Message,
        <Req as Endpoint>::Response: prost::Message + Default + 'static,
    {
        use prost::Message;

        let encoded = requests
            .iter()
            .map(encode_protobuf_message)
            .map(BytesMut::freeze);
        let batch_req = crate::api::batch::BatchSameRequest {
            endpoint: Req::ENDPOINT_PATH.to_string(),
            requests: encoded.collect(),
        };
        let fut = self.batch().batch_same(batch_req);
        async move {
            let responses = fut.await?.into_message().await?.responses;
            let mut decoded = Vec::with_capacity(responses.len());
            for response in responses {
                let decoded_msg = <Req as Endpoint>::Response::decode(response.as_ref())?;
                decoded.push(decoded_msg);
            }
            Ok(decoded)
        }
    }

    #[inline(always)]
    fn auth_status_lock(&self) -> RwLockReadGuard<(AuthStatus, Bytes)> {
        self.data.auth_status.read()
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
        self.auth_status_lock().0.clone()
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
    /// # let client = Client::new("https://chat.harmonyapp.io:2289".parse().unwrap(), None).await?;
    /// assert_eq!(client.make_hmc("404").unwrap(), Hmc::new("chat.harmonyapp.io:2289", "404").unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn make_hmc(&self, id: impl fmt::Display) -> Result<Hmc, HmcFromStrError> {
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
    pub fn begin_auth(&self) -> impl Future<Output = ClientResult<()>> + Send + 'static {
        let fut = self.auth().begin_auth(BeginAuthRequest {});
        let auth_status_lock = self.data.auth_status.clone();

        async move {
            let resp = fut.await?.into_message().await?;
            auth_status_lock.write().0 = AuthStatus::InProgress(resp.auth_id);
            Ok(())
        }
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
    pub fn next_auth_step(
        &self,
        response: AuthStepResponse,
    ) -> impl Future<Output = ClientResult<Option<NextStepResponse>>> + Send + 'static {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            let auth_status_lock = self.data.auth_status.clone();
            let fut = self
                .auth()
                .next_step(NextStepRequest::new(auth_id, response.into()));

            Either::Left(async move {
                let step = fut.await?.into_message().await?;

                let step = if let Some(AuthStep {
                    step: Some(auth_step::Step::Session(session)),
                    ..
                }) = step.step
                {
                    let token_bytes = Bytes::copy_from_slice(session.session_token.as_bytes());
                    *auth_status_lock.write() = (AuthStatus::Complete(session), token_bytes);
                    None
                } else {
                    Some(step)
                };

                Ok(step)
            })
        } else {
            Either::Right(future::ready(Err(ClientError::NoAuthId)))
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
    pub fn prev_auth_step(
        &self,
    ) -> impl Future<Output = ClientResult<StepBackResponse>> + Send + 'static {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            let fut = self.auth().step_back(StepBackRequest::new(auth_id));
            Either::Left(async move { Ok(fut.await?.into_message().await?) })
        } else {
            Either::Right(future::ready(Err(ClientError::NoAuthId)))
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
    pub fn auth_stream(&self) -> impl Future<Output = ClientResult<AuthSocket>> + Send + 'static {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            let fut = self.auth().stream_steps(AuthId::new(auth_id));
            Either::Left(
                fut.map_ok(|inner| AuthSocket { inner })
                    .map_err(ClientError::from),
            )
        } else {
            Either::Right(future::ready(Err(ClientError::NoAuthId)))
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
    pub fn subscribe_events(
        &self,
        subscriptions: Vec<EventSource>,
    ) -> impl Future<Output = ClientResult<EventsSocket>> + Send + 'static {
        let fut = self.chat().stream_events(());
        async move {
            let (tx, rx) = fut.await?.split();
            let mut socket = EventsSocket {
                write: EventsWriteSocket { inner: tx },
                read: EventsReadSocket { inner: rx },
            };
            for source in subscriptions {
                socket.add_source(source).await?;
            }

            Ok(socket)
        }
    }
}

/// Auth middleware for [`Client`].
#[derive(Debug, Clone)]
pub struct AddAuth<S> {
    inner: S,
    auth_status: Arc<RwLock<(AuthStatus, Bytes)>>,
}

impl<S> Service<BoxRequest> for AddAuth<S>
where
    S: Service<BoxRequest>,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Service::poll_ready(&mut self.inner, cx)
    }

    fn call(&mut self, mut req: BoxRequest) -> Self::Future {
        let guard = self.auth_status.read();
        if guard.0.is_authenticated() {
            req.get_or_insert_header_map().insert(
                http::header::AUTHORIZATION,
                // This is safe on the assumption that servers will never send session tokens
                // with invalid-byte(s). If they do, they aren't respecting the protocol
                unsafe { http::HeaderValue::from_maybe_shared_unchecked(guard.1.clone()) },
            );
        }

        Service::call(&mut self.inner, req)
    }
}

/// Event subscription socket.
pub struct EventsSocket {
    read: EventsReadSocket,
    write: EventsWriteSocket,
}

impl EventsSocket {
    /// Get an event.
    #[inline]
    pub async fn get_event(&mut self) -> ClientResult<Option<crate::api::chat::Event>> {
        self.read.get_event().await
    }

    /// Add a new event source.
    #[inline]
    pub async fn add_source(&mut self, source: EventSource) -> ClientResult<()> {
        self.write.add_source(source).await
    }

    /// Close this socket.
    #[inline]
    pub async fn close(self) -> ClientResult<()> {
        self.write.close().await
    }

    /// Split this socket into the read and write half.
    pub fn split(self) -> (EventsWriteSocket, EventsReadSocket) {
        (self.write, self.read)
    }
}

impl Debug for EventsSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("EventsSocket")
    }
}

/// Write half of the event subscription socket.
pub struct EventsWriteSocket {
    inner: hrpc::client::socket::WriteSocket<crate::api::chat::StreamEventsRequest>,
}

impl EventsWriteSocket {
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

impl Debug for EventsWriteSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("EventsWriteSocket")
    }
}

/// Read half of the event subscription socket.
pub struct EventsReadSocket {
    inner: hrpc::client::socket::ReadSocket<crate::api::chat::StreamEventsResponse>,
}

impl EventsReadSocket {
    /// Get an event.
    pub async fn get_event(&mut self) -> ClientResult<Option<crate::api::chat::Event>> {
        let resp = self.inner.receive_message().await?;
        Ok(resp
            .event
            .and_then(|a| crate::api::chat::Event::try_from(a).ok()))
    }
}

impl Debug for EventsReadSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("EventsReadSocket")
    }
}

/// Auth steps subscription socket.
pub struct AuthSocket {
    inner: hrpc::client::socket::Socket<
        crate::api::auth::StreamStepsRequest,
        crate::api::auth::StreamStepsResponse,
    >,
}

impl AuthSocket {
    /// Get an auth step.
    pub async fn get_step(&mut self) -> ClientResult<Option<crate::api::auth::AuthStep>> {
        self.inner
            .receive_message()
            .await
            .map(|s| s.step)
            .map_err(Into::into)
    }

    /// Close this socket.
    pub async fn close(self) -> ClientResult<()> {
        self.inner.close().await.map_err(Into::into)
    }
}

impl Debug for AuthSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("AuthSocket")
    }
}
