/// Harmony protocol code generated with [`tonic-build`](https://lib.rs/crates/tonic-build).
pub mod api {
    pub mod core {
        tonic::include_proto!("protocol.core.v1");
    }

    pub mod foundation {
        tonic::include_proto!("protocol.foundation.v1");
    }

    pub mod profile {
        tonic::include_proto!("protocol.profile.v1");
    }
}

#[cfg(feature = "client")]
pub mod client;
