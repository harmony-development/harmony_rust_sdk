use harmony_derive::into_request;
use std::fmt::{self, Display, Formatter};

pub mod v1 {
    #![allow(clippy::unit_arg)]
    hrpc::include_proto!("protocol.chat.v1");
}
pub use v1::*;

/// Describes where to subscribe for events.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

/// Describes a place in a list.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Place {
    /// Top of the list.
    Top { before: u64 },
    /// Between two items in the list.
    Between { after: u64, before: u64 },
    /// Bottom of the list.
    Bottom { after: u64 },
}

impl Place {
    /// Create a place between two other places.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::chat::Place;
    /// let place = Place::between(2, 3);
    /// assert_eq!(place.after(), Some(3));
    /// assert_eq!(place.before(), Some(2));
    /// ```
    pub fn between(before: u64, after: u64) -> Self {
        Self::Between { after, before }
    }

    /// Create a place at the top of a list.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::chat::Place;
    /// let place = Place::top(1);
    /// assert_eq!(place.after(), None);
    /// assert_eq!(place.before(), Some(1));
    /// ```
    pub fn top(before: u64) -> Self {
        Self::Top { before }
    }

    /// Create a place at the bottom of a list.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::chat::Place;
    /// let place = Place::bottom(1);
    /// assert_eq!(place.after(), Some(1));
    /// assert_eq!(place.before(), None);
    /// ```
    pub fn bottom(after: u64) -> Self {
        Self::Bottom { after }
    }

    /// Get next place ID.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::chat::Place;
    /// let place = Place::bottom(1);
    /// assert_eq!(place.after(), Some(1));
    /// ```
    pub fn after(&self) -> Option<u64> {
        match self {
            Place::Top { before: _ } => None,
            Place::Between { before: _, after } => Some(*after),
            Place::Bottom { after } => Some(*after),
        }
    }

    /// Get previous place ID.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::chat::Place;
    /// let place = Place::top(1);
    /// assert_eq!(place.before(), Some(1));
    /// ```
    pub fn before(&self) -> Option<u64> {
        match self {
            Place::Top { before } => Some(*before),
            Place::Between { before, after: _ } => Some(*before),
            Place::Bottom { after: _ } => None,
        }
    }
}

/// An invite ID.
#[into_request("JoinGuildRequest", "PreviewGuildRequest")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InviteId {
    invite_id: String,
}

impl InviteId {
    /// Creates an invite ID.
    ///
    /// `name` cannot be empty.
    /// If `name` is empty `None` is returned.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::chat::InviteId;
    /// let invite_id = InviteId::new("harmony").unwrap();
    /// assert_eq!(invite_id.to_string(), "harmony".to_string());
    /// ```
    pub fn new(name: impl ToString) -> Option<Self> {
        let name = name.to_string();
        if name.is_empty() {
            None
        } else {
            Some(Self { invite_id: name })
        }
    }
}

impl From<InviteId> for String {
    fn from(other: InviteId) -> String {
        other.invite_id
    }
}

impl Display for InviteId {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.invite_id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_invite_id() {
        InviteId::new("").unwrap();
    }

    #[test]
    fn invite_id() {
        const ID: &str = "harmony";
        assert_eq!(
            InviteId::new(ID).unwrap(),
            InviteId {
                invite_id: ID.to_string()
            }
        );
    }
}
