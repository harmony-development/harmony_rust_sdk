use harmony_derive::into_request;
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// v1 of chat service.
pub mod v1 {
    #![allow(clippy::unit_arg, missing_docs)]
    hrpc::include_proto!("protocol.chat.v1");

    /// All chat permissions.
    #[cfg(feature = "all_permissions")]
    pub mod all_permissions {
        #![allow(clippy::unit_arg)]
        hrpc::include_proto!("permissions");
    }
}
pub use v1::*;

impl From<String> for FormattedText {
    fn from(text: String) -> Self {
        FormattedText::new(text, Vec::new())
    }
}

impl GetGuildRequest {
    /// Create a new [`GetGuildRequest`] for fetching one guild.
    #[inline(always)]
    pub fn new_one(guild_id: u64) -> Self {
        Self::new(vec![guild_id])
    }
}

impl GetUserRolesRequest {
    /// Create a new [`GetUserRolesRequest`] for fetching one user's roles.
    #[inline(always)]
    pub fn new_one(guild_id: u64, user_id: u64) -> Self {
        Self::new(guild_id, vec![user_id])
    }
}

impl SendMessageRequest {
    /// Set the text content of this request.
    pub fn with_text(mut self, text: impl Into<FormattedText>) -> Self {
        let text = text.into();
        self.content = self.content.map(|mut c| {
            c.text = text.text;
            c.text_formats = text.format;
            c
        });
        self
    }

    /// Set the extra content of this request to attachments.
    pub fn with_attachments(
        mut self,
        attachments: impl Into<Vec<send_message_request::Attachment>>,
    ) -> Self {
        self.content = self.content.map(|mut c| {
            c.attachments = attachments.into();
            c
        });
        self
    }

    /// Set the extra content of this request to embeds.
    pub fn with_embeds(mut self, embeds: impl Into<Vec<Embed>>) -> Self {
        self.content = self.content.map(|mut c| {
            c.embeds = embeds.into();
            c
        });
        self
    }
}

impl Message {
    /// Shorthand to get the content of this message.
    #[inline(always)]
    pub fn get_content(&self) -> Option<&Content> {
        self.content.as_ref()
    }

    /// Shorthand to get the text content of this message.
    pub fn get_text(&self) -> Option<&str> {
        self.get_content().map(|c| c.text.as_str())
    }

    /// Shorthand to get the text content of this message.
    pub fn get_text_formats(&self) -> Option<&[Format]> {
        self.get_content().map(|c| c.text_formats.as_slice())
    }

    /// Shorthand to get the attachment content of this message.
    pub fn get_attachments(&self) -> Option<&[Attachment]> {
        self.get_content().map(|e| e.attachments.as_slice())
    }

    /// Shorthand to get the embed content of this message.
    pub fn get_embeds(&self) -> Option<&[Embed]> {
        self.get_content().map(|e| e.embeds.as_slice())
    }
}

/// A stream event.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv_validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Event {
    /// A chat service event.
    Chat(stream_event::Event),
    /// A profile service event.
    Profile(super::profile::stream_event::Event),
    /// An emote service event.
    Emote(super::emote::stream_event::Event),
}

/// Error returned if the [`StreamEventsResponse`] did not have valid fields.
///
/// For example, this could occur if one of the fields expected to be sent were
/// empty.
#[derive(Debug)]
pub struct EventFromResponseError;

impl Display for EventFromResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("event contains invalid fields")
    }
}

impl std::error::Error for EventFromResponseError {}

impl TryFrom<stream_events_response::Event> for Event {
    type Error = EventFromResponseError;

    fn try_from(value: stream_events_response::Event) -> Result<Self, Self::Error> {
        (match value {
            stream_events_response::Event::Chat(s) => s.event.map(Self::Chat),
            stream_events_response::Event::Emote(e) => e.event.map(Self::Emote),
            stream_events_response::Event::Profile(p) => p.event.map(Self::Profile),
        })
        .ok_or(EventFromResponseError)
    }
}

impl From<Event> for stream_events_response::Event {
    fn from(ev: Event) -> Self {
        match ev {
            Event::Chat(ev) => stream_events_response::Event::Chat(StreamEvent { event: Some(ev) }),
            Event::Emote(ev) => {
                stream_events_response::Event::Emote(super::emote::StreamEvent { event: Some(ev) })
            }
            Event::Profile(ev) => {
                stream_events_response::Event::Profile(super::profile::StreamEvent {
                    event: Some(ev),
                })
            }
        }
    }
}

/// Describes where to subscribe for events.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv_validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum EventSource {
    /// Subscription for a guild's events.
    Guild(u64),
    /// Subscription to homeserver events.
    Homeserver,
    /// Subscription to action events.
    Action,
    /// Subscription to disable auto subscription and remove all current
    /// subscriptions.
    Unsubscribe,
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
                EventSource::Unsubscribe => stream_events_request::Request::UnsubscribeFromAll(
                    stream_events_request::UnsubscribeFromAll {},
                ),
            }),
        }
    }
}

/// An invite ID.
#[into_request("JoinGuildRequest", "PreviewGuildRequest")]
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv_validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
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

