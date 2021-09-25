pub use crate::api::chat::{
    content::{AttachmentContent, EmbedContent, PhotoContent, TextContent},
    format as text_format, DeleteMessageRequest, Format as TextFormat, FormattedText,
    GetMessageRequest, SendMessageRequest, UpdateMessageTextRequest,
};

use super::{harmonytypes::Metadata, *};

/// Trait that implements convenience methods for [`Message`] type.
pub trait MessageExt {
    /// Get the text content of the message if it has one.
    fn text(&self) -> Option<&str>;
    /// Get the embed content of the message if it has one.
    fn embeds(&self) -> Option<&Embed>;
    /// Get the file content of the message if it has one.
    fn files(&self) -> Option<&[Attachment]>;
}

impl MessageExt for Message {
    fn text(&self) -> Option<&str> {
        match self.content.as_ref()?.content.as_ref()? {
            content::Content::TextMessage(text) => text.content.as_ref().map(|f| f.text.as_str()),
            _ => None,
        }
    }

    fn embeds(&self) -> Option<&Embed> {
        match self.content.as_ref()?.content.as_ref()? {
            content::Content::EmbedMessage(embeds) => embeds.embed.as_deref(),
            _ => None,
        }
    }

    fn files(&self) -> Option<&[Attachment]> {
        match self.content.as_ref()?.content.as_ref()? {
            content::Content::AttachmentMessage(files) => Some(&files.files),
            _ => None,
        }
    }
}

/// Convenience type to create a valid [`SendMessageRequest`].
#[impl_call_action(chat)]
#[into_request("SendMessageRequest")]
#[derive(new, Debug, Clone, self_builder)]
pub struct SendMessage {
    guild_id: u64,
    channel_id: u64,
    #[new(default)]
    content: Content,
    #[new(default)]
    #[builder(setter(strip_option))]
    echo_id: Option<u64>,
    #[new(default)]
    in_reply_to: Option<u64>,
    #[new(default)]
    #[builder(setter(strip_option))]
    overrides: Option<Overrides>,
    #[new(default)]
    #[builder(setter(strip_option))]
    metadata: Option<Metadata>,
}

impl SendMessage {
    pub fn text(mut self, text: impl std::fmt::Display) -> Self {
        self.content.content = Some(content::Content::TextMessage(TextContent {
            content: Some(FormattedText {
                text: text.to_string(),
                format: Vec::new(),
            }),
        }));
        self
    }

    pub fn files(mut self, files: impl Into<Vec<Attachment>>) -> Self {
        self.content.content = Some(content::Content::AttachmentMessage(AttachmentContent {
            files: files.into(),
        }));
        self
    }

    pub fn embed(mut self, embed: impl Into<Embed>) -> Self {
        self.content.content = Some(content::Content::EmbedMessage(EmbedContent {
            embed: Some(Box::new(embed.into())),
        }));
        self
    }
}
