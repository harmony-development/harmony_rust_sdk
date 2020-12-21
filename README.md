![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yusdacra/harmony_rust_sdk/Rust)

Rust implementation of [the Harmony chat protocol](https://github.com/harmony-development).

## Requirements
- Latest stable Rust and Cargo.
- If you are using Nix, `nix-shell` (or `nix develop` if you use flakes) should get you covered.
- Otherwise, you'll need to get protobuf and make sure `protoc` is in your `PATH` env variable.
  - If for some reason `build.rs` fails, make sure to set: 
    - `PROTOC` env variable to your `protoc` executable
    - and `PROTOC_INCLUDE` env variable to wherever protobuf include files are located, most likely in `/usr/share/include`.
- For tests to work, you'll need to run [legato](https://github.com/harmony-development) on `http://127.0.0.1:2289`.

## Crate features
- Enable the `use_parking_lot` feature if you want to use [parking_lot](https://github.com/Amanieu/parking_lot) `sync` types instead of `std::sync`.
