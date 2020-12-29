{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [ protobuf ];
  nativeBuildInputs = [ cargo rustc rustfmt clippy cargo-watch ];
  shellHook = ''
    export PROTOC=${protobuf}/bin/protoc
    export PROTOC_INCLUDE=${protobuf}/include
  '';
}
