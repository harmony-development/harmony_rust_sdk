use std::fmt::{self, Display, Formatter};

pub mod v1 {
    hrpc::include_proto!("protocol.harmonytypes.v1");
}
pub use v1::*;

impl Display for UserStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = match self {
            UserStatus::Offline => "Offline",
            UserStatus::OnlineUnspecified => "Online",
            UserStatus::Idle => "Idle",
            UserStatus::DoNotDisturb => "Do Not Disturb",
            UserStatus::Streaming => "Streaming",
        };
        write!(f, "{}", text)
    }
}
