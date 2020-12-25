![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yusdacra/harmony_rust_sdk/Rust)
[![crates.io](https://img.shields.io/crates/v/harmony_rust_sdk)](https://crates.io/crates/harmony_rust_sdk)
[![docs.rs](https://docs.rs/harmony_rust_sdk/badge.svg)](https://docs.rs/harmony_rust_sdk)

Rust implementation of [the Harmony chat protocol](https://github.com/harmony-development).
Currently implements a lightweight client and a client API (powered by `tonic`), along with auto generated API via `tonic-build`.

## Requirements
- Latest stable Rust and Cargo.
- If you are using Nix, `nix-shell` (or `nix develop` if you use flakes) should get you covered.
- Otherwise, you'll need to get protobuf and make sure `protoc` is in your `PATH` env variable.
  - If for some reason `build.rs` fails, make sure to set: 
    - `PROTOC` env variable to your `protoc` executable
    - and `PROTOC_INCLUDE` env variable to wherever protobuf include files are located, most likely in `/usr/share/include`.
- For tests to work, you'll need to run [legato](https://github.com/harmony-development/legato) on `http://127.0.0.1:2289`.

## Examples
- `echo_bot`: Showcases a simple message echo bot that operates in a guild. It will repost messages whenever someone else posts a message.
- `message_log`: Showcases a simple message log bot that operates in a guild. It will log messages to the console whenever someone posts a message.

- Bot run instructions:
    - Make sure legato is running on `http://127.0.0.1:2289` or whatever you set `HOMESERVER` constant to.
    - Run bots with `cargo run --example example_name`. First run will register to the homeserver.
    - Login as bot with a client, and join your guild (eg. by creating an invite and using it with bot's account).
    - Make sure the bot has necessary permissions to view channels / send messages etc.
    - Save your guild ID to `guild_id`.
    - Run the bot again and it should now work! (hopefully™)

## Crate features
- By default, no features are enabled and only a bare-bones common API is generated. You can customize the crate to your needs by enabling feature(s) listed below:
    - Enable the `use_parking_lot` feature if you want to use [parking_lot](https://github.com/Amanieu/parking_lot) `sync` types instead of `std::sync`.
    - Enable the `client` feature for a lightweight client implementation and the client API (implies `gen_client` feature).
    - Enable the `gen_client` feature to generate client service code.
    - Enable the `gen_server` feature to generate server service code.
