use crate::{
    api::profile::*,
    client::{Client, ClientResult},
    client_api,
};
use tonic::{Request, Response};

// Export everything a client may need for this service
pub use crate::api::profile::UserStatus;

client_api! {
    args: { user_id: u64, },
    action: GetUser,
    api_func: get_user,
    service: profile,
}

client_api! {
    args: { app_id: String, },
    action: GetUserMetadata,
    api_func: get_user_metadata,
    service: profile,
}

client_api! {
    args: { new_username: String, },
    request: UsernameUpdateRequest { user_name: new_username },
    api_func: username_update,
    service: profile,
}

client_api! {
    args: { new_status: UserStatus, },
    request: StatusUpdateRequest { new_status: new_status.into(), },
    api_func: status_update,
    service: profile,
}
