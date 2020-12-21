use super::Unit;
use crate::{
    api::profile::*,
    client::{Client, ClientResult},
    client_api, client_api_action,
};
use tonic::Response;

// Export everything a client may need for this service
pub use crate::api::profile::UserStatus;

client_api_action! {
    api_func: get_user,
    service: profile,
    action: GetUser,

    args {
        user_id: u64 => user_id: (|u| u);
    }
}

client_api_action! {
    api_func: get_user_metadata,
    service: profile,
    action: GetUserMetadata,

    args {
        app_id: String => app_id: (|u| u);
    }
}

client_api! {
    api_func: username_update,
    service: profile,
    resp: Unit,
    req: UsernameUpdateRequest,

    args {
        new_username: String => user_name: (|u| u);
    }
}

client_api! {
    api_func: status_update,
    service: profile,
    resp: Unit,
    req: StatusUpdateRequest,

    args {
        new_status: UserStatus => new_status: (|u: UserStatus| u.into());
    }
}
