pub use crate::api::chat::{
    get_guild_list_response, BanUserRequest, CreateGuildRequest, DeleteGuildRequest,
    GetGuildListRequest, GetGuildMembersRequest, GetGuildRequest, JoinGuildRequest,
    KickUserRequest, LeaveGuildRequest, PreviewGuildRequest, UnbanUserRequest,
    UpdateGuildInformationRequest,
};
use crate::client::api::rest::FileId;

use super::{harmonytypes::Metadata, *};

/// Convenience type to create a valid [`CreateGuildRequest`].
#[derive(Debug, Clone, new, builder)]
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
            guild_name: o.guild_name,
            picture_url: o.picture_url.map_or_else(String::default, Into::into),
            metadata: o.metadata,
        }
    }
}

impl_into_req!(CreateGuild);

/// Convenience type to create a valid [`UpdateGuildInformationRequest`].
#[derive(Debug, Clone, new)]
pub struct UpdateGuildInformation {
    guild_id: u64,
    #[new(default)]
    new_guild_name: String,
    #[new(default)]
    new_guild_picture: Option<FileId>,
    #[new(default)]
    metadata: Option<Metadata>,
    #[new(default)]
    update_guild_name: bool,
    #[new(default)]
    update_guild_picture: bool,
    #[new(default)]
    update_metadata: bool,
}

impl UpdateGuildInformation {
    /// Set the new name of this guild.
    pub fn new_guild_name(mut self, guild_name: impl std::fmt::Display) -> Self {
        self.new_guild_name = guild_name.to_string();
        self.update_guild_name = true;
        self
    }

    /// Set the new picture of this guild.
    pub fn new_guild_picture(mut self, guild_picture: impl Into<Option<FileId>>) -> Self {
        self.new_guild_picture = guild_picture.into();
        self.update_guild_picture = true;
        self
    }

    /// Set the new metadata of this guild.
    pub fn new_metadata(mut self, metadata: impl Into<Option<Metadata>>) -> Self {
        self.metadata = metadata.into();
        self.update_metadata = true;
        self
    }
}

impl From<UpdateGuildInformation> for UpdateGuildInformationRequest {
    fn from(o: UpdateGuildInformation) -> Self {
        Self {
            guild_id: o.guild_id,
            new_guild_name: o.new_guild_name,
            new_guild_picture: o.new_guild_picture.map_or_else(String::default, Into::into),
            metadata: o.metadata,
            update_guild_name: o.update_guild_name,
            update_guild_picture: o.update_guild_picture,
            update_metadata: o.update_metadata,
        }
    }
}

impl_into_req!(UpdateGuildInformation);
