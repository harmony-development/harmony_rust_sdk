use super::*;

use crate::{api::chat::*, client_api};
use futures::StreamExt;
use http::Uri;

// Export everything a client may need for this service
pub use crate::api::chat::{
    event, get_emote_pack_emotes_response::Emote, get_emote_packs_response::EmotePack,
    get_guild_channels_response::Channel, get_guild_invites_response::Invite,
    get_guild_list_response::GuildListEntry, permission::Mode, stream_events_request, Event,
    InviteId, PermissionList, Place, Role,
};

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
#[derive(Debug, new)]
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
    request: impl futures::stream::Stream<Item = EventSource> + Send + Sync + 'static,
) -> ClientResult<tonic::Streaming<Event>> {
    use stream_events_request::{Request as SReq, *};

    let request = request.map(|source| match source {
        EventSource::Action => StreamEventsRequest {
            request: Some(SReq::SubscribeToActions(SubscribeToActions {})),
        },
        EventSource::Homeserver => StreamEventsRequest {
            request: Some(SReq::SubscribeToHomeserverEvents(
                SubscribeToHomeserverEvents {},
            )),
        },
        EventSource::Guild(guild_id) => StreamEventsRequest {
            request: Some(SReq::SubscribeToGuild(SubscribeToGuild { guild_id })),
        },
    });

    log::debug!("Sending streaming request");
    let mut request = request.into_request();

    if let crate::client::AuthStatus::Complete(session) = client.auth_status() {
        // Session session_token should be ASCII, so this unwrap won't panic
        request
            .metadata_mut()
            .insert("auth", session.session_token.parse().unwrap());
    }

    let response = client.chat_lock().stream_events(request).await;

    log::debug!("Received response: {:?}", response);

    response.map(Response::into_inner).map_err(Into::into)
}
