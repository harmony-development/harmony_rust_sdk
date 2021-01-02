use super::*;

use crate::{api::chat::*, client_api};
use futures_util::StreamExt;
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
    Guild(u64),
    Homeserver,
    Action,
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

client_api! {
    /// Triggers the specified action.
    args: {
        guild_id: u64,
        channel_id: u64,
        message_id: u64,
        action_id: String,
        action_data: String,
    },
    request_type: TriggerActionRequest,
    api_func: trigger_action,
    service: chat,
}

client_api! {
    /// Notifies the server that the local user is typing.
    args: {
        guild_id: u64,
        channel_id: u64,
    },
    request_type: TypingRequest,
    api_func: typing,
    service: chat,
}

client_api! {
    /// Streams events from specified event sources from the server.
    args: {
        requests: impl futures_util::stream::Stream<Item = EventSource> + Send + Sync + 'static,
    },
    response: tonic::Streaming<Event>,
    request: requests.map(|r| {
        use stream_events_request::*;

        StreamEventsRequest {
            request: Some(match r {
                EventSource::Action => Request::SubscribeToActions(SubscribeToActions {}),
                EventSource::Homeserver => Request::SubscribeToHomeserverEvents(SubscribeToHomeserverEvents {}),
                EventSource::Guild(guild_id) => Request::SubscribeToGuild(SubscribeToGuild { guild_id }),
            })
        }
    }),
    api_func: stream_events,
    service: chat,
}
