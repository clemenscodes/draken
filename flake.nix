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
          config = {
            allowUnfree = true;
            android_sdk = {
              accept_license = true;
            };
          };
        };
        rustToolchain = (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = ["rust-src" "clippy"];
        };
        androidEnv = pkgs.androidenv.override {licenseAccepted = true;};
        androidComposition = androidEnv.composeAndroidPackages {
          includeEmulator = true;
          includeNDK = true;
          emulatorVersion = "34.1.9";
          platformToolsVersion = "33.0.3";
          buildToolsVersions = ["30.0.3"];
          ndkVersions = ["26.1.10909125"];
          platformVersions = ["33"];
        };
        nativeBuildInputs = with pkgs; [
          androidComposition.androidsdk
          androidComposition.ndk-bundle
          rustToolchain
          pkg-config
          lldb
          jetbrains.jdk
        ];
        libraries = with pkgs; [
          webkitgtk
          webkitgtk_4_1
          libsoup
          openssl_3
          atk
          cairo
          gdk-pixbuf
          dbus
          zlib
          lldb
        ];
        tauri_packages = with pkgs; [
          wget
          pkg-config
          atk
          libsoup_3
          webkitgtk
          webkitgtk_4_1
          openssl_3
          librsvg
        ];
        buildInputs = with pkgs;
          [
            rust-analyzer
            nodejs_20
            corepack_20
          ]
          ++ tauri_packages;
      in
        with pkgs; {
          devShells.default = mkShell rec {
            inherit buildInputs nativeBuildInputs;
            RUST_BACKTRACE = 1;
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
            NIX_LD = "${pkgs.stdenv.cc.libc}/lib/ld-linux-x86-64.so.2";
            ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
            NDK_HOME = "${ANDROID_HOME}/ndk-bundle";
            NDK_LIBS = "${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/lib";
            JAVA_HOME = "${pkgs.jetbrains.jdk}";
            shellHook = ''
              export LD_LIBRARY_PATH=${NDK_LIBS}:${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
            '';
          };
        }
    );
}
