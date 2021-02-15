{ common }:
with common; with pkgs;
devshell.mkShell {
  packages = [ rustc binutils ] ++ crateDeps.nativeBuildInputs ++ crateDeps.buildInputs;
  commands =
    let
      pkgCmd = pkg: { package = pkg; };
    in
    [
      (pkgCmd git)
      (pkgCmd nixpkgs-fmt)
      (pkgCmd cachix)
    ];
  env =
    let
      nv = lib.nameValuePair;
    in
    [
      (
        nv "NIX_CONFIG" ''
          substituters = https://cache.nixos.org https://harmony-rust-sdk.cachix.org
          trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= harmony-rust-sdk.cachix.org-1:m6dligIAIzpfXY3Z3hqYLLMSCaf+fd5QklxRGN1bEUg=
        ''
      )
    ] ++ env;
}
