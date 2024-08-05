{ pkgs ? import <nixpkgs> {} }:

let
  rustOverlay = import (builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  });
  pkgsWithRust = import pkgs.path { overlays = [ rustOverlay ]; };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.git
    pkgs.openssl
    pkgs.pkg-config
    pkgs.zsh
    pkgsWithRust.rust-bin.stable."1.79.0".default
    pkgs.wayland
    pkgs.wayland-protocols
    pkgs.glfw
  ];

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib
    export WAYLAND_DISPLAY=wayland-0
    export XDG_SESSION_TYPE=wayland
    echo "Welcome to the Rust development environment!"
  '';
}
