{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [ ];
  nativeBuildInputs = [ protobuf pkg-config cargo rustc rustfmt clippy ];
  shellHook = ''
    export PROTOC=${protobuf}/bin/protoc
    export PROTOC_INCLUDE=${protobuf}/include
  '';
}
