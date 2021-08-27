pub use crate::api::chat::{
    GetUserBulkRequest, GetUserMetadataRequest, GetUserRequest, ProfileUpdateRequest,
};
use crate::client::api::rest::FileId;

use super::{harmonytypes::UserStatus, *};

/// Convenience type to create a valid [`GetUserMetadataRequest`].
#[into_request("GetUserMetadataRequest")]
#[derive(Debug, Clone, new)]
pub struct AppId {
    app_id: String,
}

/// Convenience type to create a valid [`ProfileUpdateRequest`].
#[derive(Debug, Clone, Default, new, builder)]
pub struct ProfileUpdate {
    #[builder(setter(strip_option))]
    #[new(default)]
    new_username: Option<String>,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_status: Option<UserStatus>,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_avatar: Option<Option<FileId>>,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_is_bot: Option<bool>,
}

impl From<ProfileUpdate> for ProfileUpdateRequest {
    fn from(o: ProfileUpdate) -> Self {
        Self {
            new_username: o.new_username,
            new_status: o.new_status.map(UserStatus::into),
            new_avatar: o
                .new_avatar
                .map(|a| a.map_or_else(String::new, FileId::into)),
            new_is_bot: o.new_is_bot,
        }
    }
}

impl_into_req!(ProfileUpdate);
