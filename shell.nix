{ pkgs ? import <nixpkgs> { } }:
with pkgs;
let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override { extensions = [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ];});
in
mkShell {
  buildInputs = [ protobuf ];
  nativeBuildInputs = [ cargo ruststable rustfmt clippy cargo-watch ];
  shellHook = ''
    export PROTOC=${protobuf}/bin/protoc
    export PROTOC_INCLUDE=${protobuf}/include
  '';
}
