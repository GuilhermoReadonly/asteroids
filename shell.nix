with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustup
    vulkan-tools

    # Example Build-time Additional Dependencies
    pkg-config
    systemd.dev
    alsaLib
    xorg.libX11
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    openssl

    lutris
    pkgconfig
    vulkan-headers
    vulkan-loader
    vulkan-tools
    vulkan-validation-layers
    x11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

LD_LIBRARY_PATH = stdenv.lib.makeLibraryPath [
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libglvnd
  ];
  
  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
