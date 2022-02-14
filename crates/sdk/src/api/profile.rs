/// v1 of profile service.
pub mod v1 {
    #![allow(missing_docs)]
    hrpc::include_proto!("protocol.profile.v1");
}
pub use v1::*;

use std::fmt::{self, Display, Formatter};

impl From<UserStatus> for Option<i32> {
    fn from(status: UserStatus) -> Self {
        Some(status.into())
    }
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = match self {
            UserStatus::OfflineUnspecified => "Offline",
            UserStatus::Online => "Online",
            UserStatus::Idle => "Idle",
            UserStatus::DoNotDisturb => "Do Not Disturb",
            UserStatus::Streaming => "Streaming",
            UserStatus::Mobile => "Mobile",
        };
        write!(f, "{}", text)
    }
}

impl GetProfileRequest {
    /// Create a [`GetProfileRequest`] for fetching one user.
    #[inline(always)]
    pub fn new_one(user_id: u64) -> Self {
        Self::new(vec![user_id])
    }
}
