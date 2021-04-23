{ pkgs ? import <nixpkgs> {}
, unstable ? import <unstable> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.alsaLib
    pkgs.openssl
    pkgs.libudev
    pkgs.lutris
    pkgs.pkg-config
    pkgs.vulkan-headers
    pkgs.vulkan-loader
    pkgs.vulkan-tools
    unstable.vulkan-validation-layers
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.xorg.libXrandr
  ];
}