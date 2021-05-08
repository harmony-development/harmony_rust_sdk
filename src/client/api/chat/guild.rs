pub use crate::api::chat::{
    get_guild_list_response, AddGuildToGuildListRequest, BanUserRequest, CreateGuildRequest,
    DeleteGuildRequest, GetGuildListRequest, GetGuildMembersRequest, GetGuildRequest,
    JoinGuildRequest, KickUserRequest, LeaveGuildRequest, PreviewGuildRequest,
    RemoveGuildFromGuildListRequest, UnbanUserRequest, UpdateGuildInformationRequest,
};

use super::{harmonytypes::Metadata, *};

client_api! {
    /// Get guild list for local user.
    action: GetGuildList,
    api_fn: get_guild_list,
    service: chat,
}

client_api! {
    /// Get guild data of a guild.
    action: GetGuild,
    api_fn: get_guild,
    service: chat,
}

client_api! {
    /// Get a list of all users in a guild.
    action: GetGuildMembers,
    api_fn: get_guild_members,
    service: chat,
}

/// Convenience type to create a valid [`CreateGuildRequest`].
#[derive(Debug, Clone, new)]
pub struct CreateGuild {
    guild_name: String,
    #[new(default)]
    picture_url: Option<Hmc>,
    #[new(default)]
    metadata: Option<Metadata>,
}

impl CreateGuild {
    /// Set the picture HMC for this new guild.
    pub fn picture(mut self, picture: impl Into<Hmc>) -> Self {
        self.picture_url = Some(picture.into());
        self
    }

    /// Set the metadata for this new guild.
    pub fn metadata(mut self, metadata: impl Into<Metadata>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
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

client_api! {
    /// Create a new guild.
    action: CreateGuild,
    api_fn: create_guild,
    service: chat,
}

/// Convenience type to create a valid [`AddGuildToGuildListRequest`] and [`RemoveGuildFromGuildListRequest`].
#[derive(Debug, Clone, new)]
pub struct GuildList {
    guild_id: u64,
    homeserver: Url,
}

impl From<GuildList> for AddGuildToGuildListRequest {
    fn from(o: GuildList) -> Self {
        Self {
            guild_id: o.guild_id,
            homeserver: o.homeserver.to_string(),
        }
    }
}

client_api! {
    /// Add a guild to the guild list.
    action: AddGuildToGuildList,
    api_fn: add_guild_to_guild_list,
    service: chat,
}

impl From<GuildList> for RemoveGuildFromGuildListRequest {
    fn from(o: GuildList) -> Self {
        Self {
            guild_id: o.guild_id,
            homeserver: o.homeserver.to_string(),
        }
    }
}

client_api! {
    /// Remove a guild from the guild list.
    action: RemoveGuildFromGuildList,
    api_fn: remove_guild_from_guild_list,
    service: chat,
}

/// Convenience type to create a valid [`UpdateGuildInformationRequest`].
#[derive(Debug, Clone, new)]
pub struct UpdateGuildInformation {
    guild_id: u64,
    #[new(default)]
    new_guild_name: String,
    #[new(default)]
    new_guild_picture: Option<Hmc>,
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
    pub fn new_guild_picture(mut self, guild_picture: impl Into<Option<Hmc>>) -> Self {
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

client_api! {
    /// Update a guild's information.
    request: UpdateGuildInformationRequest,
    api_fn: update_guild_information,
    service: chat,
}

client_api! {
    /// Delete a guild.
    request: DeleteGuildRequest,
    api_fn: delete_guild,
    service: chat,
}

client_api! {
    /// Join a guild, using the specified invite id.
    action: JoinGuild,
    api_fn: join_guild,
    service: chat,
}

client_api! {
    /// Leave a guild.
    request: LeaveGuildRequest,
    api_fn: leave_guild,
    service: chat,
}

client_api! {
    /// Preview a guild.
    action: PreviewGuild,
    api_fn: preview_guild,
    service: chat,
}

client_api! {
    /// Ban a user.
    request: BanUserRequest,
    api_fn: ban_user,
    service: chat,
}

client_api! {
    /// Kick a user.
    request: KickUserRequest,
    api_fn: kick_user,
    service: chat,
}

client_api! {
    /// Unban a user.
    request: UnbanUserRequest,
    api_fn: unban_user,
    service: chat,
}
