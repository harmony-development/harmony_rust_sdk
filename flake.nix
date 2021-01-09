{
  description = "Flake for harmony_rust_sdk";

  inputs = rec {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs = nixpkgs;
    };
    flakeUtils.url = "github:numtide/flake-utils";
    nixpkgsMoz = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = inputs: with inputs;
    with flakeUtils.lib;
    eachSystem defaultSystems (system:
      let
        common = import ./nix/common.nix {
          sources = { inherit naersk nixpkgs nixpkgsMoz; };
          inherit system;
        };
      in
      rec {
        packages = {
          # Compiles slower but has tests and faster executable
          "harmony_rust_sdk" = import ./nix/build.nix {
            inherit common;
            doCheck = true;
            release = true;
          };
          # Compiles faster but no tests and slower executable
          "harmony_rust_sdk-debug" = import ./nix/build.nix { inherit common; };
          # Compiles faster but has tests and slower executable
          "harmony_rust_sdk-tests" = import ./nix/build.nix { inherit common; doCheck = true; };
        };
        # Release build is the default package
        defaultPackage = packages."harmony_rust_sdk";

        devShell = import ./nix/devShell.nix { inherit common; };
      }
    );
}
