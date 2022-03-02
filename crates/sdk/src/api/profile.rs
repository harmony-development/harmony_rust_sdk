/// v1 of profile service.
pub mod v1 {
    #![allow(missing_docs)]
    hrpc::include_proto!("protocol.profile.v1");
}
pub use v1::*;

use std::fmt::{self, Display, Formatter};

impl Display for user_status::Kind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = match self {
            Self::OfflineUnspecified => "Offline",
            Self::Online => "Online",
            Self::Idle => "Idle",
            Self::DoNotDisturb => "Do Not Disturb",
        };
        f.write_str(text)
    }
}

impl UpdateStatusRequest {
    /// Create a [`UpdateStatusRequest`] for changing user status kind.
    pub fn update_kind(status: user_status::Kind) -> Self {
        Self {
            new_status: Some(UserStatus {
                kind: status.into(),
                ..Default::default()
            }),
        }
    }
}

impl GetProfileRequest {
    /// Create a [`GetProfileRequest`] for fetching one user.
    #[inline(always)]
    pub fn new_one(user_id: u64) -> Self {
        Self::new(vec![user_id])
    }
}
