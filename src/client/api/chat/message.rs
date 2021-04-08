pub use crate::api::chat::{
    DeleteMessageRequest, GetMessageRequest, SendMessageRequest, UpdateMessageTextRequest,
};

use super::{
    harmonytypes::{
        content, Action, Attachment, Content, ContentEmbed, ContentFiles, ContentText, Embed,
        Message, Metadata, Override,
    },
    *,
};

/// Trait that implements convenience methods for [`Message`] type.
pub trait MessageExt {
    /// Get the text content of the message if it has one.
    fn text(&self) -> Option<&str>;
    /// Get the embed content of the message if it has one.
    fn embeds(&self) -> Option<&[Embed]>;
    /// Get the file content of the message if it has one.
    fn files(&self) -> Option<&[Attachment]>;
}

impl MessageExt for Message {
    fn text(&self) -> Option<&str> {
        match self.content.as_ref()?.content.as_ref()? {
            content::Content::TextMessage(text) => Some(&text.content),
            _ => None,
        }
    }

    fn embeds(&self) -> Option<&[Embed]> {
        match self.content.as_ref()?.content.as_ref()? {
            content::Content::EmbedMessage(embeds) => Some(&embeds.embeds),
            _ => None,
        }
    }

    fn files(&self) -> Option<&[Attachment]> {
        match self.content.as_ref()?.content.as_ref()? {
            content::Content::FilesMessage(files) => Some(&files.attachments),
            _ => None,
        }
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

/// Convenience type to create a valid [`SendMessageRequest`].
#[into_request("SendMessageRequest")]
#[derive(new, Debug, Clone, SelfBuilder)]
pub struct SendMessage {
    guild_id: u64,
    channel_id: u64,
    #[new(default)]
    content: Content,
    #[new(default)]
    echo_id: u64,
    #[new(default)]
    in_reply_to: u64,
    #[new(default)]
    #[builder(setter(strip_option))]
    overrides: Option<Override>,
    #[new(default)]
    #[builder(setter(strip_option))]
    metadata: Option<Metadata>,
}

impl SendMessage {
    pub fn actions(mut self, actions: impl Into<Vec<Action>>) -> Self {
        self.content.actions = actions.into();
        self
    }

    pub fn text(mut self, text: impl std::fmt::Display) -> Self {
        self.content.content = Some(content::Content::TextMessage(ContentText {
            content: text.to_string(),
        }));
        self
    }

    pub fn files(mut self, files: impl Into<Vec<Attachment>>) -> Self {
        self.content.content = Some(content::Content::FilesMessage(ContentFiles {
            attachments: files.into(),
        }));
        self
    }

    pub fn embeds(mut self, embeds: impl Into<Vec<Embed>>) -> Self {
        self.content.content = Some(content::Content::EmbedMessage(ContentEmbed {
            embeds: embeds.into(),
        }));
        self
    }
}

client_api! {
    /// Send a message.
    action: SendMessage,
    api_fn: send_message,
    service: chat,
}

client_api! {
    /// Update a message.
    request: UpdateMessageTextRequest,
    api_fn: update_message_text,
    service: chat,
}
