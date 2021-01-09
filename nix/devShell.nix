{ common }:
with common; with pkgs;
mkShell {
  nativeBuildInputs =
    [ git nixpkgs-fmt cargo rustc cachix ]
    ++ crateDeps.nativeBuildInputs;
  buildInputs = crateDeps.buildInputs;
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath neededLibs}"
    
    export NIX_CONFIG="
      substituters = https://cache.nixos.org https://harmony-rust-sdk.cachix.org
      trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= harmony-rust-sdk.cachix.org-1:m6dligIAIzpfXY3Z3hqYLLMSCaf+fd5QklxRGN1bEUg=
    "

    export PROTOC=${protobuf}/bin/protoc
    export PROTOC_INCLUDE=${protobuf}/include
  '';
}
