use super::*;
use crate::{api::chat::*, client_api};

use hrpc::url::Url;

/// Describes where to subscribe for events.
#[derive(Debug, Clone, Copy)]
pub enum EventSource {
    /// Subscription for a guild's events.
    Guild(u64),
    /// Subscription to homeserver events.
    Homeserver,
    /// Subscription to action events.
    Action,
}

impl From<EventSource> for StreamEventsRequest {
    fn from(o: EventSource) -> StreamEventsRequest {
        StreamEventsRequest {
            request: Some(match o {
                EventSource::Guild(id) => stream_events_request::Request::SubscribeToGuild(
                    stream_events_request::SubscribeToGuild { guild_id: id },
                ),
                EventSource::Homeserver => {
                    stream_events_request::Request::SubscribeToHomeserverEvents(
                        stream_events_request::SubscribeToHomeserverEvents {},
                    )
                }
                EventSource::Action => stream_events_request::Request::SubscribeToActions(
                    stream_events_request::SubscribeToActions {},
                ),
            }),
        }
    }
}

/// A message location. This type can be used as multiple requests.
#[into_request("GetMessageRequest", "DeleteMessageRequest")]
#[derive(new, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageLocation {
    guild_id: u64,
    channel_id: u64,
    message_id: u64,
}

/// Wrapper around a guild ID which can be used as multiple requests.
#[into_request(
    "DeleteGuildRequest",
    "LeaveGuildRequest",
    "GetGuildRequest",
    "GetGuildInvitesRequest",
    "GetGuildChannelsRequest",
    "GetGuildRolesRequest",
    "GetGuildMembersRequest"
)]
#[derive(new, Debug, Clone, Copy, PartialEq, Eq, Into, From)]
pub struct GuildId {
    guild_id: u64,
}

/// Wrapper around an user ID which can be used as multiple requests.
#[into_request("GetUserRequest")]
#[derive(new, Debug, Clone, Copy, PartialEq, Eq, Into, From)]
pub struct UserId {
    user_id: u64,
}

/// Manage and query channels.
pub mod channel;
/// Manage and query emotes and emote packs.
pub mod emote;
/// Manage and query guilds.
pub mod guild;
/// Manage and query invites.
pub mod invite;
/// Manage and query messages.
pub mod message;
/// Manage and query user permissions and roles.
pub mod permissions;
/// Manage and query user profiles.
pub mod profile;

/// Convenience type to create a valid [`TriggerActionRequest`].
#[into_request("TriggerActionRequest")]
#[derive(Debug, Clone, new)]
pub struct TriggerAction {
    guild_id: u64,
    channel_id: u64,
    message_id: u64,
    action_id: String,
    action_data: String,
}

client_api! {
    /// Triggers the specified action.
    request: TriggerActionRequest,
    api_fn: trigger_action,
    service: chat,
}

/// Convenience type to create a valid [`TypingRequest`].
#[into_request("TypingRequest")]
#[derive(Debug, Clone, Copy, new)]
pub struct Typing {
    guild_id: u64,
    channel_id: u64,
}

client_api! {
    /// Notifies the server that the local user is typing.
    request: TypingRequest,
    api_fn: typing,
    service: chat,
}

/// Stream events from the server.
///
/// This endpoint requires authentication.
pub async fn stream_events(
    client: &Client,
) -> ClientResult<hrpc::client::socket::Socket<StreamEventsRequest, Event>> {
    use hrpc::IntoRequest;

    let mut req = ().into_request();
    if let Some(session_token) = client.auth_status().session().map(|s| &s.session_token) {
        req = req.header(
            "Authorization".parse().unwrap(),
            session_token.parse().unwrap(),
        );
    }
    let response = client.chat_lock().await.stream_events(req).await;
    tracing::debug!("Received response: {:?}", response);
    response.map_err(Into::into)
}
