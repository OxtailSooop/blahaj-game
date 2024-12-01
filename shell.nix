# TODO: make this a flake
{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    cargo
    rustc
    rust-analyzer
    pkgs.rust.packages.stable.rustPlatform.rustcSrc
    rustfmt
  ];
  buildInputs = [
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}