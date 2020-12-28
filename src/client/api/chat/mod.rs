use super::*;

use crate::{api::chat::*, client_api};
use futures_util::StreamExt;
use http::Uri;
use std::fmt::{self, Display, Formatter};

// Export everything a client may need for this service
pub use crate::api::chat::{
    event, get_emote_pack_emotes_response::Emote, get_emote_packs_response::EmotePack,
    get_guild_channels_response::Channel, get_guild_invites_response::Invite,
    get_guild_list_response::GuildListEntry, permission::Mode, stream_events_request, Event,
    PermissionList, Role,
};

/// Describes a place in a list.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Place {
    Top { before: u64 },
    Between { after: u64, before: u64 },
    Bottom { after: u64 },
}

impl Place {
    /// Create a place between two other places.
    pub fn between(before: u64, after: u64) -> Self {
        Self::Between { after, before }
    }

    /// Create a place at the top of a list.
    pub fn top(before: u64) -> Self {
        Self::Top { before }
    }

    /// Create a place at the bottom of a list.
    pub fn bottom(after: u64) -> Self {
        Self::Bottom { after }
    }

    fn next(&self) -> u64 {
        match self {
            Place::Top { before } => *before,
            Place::Between { before, after: _ } => *before,
            Place::Bottom { after: _ } => 0,
        }
    }

    fn previous(&self) -> u64 {
        match self {
            Place::Top { before: _ } => 0,
            Place::Between { before: _, after } => *after,
            Place::Bottom { after } => *after,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InviteId(String);

impl InviteId {
    /// Creates an invite ID.
    ///
    /// `name` cannot be empty.
    /// If `name` is empty `None` is returned.
    pub fn new(name: String) -> Option<Self> {
        if name.is_empty() {
            None
        } else {
            Some(Self(name))
        }
    }

    fn into_name(self) -> String {
        self.0
    }
}

impl Display for InviteId {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
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
    /// Streams specified events (guild, homeserver or action) from the server.
    args: {
        requests: impl futures_util::stream::Stream<Item = stream_events_request::Request> + Send + Sync + 'static + std::fmt::Debug,
    },
    response: tonic::Streaming<Event>,
    request: requests.map(|r| StreamEventsRequest { request: Some(r) }),
    api_func: stream_events,
    service: chat,
}
