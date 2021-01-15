pub use crate::api::chat::{GetUserMetadataRequest, GetUserRequest, ProfileUpdateRequest};

use super::{harmonytypes::UserStatus, *};

client_api! {
    /// Get a user's profile.
    action: GetUser,
    api_fn: get_user,
    service: chat,
}

/// Convenience type to create a valid [`GetUserMetadataRequest`].
#[into_request("GetUserMetadataRequest")]
#[derive(Debug, new)]
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
#[into_request("ProfileUpdateRequest")]
#[derive(Debug, Default)]
pub struct ProfileUpdate {
    new_username: String,
    new_status: UserStatus,
    new_avatar: Hmc,
    is_bot: bool,
    update_username: bool,
    update_status: bool,
    update_avatar: bool,
    update_is_bot: bool,
}

impl ProfileUpdate {
    /// Set the new username of this user.
    pub fn new_username(mut self, username: impl Into<String>) -> Self {
        self.new_username = username.into();
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
    pub fn new_avatar(mut self, avatar: impl Into<Hmc>) -> Self {
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

client_api! {
    /// Update local user's profile.
    request: ProfileUpdateRequest,
    api_fn: profile_update,
    service: chat,
}
