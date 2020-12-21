![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yusdacra/harmony_rust_sdk/Rust)

Rust implementation of [the Harmony chat protocol](https://github.com/harmony-development).

## Requirements
- Latest stable Rust and Cargo.
- If you are using Nix, `nix-shell` (or `nix develop` if you use flakes) should get you covered.
- Otherwise, you'll need to get protobuf and make sure `protoc` is in your `PATH` env variable.
  - If for some reason `build.rs` fails, make sure to set: 
    - `PROTOC` env variable to your `protoc` executable
    - and `PROTOC_INCLUDE` env variable to wherever protobuf include files are located, most likely in `/usr/share/include`.
- For tests to work, you'll need to run [legato](https://github.com/harmony-development/legato) on `http://127.0.0.1:2289`.

## Examples
- `echo_bot`: Showcases a simple message echo bot that operates in one channel.
    - Make sure legato is running on `http://127.0.0.1:2289` or whatever you set `HOMESERVER` constant to.
    - Run the bot once with `cargo run --example echo_bot`, it will register to the homeserver.
    - Login as bot with a client, and join your guild (eg. by creating an invite and using it with bot's account).
    - Save your `guild_id` to `guild_id` and `channel_id` to `channel_id`.
    - Run the bot again and it should now work!

## Crate features
- Enable the `use_parking_lot` feature if you want to use [parking_lot](https://github.com/Amanieu/parking_lot) `sync` types instead of `std::sync`.
