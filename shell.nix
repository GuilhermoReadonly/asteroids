with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustup

    # Example Build-time Additional Dependencies
    pkg-config
    systemd.dev
    alsaLib
    xorg.libX11
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    openssl
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
