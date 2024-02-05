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
        nativeBuildInputs = with pkgs; [rustToolchain pkg-config];
        libraries = with pkgs; [
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl_3
          librsvg
        ];
        buildInputs = with pkgs; [
          openssl
          rust-analyzer
          nodejs_20
          corepack_20
          gtk4
          glib.dev
          curl
          wget
          dbus
          openssl_3
          glib
          gtk3
          libsoup
          webkitgtk
          librsvg
        ];
      in
        with pkgs; {
          devShells.default = mkShell {
            inherit buildInputs nativeBuildInputs;
            RUST_BACKTRACE = 1;
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            shellHook = ''
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
              export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
            '';
          };
        }
    );
}
