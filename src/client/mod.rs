//! Rust client implementation for Harmony, powered by [`tonic`].
//!
//! See the `examples` directory in the repository on how to use this.

/// [`Client`] API implementations.
pub mod api;
/// Error related code used by [`Client`].
pub mod error;

pub use crate::api::auth::Session;
pub use error::*;
pub use prost::Message;

use crate::api::{
    auth::auth_service_client::AuthServiceClient, chat::chat_service_client::ChatServiceClient,
    mediaproxy::media_proxy_service_client::MediaProxyServiceClient,
};
use api::auth::{next_step_request::form_fields::Field, *};

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
    pub fn session(&self) -> Option<&Session> {
        match self {
            AuthStatus::None => None,
            AuthStatus::InProgress(_) => None,
            AuthStatus::Complete(session) => Some(session),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthStatus::Complete(_))
    }
}

/// A response to an [`AuthStep`].
#[derive(Debug, Clone)]
pub enum AuthStepResponse {
    /// A choice selection.
    Choice(String),
    /// A form.
    Form(Vec<Field>),
    Initial,
}

impl AuthStepResponse {
    /// Create a new [`AuthStepResponse`] with the specified choice.
    #[inline(always)]
    pub fn choice(choice: impl ToString) -> Self {
        Self::Choice(choice.to_string())
    }

    /// Create a new [`AuthStepResponse`] with the specified form fields.
    #[inline(always)]
    pub fn form(fields: Vec<Field>) -> Self {
        Self::Form(fields)
    }

    /// A login choice response.
    #[inline(always)]
    pub fn login_choice() -> Self {
        Self::choice("login")
    }

    /// A register choice response.
    #[inline(always)]
    pub fn register_choice() -> Self {
        Self::choice("register")
    }

    /// Create a login form response from specified email and password.
    pub fn login_form(email: impl ToString, password: impl ToString) -> Self {
        Self::form(vec![
            Field::String(email.to_string()),
            Field::Bytes(password.to_string().into_bytes()),
        ])
    }

    /// Create a register form response from specified email, username and password.
    pub fn register_form(
        email: impl ToString,
        username: impl ToString,
        password: impl ToString,
    ) -> Self {
        Self::form(vec![
            Field::String(email.to_string()),
            Field::String(username.to_string()),
            Field::Bytes(password.to_string().into_bytes()),
        ])
    }
}

impl Into<Option<next_step_request::Step>> for AuthStepResponse {
    fn into(self) -> Option<next_step_request::Step> {
        match self {
            AuthStepResponse::Choice(choice) => {
                Some(next_step_request::Step::Choice(next_step_request::Choice {
                    choice,
                }))
            }
            AuthStepResponse::Form(fields) => {
                Some(next_step_request::Step::Form(next_step_request::Form {
                    fields: fields
                        .into_iter()
                        .map(|f| next_step_request::FormFields { field: Some(f) })
                        .collect(),
                }))
            }
            AuthStepResponse::Initial => None,
        }
    }
}

#[derive(Debug)]
struct ClientData {
    homeserver_url: Uri,
    auth_status: Mutex<AuthStatus>,
    chat: Mutex<ChatService>,
    auth: Mutex<AuthService>,
    mediaproxy: Mutex<MediaProxyService>,
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
    pub async fn new(mut homeserver_url: Uri, session: Option<Session>) -> ClientResult<Self> {
        // Add the default port if not specified
        homeserver_url = if let (None, Some(authority)) =
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

        // Add the default scheme if not specified
        homeserver_url = if homeserver_url.scheme().is_none() {
            Uri::from_parts(assign::assign!(homeserver_url.into_parts(), { scheme: Some("https".parse().unwrap()) })).unwrap()
        } else {
            homeserver_url
        };

        log::debug!(
            "Using homeserver URL {} with session {:?} to create a `Client`",
            homeserver_url,
            session
        );

        let mut endpoint = Channel::builder(homeserver_url.clone());

        if homeserver_url.scheme_str().unwrap() == "https" {
            endpoint = endpoint.tls_config(tonic::transport::ClientTlsConfig::new())?;
        }

        let auth_channel = endpoint.connect().await?;
        let chat_channel = endpoint.connect().await?;
        let mediaproxy_channel = endpoint.connect().await?;

        let auth = AuthService::new(auth_channel);
        let chat = ChatService::new(chat_channel);
        let mediaproxy = MediaProxyService::new(mediaproxy_channel);

        let data = ClientData {
            homeserver_url,
            auth_status: Mutex::new(AuthStatus::None),
            chat: Mutex::new(chat),
            auth: Mutex::new(auth),
            mediaproxy: Mutex::new(mediaproxy),
        };

        Ok(Self {
            data: Arc::new(data),
        })
    }

