pub use crate::api::chat::{
    get_channel_messages_request::Direction, CreateChannelRequest, DeleteChannelRequest,
    GetChannelMessagesRequest, GetGuildChannelsRequest, UpdateChannelInformationRequest,
    UpdateChannelOrderRequest,
};

use super::{harmonytypes::Metadata, *};

/// Convenience type to create a valid [`GetChannelMessagesRequest`].
///
/// If `before_message_id` is not specified, it will default to `0`, which
/// means the server should return the latest messages.
///
/// Note that the number of messages returned may be limited by servers.
#[impl_call_action(chat)]
#[derive(Debug, new, Clone, self_builder)]
pub struct GetChannelMessages {
    guild_id: u64,
    channel_id: u64,
    #[new(default)]
    message_id: u64,
    #[new(default)]
    #[builder(setter(strip_option))]
    direction: Option<Direction>,
    #[new(default)]
    #[builder(setter(strip_option))]
    count: Option<u32>,
}

impl From<GetChannelMessages> for GetChannelMessagesRequest {
    fn from(o: GetChannelMessages) -> Self {
        Self {
            guild_id: o.guild_id,
            channel_id: o.channel_id,
            message_id: o.message_id,
            direction: o.direction.map(Into::into),
            count: o.count,
        }
    }
}

impl_into_req_from!(GetChannelMessages);

/// Convenience type to create a valid [`CreateChannelRequest`].
#[impl_call_action(chat)]
#[into_request("CreateChannelRequest")]
#[derive(Debug, new, Clone, self_builder)]
pub struct CreateChannel {
    guild_id: u64,
    channel_name: String,
    position: Place,
    #[new(default)]
    #[builder(setter(strip_option))]
    metadata: Option<Metadata>,
    #[new(default)]
    is_category: bool,
}

/// Convenience type to create a valid [`DeleteChannelRequest`].
#[impl_call_action(chat)]
#[into_request("DeleteChannelRequest")]
#[derive(Debug, Clone, new)]
pub struct DeleteChannel {
    guild_id: u64,
    channel_id: u64,
}

/// Convenience type to create a valid [`UpdateChannelInformationRequest`].
#[impl_call_action(chat)]
#[derive(Debug, Clone, new, self_builder)]
pub struct UpdateChannelInformation {
    guild_id: u64,
    channel_id: u64,
    #[new(default)]
    #[builder(setter(strip_option))]
    new_name: Option<String>,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_metadata: Option<Option<Metadata>>,
}

impl From<UpdateChannelInformation> for UpdateChannelInformationRequest {
    fn from(u: UpdateChannelInformation) -> Self {
        Self {
            guild_id: u.guild_id,
            channel_id: u.channel_id,
            new_name: u.new_name,
            new_metadata: u.new_metadata.map(Option::unwrap_or_default),
        }
    }
}

impl_into_req_from!(UpdateChannelInformation);

/// Convenience type to create a valid [`UpdateChannelOrderRequest`].
#[impl_call_action(chat)]
#[into_request("UpdateChannelOrderRequest")]
#[derive(Debug, Clone, new)]
pub struct UpdateChannelOrder {
    guild_id: u64,
    channel_id: u64,
    new_position: Place,
}
