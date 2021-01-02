//! Rust library to work with the Harmony chat protocol.
//!
//! This crate currently contains the generated API code and client API with a lightweight client implementation.
//!
//! ## Examples
//! - `echo_bot`: Showcases a simple message echo bot that operates in a guild. It will repost messages whenever someone else posts a message.
//! - `message_log`: Showcases a simple message log bot that operates in a guild. It will log messages to the console whenever someone posts a message.
//!
//! - Bot run instructions:
//!     - Make sure legato is running on `http://127.0.0.1:2289` or whatever you set `HOMESERVER` constant to.
//!     - Run bots with `cargo run --example example_name`. First run will register to the homeserver.
//!     - Login as bot with a client, and join your guild (eg. by creating an invite and using it with bot's account).
//!     - Make sure the bot has necessary permissions to view channels / send messages etc.
//!     - Save your guild ID to `guild_id`.
//!     - Run the bot again and it should now work! (hopefully™)
//!
//! ## Crate features
//! - By default, no features are enabled and only a bare-bones common API is generated. You can customize the crate to your needs by enabling feature(s) listed below:
//!     - Enable the `use_parking_lot` feature if you want to use [parking_lot](https://github.com/Amanieu/parking_lot) `sync` types instead of `std::sync`.
//!     - Enable the `client` feature for a lightweight client implementation and the client API (implies `gen_client` feature).
//!     - Enable the `gen_client` feature to generate client service code.
//!    - Enable the `gen_server` feature to generate server service code.

/// Harmony protocol code generated with [`tonic-build`](https://crates.io/crates/tonic-build).
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

    pub mod mediaproxy {
        pub mod v1 {
            tonic::include_proto!("protocol.mediaproxy.v1");
        }
        pub use v1::*;
    }
}

#[cfg(feature = "client")]
pub mod client;
