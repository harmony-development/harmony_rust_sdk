{ release ? false
, doCheck ? false
, doDoc ? false
, common
,
}:
with common;
let
  meta = with pkgs.stdenv.lib; {
    description = "Rust implementation of the Harmony protocol.";
    homepage = "https://github.com/harmony-development/harmony_rust_sdk";
    license = licenses.mit;
  };

  package = with pkgs; naersk.buildPackage {
    root = ../.;
    nativeBuildInputs = crateDeps.nativeBuildInputs;
    buildInputs = crateDeps.buildInputs;
    cargoBuildOptions = defs: defs ++ [ "--all-features" ];
    cargoTestOptions = defs: defs ++ [ "--all-features" "--tests" "--" "api" ];
    override = (_: (lib.listToAttrs (map (e: { "${e.name}" = e.value; }) env)));
    overrideMain = (prev: {
      inherit meta;
      nativeBuildInputs = prev.nativeBuildInputs ++ [ rustfmt ];
    });
    copyLibs = true;
    inherit release doCheck doDoc;
  };
in
package
