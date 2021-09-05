pub mod v1 {
    hrpc::include_proto!("protocol.profile.v1");
}
pub use v1::*;

use std::fmt::{self, Display, Formatter};

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
