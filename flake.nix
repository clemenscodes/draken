{
  description = "draken development shell";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
        flake-utils = {
          follows = "flake-utils";
        };
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = ["rust-src" "clippy"];
        };
        nativeBuildInputs = with pkgs; [rustToolchain pkg-config wrapGAppsHook cargo-tauri];
        libraries = with pkgs; [
          webkitgtk
          webkitgtk_4_1
          libsoup
          openssl_3
          atk
          pango
          cairo
          gdk-pixbuf
          dbus
          zlib
        ];
        tauri_packages = with pkgs; [
          wget
          pkg-config
          atk
          pango
          libsoup_3
          webkitgtk
          webkitgtk_4_1
          openssl_3
          appimagekit
        ];
        buildInputs = with pkgs; [
          rust-analyzer
          nodejs_20
          corepack_20
        ] ++ tauri_packages ;
      in
        with pkgs; {
          devShells.default = mkShell {
            inherit buildInputs nativeBuildInputs;
            RUST_BACKTRACE = 1;
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
            NIX_LD = "${pkgs.stdenv.cc.libc}/lib/ld-linux-x86-64.so.2";
            shellHook = ''
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
            '';
          };
        }
    );
}
