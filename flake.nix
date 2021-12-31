{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixCargoIntegration = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.nixCargoIntegration.lib.makeOutputs {
    root = ./.;
    buildPlatform = "crate2nix";
    overrides = {
      crateOverrides = common: _: {
        harmony-rust-sdk = prev: {
          src = common.pkgs.fetchFromGitHub {
            owner = "harmony-development";
            repo = "harmony_rust_sdk";
            rev = inputs.self.rev;
            sha256 = common.pkgs.lib.fakeHash;
            fetchSubmodules = true;
          };
        };
      };
    };
  };
}
