pub mod core {
    include!(concat!(env!("OUT_DIR"), "/protocol.core.v1.rs"));
}

pub mod foundation {
    include!(concat!(env!("OUT_DIR"), "/protocol.foundation.v1.rs"));
}

pub mod profile {
    include!(concat!(env!("OUT_DIR"), "/protocol.profile.v1.rs"));
}

pub mod exports {
    pub use prost;
    pub use prost_types;
}
