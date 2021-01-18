pub use crate::api::chat::{
    get_guild_channels_response, CreateChannelRequest, DeleteChannelRequest,
    GetChannelMessagesRequest, GetGuildChannelsRequest, UpdateChannelInformationRequest,
    UpdateChannelOrderRequest,
};

use super::{harmonytypes::Metadata, *};

client_api! {
    /// Get channels all channels (that you have permission to view) in a guild.
    action: GetGuildChannels,
    api_fn: get_guild_channels,
    service: chat,
}

/// Convenience type to create a valid [`GetChannelMessagesRequest`].
///
/// If `before_message_id` is not specified, it will default to `0`, which
/// means the server should return the latest messages.
///
/// Note that the number of messages returned may be limited by servers.
#[into_request("GetChannelMessagesRequest")]
#[derive(Debug, new, Clone, SelfBuilder)]
pub struct GetChannelMessages {
    guild_id: u64,
    channel_id: u64,
    #[new(default)]
    before_message: u64,
}

client_api! {
    /// Get messages before a message in a channel of a guild.
    action: GetChannelMessages,
    api_fn: get_channel_messages,
    service: chat,
}

/// Convenience type to create a valid [`CreateChannelRequest`].
#[derive(Debug, new, Clone, SelfBuilder)]
pub struct CreateChannel {
    guild_id: u64,
    channel_name: String,
    channel_place: Place,
    #[new(default)]
    #[builder(setter(strip_option))]
    metadata: Option<Metadata>,
    #[new(default)]
    is_category: bool,
}

impl IntoRequest<CreateChannelRequest> for CreateChannel {
    fn into_request(self) -> Request<CreateChannelRequest> {
        CreateChannelRequest {
            guild_id: self.guild_id,
            channel_name: self.channel_name,
            previous_id: self.channel_place.previous(),
            next_id: self.channel_place.next(),
            metadata: self.metadata,
            is_category: self.is_category,
        }
        .into_request()
    }
}

client_api! {
    /// Create a channel.
    action: CreateChannel,
    api_fn: create_channel,
    service: chat,
}

/// Convenience type to create a valid [`DeleteChannelRequest`].
#[into_request("DeleteChannelRequest")]
#[derive(Debug, Clone, new)]
pub struct DeleteChannel {
    guild_id: u64,
    channel_id: u64,
}

client_api! {
    /// Delete a channel.
    request: DeleteChannelRequest,
    api_fn: delete_channel,
    service: chat,
}

/// Convenience type to create a valid [`UpdateChannelInformationRequest`].
#[into_request("UpdateChannelInformationRequest")]
#[derive(Debug, Clone, new)]
pub struct UpdateChannelInformation {
    guild_id: u64,
    channel_id: u64,
    #[new(default)]
    name: String,
    #[new(default)]
    metadata: Option<Metadata>,
    #[new(default)]
    update_name: bool,
    #[new(default)]
    update_metadata: bool,
}

impl UpdateChannelInformation {
    /// Set new name for this channel.
    pub fn new_channel_name(mut self, channel_name: impl Into<String>) -> Self {
        self.name = channel_name.into();
        self.update_name = true;
        self
    }

    /// Set new metadata for this channel.
    pub fn new_metadata(mut self, metadata: impl Into<Option<Metadata>>) -> Self {
        self.metadata = metadata.into();
        self.update_metadata = true;
        self
    }
}

client_api! {
    /// Update a channel's information.
    request: UpdateChannelInformationRequest,
    api_fn: update_channel_information,
    service: chat,
}

/// Convenience type to create a valid [`UpdateChannelOrderRequest`].
#[derive(Debug, Clone, new)]
pub struct UpdateChannelOrder {
    guild_id: u64,
    channel_id: u64,
    new_channel_place: Place,
}

impl IntoRequest<UpdateChannelOrderRequest> for UpdateChannelOrder {
    fn into_request(self) -> Request<UpdateChannelOrderRequest> {
        UpdateChannelOrderRequest {
            guild_id: self.guild_id,
            channel_id: self.channel_id,
            previous_id: self.new_channel_place.previous(),
            next_id: self.new_channel_place.next(),
        }
        .into_request()
    }
}

client_api! {
    /// Update a channel's place in the channel list.
    request: UpdateChannelOrderRequest,
    api_fn: update_channel_order,
    service: chat,
}
