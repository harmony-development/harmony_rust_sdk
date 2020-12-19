{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [ ];
  nativeBuildInputs = [ protobuf pkg-config cargo rustc rustfmt clippy ];
}
