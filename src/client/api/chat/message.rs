use super::*;

client_api! {
    /// Get a message.
    args: {
        guild_id: u64,
        channel_id: u64,
        message_id: u64,
    },
    action: GetMessage,
    api_func: get_message,
    service: chat,
}

client_api! {
    /// Delete a message.
    args: {
        guild_id: u64,
        channel_id: u64,
        message_id: u64,
    },
    request_type: DeleteMessageRequest,
    api_func: delete_message,
    service: chat,
}

client_api! {
    /// Send a message.
    args: {
        guild_id: u64,
        channel_id: u64,
        echo_id: Option<u64>,
        in_reply_to: Option<u64>,
        content: Option<String>,
        embeds: Option<Vec<Embed>>,
        actions: Option<Vec<Action>>,
        attachments: Option<Vec<Uri>>,
        overrides: Option<Option<Override>>,
        metadata: Option<Option<Metadata>>,
    },
    action: SendMessage,
    request_fields: {
        echo_id: echo_id.unwrap_or_default(),
        in_reply_to: in_reply_to.unwrap_or_default(),
        content: content.unwrap_or_default(),
        embeds: embeds.unwrap_or_default(),
        actions: actions.unwrap_or_default(),
        attachments: attachments.unwrap_or_default().into_iter().map(|u| u.to_string()).collect(),
        overrides: overrides.unwrap_or_default(),
        metadata: metadata.unwrap_or_default(),
        = guild_id, channel_id,
    },
    api_func: send_message,
    service: chat,
}

client_api! {
    /// Update a message.
    args: {
        guild_id: u64,
        channel_id: u64,
        message_id: u64,
        new_content: Option<String>,
        new_embeds: Option<Vec<Embed>>,
        new_actions: Option<Vec<Action>>,
        new_attachments: Option<Vec<Uri>>,
        new_overrides: Option<Option<Override>>,
        new_metadata: Option<Option<Metadata>>,
    },
    request: UpdateMessageRequest {
        update_content: new_content.is_some(),
        update_embeds: new_embeds.is_some(),
        update_actions: new_actions.is_some(),
        update_attachments: new_attachments.is_some(),
        update_overrides: new_overrides.is_some(),
        update_metadata: new_metadata.is_some(),
        content: new_content.unwrap_or_default(),
        embeds: new_embeds.unwrap_or_default(),
        actions: new_actions.unwrap_or_default(),
        attachments: new_attachments.unwrap_or_default().into_iter().map(|u| u.to_string()).collect(),
        overrides: new_overrides.unwrap_or_default(),
        metadata: new_metadata.unwrap_or_default(),
        guild_id, channel_id, message_id,
    },
    api_func: update_message,
    service: chat,
}
