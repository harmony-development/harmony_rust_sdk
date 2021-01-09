{ sources, system }:
let
  pkgz = import sources.nixpkgs { inherit system; };
  mozPkgs = import "${sources.nixpkgsMoz}/package-set.nix" { pkgs = pkgz; };

  rustChannel =
    let
      channel = mozPkgs.rustChannelOf {
        channel = "nightly";
        # Replace this with the expected hash that Nix will output when trying to build the package.
        sha256 = "sha256-utyBii+c0ohEjvxvr0Cf8MB97du2Gsxozm+0Q+FhVNw=";
      };
    in
    channel // {
      rust = channel.rust.override { extensions = [ "rust-src" "rustfmt-preview" "clippy-preview" ]; };
    };

  pkgs = import sources.nixpkgs {
    inherit system;
    overlays = [
      (final: prev: {
        rustc = rustChannel.rust;
        inherit (rustChannel) cargo;
        clippy = rustChannel.clippy-preview;
        rustfmt = rustChannel.rustfmt-preview;
      })
      (final: prev: {
        naersk = prev.callPackage sources.naersk { };
      })
    ];
  };
in
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
    with pkgs;
    {
      buildInputs = [ protobuf ];
      nativeBuildInputs = [ /* Add compile time dependencies here */ ];
    };
}