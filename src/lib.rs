//! Rust library to work with the Harmony chat protocol.
//!
//! This crate currently contains the generated API code and client API with a lightweight client implementation.

/// Harmony protocol code generated with [`tonic-build`](https://lib.rs/crates/tonic-build).
pub mod api {
    pub mod chat {
        pub mod v1 {
            tonic::include_proto!("protocol.chat.v1");
        }
        pub use v1::*;
    }

    pub mod auth {
        pub mod v1 {
            tonic::include_proto!("protocol.auth.v1");
        }
        pub use v1::*;
    }

    pub mod harmonytypes {
        pub mod v1 {
            tonic::include_proto!("protocol.harmonytypes.v1");
        }
        pub use v1::*;
    }
}

#[cfg(feature = "client")]
pub mod client;
