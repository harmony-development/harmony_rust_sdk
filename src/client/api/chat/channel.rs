use super::*;

client_api! {
    /// Get channels all channels (that you have permission to view) in a guild.
    args: { guild_id: u64, },
    action: GetGuildChannels,
    api_func: get_guild_channels,
    service: chat,
}

client_api! {
    /// Get messages before a message in a channel of a guild.
    ///
    /// If `before_message_id` is not specified, it will default to `0`, which
    /// means the server should return the latest messages.
    ///
    /// Note that the number of messages returned may be limited by servers.
    args: {
        guild_id: u64,
        channel_id: u64,
        before_message_id: Option<u64>,
    },
    action: GetChannelMessages,
    request_fields: {
        before_message: before_message_id.unwrap_or_default(),
        = guild_id, channel_id,
    },
    api_func: get_channel_messages,
    service: chat,
}

client_api! {
    /// Create a channel.
    args: {
        guild_id: u64,
        channel_name: String,
        is_category: bool,
        channel_place: Place,
        metadata: Option<Metadata>,
    },
    action: CreateChannel,
    request_fields: {
        previous_id: channel_place.previous(),
        next_id: channel_place.next(),
        = guild_id, is_category, channel_name, metadata,
    },
    api_func: create_channel,
    service: chat,
}

client_api! {
    /// Delete a channel.
    args: {
        guild_id: u64,
        channel_id: u64,
    },
    request_type: DeleteChannelRequest,
    api_func: delete_channel,
    service: chat,
}

client_api! {
    /// Update a channel's information.
    args: {
        new_channel_name: Option<String>,
        new_metadata: Option<Option<Metadata>>,
        guild_id: u64,
        channel_id: u64,
    },
    request: UpdateChannelInformationRequest {
        update_name: new_channel_name.is_some(),
        update_metadata: new_metadata.is_some(),
        name: new_channel_name.unwrap_or_default(),
        metadata: new_metadata.unwrap_or_default(),
        guild_id, channel_id,
    },
    api_func: update_channel_information,
    service: chat,
}

client_api! {
    /// Update a channel's place in the channel list.
    args: {
        guild_id: u64,
        channel_id: u64,
        new_channel_place: Place,
    },
    request: UpdateChannelOrderRequest {
        previous_id: new_channel_place.previous(),
        next_id: new_channel_place.next(),
        channel_id, guild_id,
    },
    api_func: update_channel_order,
    service: chat,
}
