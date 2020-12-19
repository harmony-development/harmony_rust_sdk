/// The generated Harmony protocol code.
pub mod harmony {
    pub mod core {
        include!(concat!(env!("OUT_DIR"), "/protocol.core.v1.rs"));
    }
    pub mod foundation {
        include!(concat!(env!("OUT_DIR"), "/protocol.foundation.v1.rs"));
    }
    pub mod profile {
        include!(concat!(env!("OUT_DIR"), "/protocol.profile.v1.rs"));
    }
}
