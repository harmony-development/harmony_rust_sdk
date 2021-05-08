pub use crate::api::chat::{
    GetUserBulkRequest, GetUserMetadataRequest, GetUserRequest, ProfileUpdateRequest,
};

use super::{harmonytypes::UserStatus, *};

client_api! {
    /// Get a user's profile.
    action: GetUser,
    api_fn: get_user,
    service: chat,
}

client_api! {
    /// Get given users' profile.
    action: GetUserBulk,
    api_fn: get_user_bulk,
    service: chat,
}

impl From<Vec<u64>> for GetUserBulkRequest {
    fn from(ids: Vec<u64>) -> Self {
        Self { user_ids: ids }
    }
}

impl From<Vec<UserId>> for GetUserBulkRequest {
    fn from(ids: Vec<UserId>) -> Self {
        Self {
            user_ids: ids.into_iter().map(Into::into).collect(),
        }
    }
}

/// Convenience type to create a valid [`GetUserMetadataRequest`].
#[into_request("GetUserMetadataRequest")]
#[derive(Debug, Clone, new)]
pub struct AppId {
    app_id: String,
}

client_api! {
    /// Get a user's metadata.
    action: GetUserMetadata,
    api_fn: get_user_metadata,
    service: chat,
}

/// Convenience type to create a valid [`ProfileUpdateRequest`].
#[derive(Debug, Clone, Default)]
pub struct ProfileUpdate {
    new_username: String,
    new_status: UserStatus,
    new_avatar: Option<Hmc>,
    is_bot: bool,
    update_username: bool,
    update_status: bool,
    update_avatar: bool,
    update_is_bot: bool,
}

impl ProfileUpdate {
    /// Set the new username of this user.
    pub fn new_username(mut self, username: impl std::fmt::Display) -> Self {
        self.new_username = username.to_string();
        self.update_username = true;
        self
    }

    /// Set the new status of this user.
    pub fn new_status(mut self, status: impl Into<UserStatus>) -> Self {
        self.new_status = status.into();
        self.update_status = true;
        self
    }

    /// Set the new avatar of this user.
    pub fn new_avatar(mut self, avatar: impl Into<Option<Hmc>>) -> Self {
        self.new_avatar = avatar.into();
        self.update_avatar = true;
        self
    }

    /// Set the new bot marker of this user.
    pub fn new_is_bot(mut self, is_bot: impl Into<bool>) -> Self {
        self.is_bot = is_bot.into();
        self.update_is_bot = true;
        self
    }
}

impl From<ProfileUpdate> for ProfileUpdateRequest {
    fn from(o: ProfileUpdate) -> Self {
        Self {
            new_username: o.new_username,
            new_status: o.new_status.into(),
            new_avatar: o.new_avatar.map_or_else(String::default, Into::into),
            is_bot: o.is_bot,
            update_username: o.update_username,
            update_status: o.update_status,
            update_avatar: o.update_avatar,
            update_is_bot: o.update_is_bot,
        }
    }
}

client_api! {
    /// Update local user's profile.
    request: ProfileUpdateRequest,
    api_fn: profile_update,
    service: chat,
}
