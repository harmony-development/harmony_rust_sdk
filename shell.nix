{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [ protobuf ];
  nativeBuildInputs = [ cargo rustc rustfmt clippy ];
  shellHook = ''
    export PROTOC=${protobuf}/bin/protoc
    export PROTOC_INCLUDE=${protobuf}/include
  '';
}
