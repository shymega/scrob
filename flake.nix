# SPDX-FileCopyrightText: 2023 Dom Rodriguez
#
# SPDX-License-Identifier: MIT

{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };

        toolchain = (pkgs.rustChannelOf {
          date = "2023-08-03";
          channel = "stable";
          sha256 = "sha256-R0F0Risbr74xg9mEYydyebx/z0Wu6HI0/KWwrV30vZo=";
        }).rust.override {
          targets = [
            "x86_64-unknown-linux-musl"
            "armv7-unknown-linux-musleabi"
            "armv7-unknown-linux-musleabihf"
            "aarch64-unknown-linux-musl"
          ];
          extensions = [
            "rust-src"
            "rustfmt-preview"
            "llvm-tools-preview"
          ];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ systemd.dev protobuf ];
          cargoBuildOptions = (opts: opts ++ [ "--all-features" "--all-targets" ]);
          release = true;
          singleStep = true;
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ toolchain ] ++ (with pkgs; [ pkg-config ]);
          buildInputs = with pkgs; [ systemd.dev protobuf ];
        };
      }
    );
}
