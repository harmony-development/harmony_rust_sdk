pub use crate::api::chat::{
    BanUserRequest, CreateGuildRequest, DeleteGuildRequest, GetGuildListRequest,
    GetGuildMembersRequest, GetGuildRequest, JoinGuildRequest, KickUserRequest, LeaveGuildRequest,
    PreviewGuildRequest, UnbanUserRequest, UpdateGuildInformationRequest,
};
use crate::client::api::rest::FileId;

use super::{harmonytypes::Metadata, *};

/// Convenience type to create a valid [`CreateGuildRequest`].
#[impl_call_action(chat.v1)]
#[derive(Debug, Clone, new, self_builder)]
pub struct CreateGuild {
    guild_name: String,
    #[builder(setter(strip_option))]
    #[new(default)]
    picture_url: Option<FileId>,
    #[builder(setter(strip_option))]
    #[new(default)]
    metadata: Option<Metadata>,
}

impl From<CreateGuild> for CreateGuildRequest {
    fn from(o: CreateGuild) -> Self {
        Self {
            name: o.guild_name,
            picture: o.picture_url.map(Into::into),
            metadata: o.metadata,
        }
    }
}

impl_into_req_from!(CreateGuild);

/// Convenience type to create a valid [`UpdateGuildInformationRequest`].
#[impl_call_action(chat.v1)]
#[derive(Debug, Clone, new, self_builder)]
pub struct UpdateGuildInformation {
    guild_id: u64,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_guild_name: Option<String>,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_guild_picture: Option<Option<FileId>>,
    #[builder(setter(strip_option))]
    #[new(default)]
    metadata: Option<Option<Metadata>>,
}

impl From<UpdateGuildInformation> for UpdateGuildInformationRequest {
    fn from(o: UpdateGuildInformation) -> Self {
        Self {
            guild_id: o.guild_id,
            new_name: o.new_guild_name,
            new_picture: o
                .new_guild_picture
                .map(|p| p.map_or_else(String::default, Into::into)),
            new_metadata: o.metadata.map(|m| m.unwrap_or_default()),
        }
    }
}

impl_into_req_from!(UpdateGuildInformation);
