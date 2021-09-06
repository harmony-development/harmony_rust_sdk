pub use crate::api::profile::{
    GetAppDataRequest, GetProfileRequest, UpdateProfileRequest, UserStatus,
};

use crate::client::api::rest::FileId;

use super::*;

/// Convenience type to create a valid [`GetAppDataRequest`].
#[into_request("GetAppDataRequest")]
#[derive(Debug, Clone, new)]
pub struct AppId {
    app_id: String,
}

/// Convenience type to create a valid [`UpdateProfileRequest`].
#[derive(Debug, Clone, Default, new, builder)]
pub struct UpdateProfile {
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

impl From<UpdateProfile> for UpdateProfileRequest {
    fn from(o: UpdateProfile) -> Self {
        Self {
            new_user_name: o.new_username,
            new_user_status: o.new_status.map(UserStatus::into),
            new_user_avatar: o
                .new_avatar
                .map(|a| a.map_or_else(String::new, FileId::into)),
            new_is_bot: o.new_is_bot,
        }
    }
}

impl_into_req_from!(UpdateProfile);
