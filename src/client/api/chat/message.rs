use crate::api::Hmcs;

use super::*;

/// Convenience type to create a valid [`SendMessageRequest`].
#[into_request("SendMessageRequest")]
#[derive(new, Debug, SelfBuilder)]
pub struct SendMessage {
    guild_id: u64,
    channel_id: u64,
    content: String,
    #[new(default)]
    echo_id: u64,
    #[new(default)]
    in_reply_to: u64,
    #[new(default)]
    embeds: Vec<Embed>,
    #[new(default)]
    actions: Vec<Action>,
    #[new(default)]
    attachments: Hmcs,
    #[new(default)]
    #[builder(setter(strip_option))]
    overrides: Option<Override>,
    #[new(default)]
    #[builder(setter(strip_option))]
    metadata: Option<Metadata>,
}

/// Convenience type to create a valid [`UpdateMessageRequest`].
#[into_request("UpdateMessageRequest")]
#[derive(new, Debug)]
pub struct UpdateMessage {
    guild_id: u64,
    channel_id: u64,
    message_id: u64,
    #[new(default)]
    content: String,
    #[new(default)]
    embeds: Vec<Embed>,
    #[new(default)]
    actions: Vec<Action>,
    #[new(default)]
    attachments: Hmcs,
    #[new(default)]
    overrides: Option<Override>,
    #[new(default)]
    metadata: Option<Metadata>,
    #[new(default)]
    update_content: bool,
    #[new(default)]
    update_embeds: bool,
    #[new(default)]
    update_actions: bool,
    #[new(default)]
    update_attachments: bool,
    #[new(default)]
    update_overrides: bool,
    #[new(default)]
    update_metadata: bool,
}

impl UpdateMessage {
    /// Set the new content of this message.
    pub fn new_content(mut self, content: String) -> Self {
        self.content = content;
        self.update_content = true;
        self
    }

    /// Set the new embeds of this message.
    pub fn new_embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = embeds;
        self.update_embeds = true;
        self
    }

    /// Set the new actions of this message.
    pub fn new_actions(mut self, actions: Vec<Action>) -> Self {
        self.actions = actions;
        self.update_actions = true;
        self
    }

    /// Set the new attachments of this message.
    pub fn new_attachments(mut self, attachments: Hmcs) -> Self {
        self.attachments = attachments;
        self.update_attachments = true;
        self
    }

    /// Set the new overrides of this message.
    pub fn new_overrides(mut self, overrides: Option<Override>) -> Self {
        self.overrides = overrides;
        self.update_overrides = true;
        self
    }

    /// Set the new metadata of this message.
    pub fn new_metadata(mut self, metadata: Option<Metadata>) -> Self {
        self.metadata = metadata;
        self.update_metadata = true;
        self
    }
}

client_api! {
    /// Get a message.
    action: GetMessage,
    api_fn: get_message,
    service: chat,
}

client_api! {
    /// Delete a message.
    request: DeleteMessageRequest,
    api_fn: delete_message,
    service: chat,
}

client_api! {
    /// Send a message.
    action: SendMessage,
    api_fn: send_message,
    service: chat,
}

client_api! {
    /// Update a message.
    request: UpdateMessageRequest,
    api_fn: update_message,
    service: chat,
}