/// Functions for working with color in Harmony API.
pub mod color {
    /// Encode an RGB value. Expects an array where values are `u8` and
    /// structured as `[red, green, blue]`.
    pub fn encode_rgb(color: impl Into<[u8; 3]>) -> i32 {
        let color = color.into();
        let mut c = color[0] as i32;
        c = (c << 8) + color[1] as i32;
        c = (c << 8) + color[2] as i32;
        c as i32
    }

    /// Decode an RGB value. Returns `[red, green, blue]` where values are `u8`.
    pub fn decode_rgb(color: impl Into<i32>) -> [u8; 3] {
        let color = color.into();
        [
            ((color >> 16) & 255) as u8,
            ((color >> 8) & 255) as u8,
            (color & 255) as u8,
        ]
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn encode() {
            assert_eq!(encode_rgb([0, 0, 0]), 0);
        }

        #[test]
        fn decode() {
            assert_eq!(decode_rgb(0), [0, 0, 0]);
        }

        #[test]
        fn encode_decode() {
            let value = [45, 12, 43];
            let encoded = encode_rgb(value);
            let decoded = decode_rgb(encoded);
            assert_eq!(decoded, value);
        }

        #[test]
        fn decode_encode() {
            let value = 727;
            let decoded = decode_rgb(value);
            let encoded = encode_rgb(decoded);
            assert_eq!(encoded, value);
        }
    }
}

/// Types and functions for working with permissions.
pub mod permission {
    /// Checks if a permission is allowed in some permission collection.
    ///
    /// Returns `None` if no permissions were matched.
    pub fn has_permission<'a, Perm, I>(perms: I, query: &str) -> Option<bool>
    where
        Perm: std::borrow::Borrow<(&'a str, bool)>,
        I: Iterator<Item = Perm>,
    {
        use std::cmp::Ordering;

        let mut matching_perms = perms
            .filter(|p| {
                let (matches, _) = p.borrow();
                matches
                    .split('.')
                    .zip(query.split('.'))
                    .all(|(m, c)| m == "*" || c == m)
            })
            .collect::<Vec<_>>();

        matching_perms.sort_unstable_by(|p, op| {
            let (m, _) = p.borrow();
            let (om, _) = op.borrow();
            let get_depth = |matches: &str| matches.chars().filter(|c| '.'.eq(c)).count();
            let ord = get_depth(m).cmp(&get_depth(om));

            if let Ordering::Equal = ord {
                let p_split = m.split('.');
                let op_split = om.split('.');
                match (p_split.last(), op_split.last()) {
                    (Some(p_last), Some(op_last)) => match (p_last, op_last) {
                        ("*", _) => Ordering::Less,
                        (_, "*") => Ordering::Greater,
                        _ => Ordering::Equal,
                    },
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            } else {
                ord
            }
        });

        matching_perms.pop().map(|p| p.borrow().1)
    }

    #[cfg(test)]
    mod tests {
        use super::has_permission;

        #[test]
        fn test_perm_compare_equal_allow() {
            let ok = has_permission([("messages.send", true)].iter(), "messages.send");
            assert_eq!(ok, Some(true));
        }

        #[test]
        fn test_perm_compare_equal_deny() {
            let ok = has_permission(
                std::array::IntoIter::new([("messages.send", false)]),
                "messages.send",
            );
            assert_eq!(ok, Some(false));
        }

        #[test]
        fn test_perm_compare_nonequal_allow() {
            let ok = has_permission([("messages.sendd", true)].iter(), "messages.send");
            assert_eq!(ok, None);
        }

        #[test]
        fn test_perm_compare_nonequal_deny() {
            let ok = has_permission([("messages.sendd", false)].iter(), "messages.send");
            assert_eq!(ok, None);
        }

        #[test]
        fn test_perm_compare_glob_allow() {
            let perms = [("messages.*", true)];
            let ok = has_permission(perms.iter(), "messages.send");
            assert_eq!(ok, Some(true));
            let ok = has_permission(perms.iter(), "messages.view");
            assert_eq!(ok, Some(true));
        }

        #[test]
        fn test_perm_compare_glob_deny() {
            let perms = [("messages.*", false)];
            let ok = has_permission(perms.iter(), "messages.send");
            assert_eq!(ok, Some(false));
            let ok = has_permission(perms.iter(), "messages.view");
            assert_eq!(ok, Some(false));
        }

        #[test]
        fn test_perm_compare_specific_deny() {
            let perms = [("messages.*", true), ("messages.send", false)];
            let ok = has_permission(perms.iter(), "messages.send");
            assert_eq!(ok, Some(false));
        }

        #[test]
        fn test_perm_compare_specific_allow() {
            let perms = [("messages.*", false), ("messages.send", true)];
            let ok = has_permission(perms.iter(), "messages.send");
            assert_eq!(ok, Some(true));
        }

        #[test]
        fn test_perm_compare_depth_allow() {
            let perms = [
                ("messages.*", false),
                ("messages.send", false),
                ("messages.send.send", true),
            ];
            let ok = has_permission(perms.iter(), "messages.send.send");
            assert_eq!(ok, Some(true));
        }

        #[test]
        fn test_perm_compare_depth_deny() {
            let perms = [
                ("messages.*", true),
                ("messages.send", true),
                ("messages.send.send", false),
            ];
            let ok = has_permission(perms.iter(), "messages.send.send");
            assert_eq!(ok, Some(false));
        }
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
