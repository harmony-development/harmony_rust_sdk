/*!
Rust implementation of [the Harmony chat protocol](https://github.com/harmony-development).
Currently implements a lightweight client and a client API (powered by [tonic](https://crates.io/crates/tonic)), along with auto generated API via [tonic-build](https://crates.io/crates/tonic-build).

## Requirements
- Latest stable Rust and Cargo.
- If you are using Nix, `nix-shell` (or `nix develop` if you use flakes) should get you covered.
- Otherwise, you'll need to get protobuf and make sure `protoc` is in your `PATH` env variable.
  - If for some reason `build.rs` fails, make sure to set:
    - `PROTOC` env variable to your `protoc` executable
    - and `PROTOC_INCLUDE` env variable to wherever protobuf include files are located, most likely in `/usr/share/include`.

## Examples
- `echo_bot`: Showcases a simple message echo bot that operates in a guild. It will repost messages whenever someone else posts a message.
- `message_log`: Showcases a simple message log bot that operates in a guild. It will log messages to the console whenever someone posts a message.

- Bot run instructions:
  - Run bots with `GUILD_INVITE=invite cargo run --example example_name`.
  - Make sure the bot has necessary permissions to view channels / send messages etc.

## Crate features
- By default, only a bare-bones common API of all services is generated. You can customize the crate to your needs by enabling feature(s) listed below:
  - Enable the `parking_lot` feature if you want to use [parking_lot](https://github.com/Amanieu/parking_lot) `sync` types instead of `std::sync`.
  - Enable the `client` feature for a lightweight client implementation and the client API.
    - Enable the `request_method` feature to enable a request method in `Client` which allows you to make requests like this:
      ```rust,no_run,ignore
        // Change our bots status to online and make sure its marked as a bot
        client
            .request::<ProfileUpdateRequest, _, _>(
                ProfileUpdate::default()
                    .new_status(UserStatus::OnlineUnspecified)
                    .new_is_bot(true),
            )
            .await?;
      ```
  - Enable the `gen_client` feature to generate client service code.
  - Enable the `gen_server` feature to generate server service code.
  - (Default) Enable the `gen_chat` feature to generate chat service code.
  - (Default) Enable the `gen_auth` feature to generate auth service code.
  - (Default) Enable the `gen_voice` feature to generate voice service code.
  - (Default) Enable the `gen_mediaproxy` feature to generate media proxy service code.
  - (Default) Enable the `gen_harmonytypes` feature to generate common Harmony types.

## MSRV
Minimum Supported Rust Version: previous stable.

Changing MSRV is not considered a semver-breaking change.
!*/
#[macro_use]
extern crate derive_new;
#[cfg(feature = "gen_chat")]
#[macro_use]
extern crate harmony_derive;

/// Harmony protocol API.
pub mod api;

#[cfg(feature = "client")]
pub mod client;
