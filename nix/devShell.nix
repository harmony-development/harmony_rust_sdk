{ common }:
with common; with pkgs;
devshell.mkShell {
  packages =
    [ git nixpkgs-fmt rustc cachix binutils ]
    ++ crateDeps.nativeBuildInputs ++ crateDeps.buildInputs;
  env = {
    NIX_CONFIG = ''
      substituters = https://cache.nixos.org https://harmony-rust-sdk.cachix.org
      trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= harmony-rust-sdk.cachix.org-1:m6dligIAIzpfXY3Z3hqYLLMSCaf+fd5QklxRGN1bEUg=
    '';
    LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${lib.makeLibraryPath neededLibs}";
  } // env;
}
