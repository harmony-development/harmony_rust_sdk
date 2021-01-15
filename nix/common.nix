{ sources, system }:
let
  pkgz = import sources.nixpkgs { inherit system; };
  mozPkgs = import "${sources.nixpkgsMoz}/package-set.nix" { pkgs = pkgz; };

  rustChannel =
    let
      channel = mozPkgs.rustChannelOf {
        date = "2020-12-31";
        channel = "stable";
        sha256 = "sha256-KCh2UBGtdlBJ/4UOqZlxUtcyefv7MH1neoVNV4z0nWs=";
      };
    in
    channel // {
      rust = channel.rust.override { extensions = [ "rust-src" "rustfmt-preview" "clippy-preview" ]; };
    };

  pkgs = import sources.nixpkgs {
    inherit system;
    overlays = [
      sources.devshell.overlay
      (final: prev: {
        rustc = rustChannel.rust;
      })
      (final: prev: {
        naersk = prev.callPackage sources.naersk { };
      })
    ];
  };
in
with pkgs;
{
  inherit pkgs;

  /* You might need this if your application utilizes a GUI. Note that the dependencies
    might change from application to application. The example dependencies provided here
    are for a typical iced application that uses Vulkan underneath.

    For example, it might look like this:

    neededLibs = with pkgs; (with xorg; [ libX11 libXcursor libXrandr libXi ])
    ++ [ vulkan-loader wayland wayland-protocols libxkbcommon ]; */
  neededLibs = [ ];

  # Dependencies listed here will be passed to Nix build and development shell
  crateDeps =
    {
      buildInputs = [ protobuf ];
      nativeBuildInputs = [ /* Add compile time dependencies here */ ];
    };

  env = {
    PROTOC = "${protobuf}/bin/protoc";
    PROTOC_INCLUDE = "${protobuf}/include";
  };
}
