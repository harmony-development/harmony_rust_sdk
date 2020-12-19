{
  description = "Flake for Rust implementation of the Harmony protocol";

  inputs = {
    flakeUtils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/master";
  };

  outputs = { self, nixpkgs, flakeUtils }:
    flakeUtils.lib.simpleFlake {
      inherit self nixpkgs;
      name = "harmony_rust_sdk";
      shell = ./shell.nix;
    };
}