    fn chat_lock(&self) -> MutexGuard<ChatService> {
        let lock = self.data.chat.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("chat service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    fn auth_lock(&self) -> MutexGuard<AuthService> {
        let lock = self.data.auth.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("auth service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    fn mediaproxy_lock(&self) -> MutexGuard<MediaProxyService> {
        let lock = self.data.mediaproxy.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("media proxy service mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
    }

    fn auth_status_lock(&self) -> MutexGuard<AuthStatus> {
        let lock = self.data.auth_status.lock();

        #[cfg(not(feature = "use_parking_lot"))]
        return lock.expect("auth status mutex was poisoned");
        #[cfg(feature = "use_parking_lot")]
        lock
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
    pub async fn auth_stream(&self) -> ClientResult<tonic::Streaming<AuthStep>> {
        if let AuthStatus::InProgress(auth_id) = self.auth_status() {
            api::auth::stream_steps(self, auth_id).await
        } else {
            Err(ClientError::NoAuthId)
        }
    }

    /// Subscribe to events relating to specified guilds, homeserver or actions.
    pub async fn subscribe_events(
        &self,
        guilds: Vec<u64>,
        actions: bool,
        homeserver: bool,
    ) -> ClientResult<
        impl Stream<Item = ClientResult<api::chat::event::Event>> + Send + Sync + 'static,
    > {
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

    const EMAIL: &str = "rust_sdk_test@example.org";
    const PASSWORD: &str = "123456789Ab";

    const TEST_SERVER: &str = "https://chat.harmonyapp.io:2289";
    const TEST_GUILD: u64 = 2699074975217745925;
    const TEST_CHANNEL: u64 = 2699489358242643973;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    async fn make_client() -> ClientResult<Client> {
        Client::new(TEST_SERVER.parse().unwrap(), None).await
    }

    async fn login_client() -> ClientResult<Client> {
        let client = make_client().await?;

        client.begin_auth().await?;
        client.next_auth_step(AuthStepResponse::Initial).await?;
        client
            .next_auth_step(AuthStepResponse::login_choice())
            .await?;
        client
            .next_auth_step(AuthStepResponse::login_form(EMAIL, PASSWORD))
            .await?;
        assert_eq!(client.auth_status().is_authenticated(), true);

        Ok(client)
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
        let _client = login_client().await?;
        Ok(())
    }

    #[tokio::test]
    async fn preview_guild() -> ClientResult<()> {
        use api::chat::*;
        init();

        let client = login_client().await?;
        let response = guild::preview_guild(&client, InviteId::new("harmony").unwrap()).await?;
        log::info!("Preview guild response: {:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_guild_list() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        let response = api::chat::guild::get_guild_list(&client).await?;
        log::info!("Get guild list response: {:?}", response);

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn get_guild_roles() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        let response = api::chat::permissions::get_guild_roles(&client, TEST_GUILD).await?;
        log::info!("Get guild roles response: {:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_guild_members() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        let response = api::chat::profile::get_guild_members(&client, TEST_GUILD).await?;
        log::info!("Get guild members response: {:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn profile_update() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        api::chat::profile::profile_update(&client, None, Some(api::UserStatus::OnlineUnspecified), None, Some(true)).await?;

        Ok(())
    }

    #[tokio::test]
    async fn get_emote_packs() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        let response = api::chat::emote::get_emote_packs(&client).await?;
        log::info!("Get emote packs response: {:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_guild_channels() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        let response = api::chat::channel::get_guild_channels(&client, TEST_GUILD).await?;
        log::info!("Get guild channels response: {:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_channel_messages() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        let response =
            api::chat::channel::get_channel_messages(&client, TEST_GUILD, TEST_CHANNEL, None)
                .await?;
        log::info!("Get channel messages response: {:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn send_message() -> ClientResult<()> {
        use api::chat::message;

        init();

        let client = login_client().await?;
        message::send_message(
            &client,
            TEST_GUILD,
            TEST_CHANNEL,
            None,
            None,
            Some("test".to_string()),
            None,
            None,
            None,
            None,
            None,
        )
        .await?;

        Ok(())
    }

    #[tokio::test]
    async fn instant_view() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        let instant_view =
            api::mediaproxy::instant_view(&client, "https://duckduckgo.com".parse().unwrap())
                .await?;
        log::info!("Instant view response: {:?}", instant_view);

        Ok(())
    }

    #[tokio::test]
    async fn can_instant_view() -> ClientResult<()> {
        init();

        let client = login_client().await?;
        let can_instant_view =
            api::mediaproxy::can_instant_view(&client, "https://duckduckgo.com".parse().unwrap())
                .await?;
        log::info!("Can instant view response: {:?}", can_instant_view);

        Ok(())
    }

    async fn client_sub(guilds: Vec<u64>, actions: bool, homeserver: bool) -> ClientResult<()> {
        let client = login_client().await?;
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
