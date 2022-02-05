#![deny(missing_docs, unsafe_code)]
/*!
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yusdacra/harmony_rust_sdk/Rust)
[![crates.io](https://img.shields.io/crates/v/harmony_rust_sdk)](https://crates.io/crates/harmony_rust_sdk)
[![docs.rs](https://docs.rs/harmony_rust_sdk/badge.svg)](https://docs.rs/harmony_rust_sdk)
[![docs.rs](https://img.shields.io/badge/docs-master-blue)](https://harmonyapp.io/harmony_rust_sdk)
![MSRV](https://img.shields.io/badge/MSRV-current%20stable-red)

Rust implementation of [the Harmony chat protocol](https://github.com/harmony-development).
Currently implements a lightweight client and a client API (powered by [hrpc](https://crates.io/crates/hrpc)),
along with auto generated API via [hrpc-build](https://crates.io/crates/hrpc-build).

## Requirements

- Latest stable Rust and Cargo.
- If you are using Nix, `nix-shell` (or `nix develop` if you use flakes) should
get you covered.
- Otherwise, you'll need to get protobuf and make sure `protoc` is in your `PATH`
env variable.
  - If for some reason `build.rs` fails, make sure to set:
    - `PROTOC` env variable to your `protoc` executable
    - and `PROTOC_INCLUDE` env variable to wherever protobuf include files are
    located, most likely in `/usr/share/include`.

## Examples

- `message_log`: Showcases a simple message log bot that operates in a guild.
It will log messages to the console whenever someone posts a message.
- Bot run instructions:
  - Run bots with `GUILD_INVITE=invite cargo run --example example_name`.
  - Make sure the bot has necessary permissions to view channels / send messages etc.

## Crate features

- By default, only a bare-bones common API types used in Harmony is generated.
You can customize the crate to your needs by enabling feature(s) listed below:
  - Enable `gen_all_protocols` to enable all protocols, stable and staging.
  - Enable `rkyv` feature to derive `rkyv::{Archive, Deserialize, Serialize}`
  for all Harmony API types (except the `batch` service).
    - Enable `rkyv_validation` to derive `bytecheck::CheckBytes` for all Harmony
    API types and enable `rkyv/validation`.
  - Enable `serde_derive` feature to derive `serde::{Deserialize, Serialize}`
  for all Harmony API types (except the `batch` service).
  - Enable `valuable` feature to derive `valuable::Valuable` for all Harmony
  API types (except the `batch` service).
  - customizing hRPC codegen:
    - Enable the `gen_client` feature to generate client service code for
    enabled protocols.
    - Enable the `gen_server` feature to generate server service code for
    enabled protocols.
  - Client:
    - Enable the `client_native` feature for a lightweight client implementation
    that uses `hyper` and works on native platforms.
    - Enable the `client_web` feature for a lightweight client implementation that
    works on web platforms (WASM).
    - Enable the `client_backoff` feature to enable request retrying on ratelimited
    requests.
  - Stable protocols (enable `gen_stable_protocols` for all):
    - Enable the `gen_chat` feature to generate chat service code.
    - Enable the `gen_auth` feature to generate auth service code.
    - Enable the `gen_mediaproxy` feature to generate media proxy service code.
    - Enable the `gen_harmonytypes` feature to generate common Harmony types.
    - Enable the `gen_sync` feature to generate sync service code.
    - Enable the `gen_emote` feature to generate emote service code.
    - Enable the `gen_profile` feature to generate profile service code.
    - Enable the `gen_batch` feature to generate batch service code.
    - Enable the `rest` feature to include REST API code.
  - Staging protocols (enable `gen_staging_protocols` for all):
    - Enable the `staging_gen_voice` feature to generate voice service code.
    - Enable the `staging_gen_bots` feature to generate bots service code.

## MSRV

Minimum Supported Rust Version: current stable.

Changing MSRV is not considered a semver-breaking change.

!*/

/// Harmony protocol API.
pub mod api;

#[cfg(feature = "_client_common")]
pub mod client;
