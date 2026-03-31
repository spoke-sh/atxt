{
  description = "atxt development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    keel = {
      url = "github:spoke-sh/keel";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };
    sift = {
      url = "github:rupurt/sift";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.keel.follows = "keel";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
      keel,
      sift,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "clippy"
            "rustfmt"
            "rust-src"
            "rust-analyzer"
            "llvm-tools"
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;
        keelSrc = keel.outPath;
        keelPkg = pkgs.callPackage "${keelSrc}/nix/keel.nix" {
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
        };
        siftPkg = sift.packages.${system}.sift;
        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          outputHashes = {
            "txtplot-0.1.0" = "sha256-bC6zo1yhJg41iz69XbXqwIKOfNVXwFke0vzcSMbqvFE=";
          };
        });
        package = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
      in
      {
        packages = {
          default = package;
          keel = keelPkg;
          sift = siftPkg;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.cargo-nextest
            pkgs.cargo-llvm-cov
            pkgs.just
            pkgs.ffmpeg
            pkgs.util-linux
            pkgs.vhs
            pkgs.pkg-config
            keelPkg
            siftPkg
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
            pkgs.mold
          ];

          shellHook =
            ''
              export CARGO_TARGET_DIR="$HOME/.cache/cargo-target/atxt"
            ''
            + pkgs.lib.optionalString pkgs.stdenv.isDarwin ''
              export TMPDIR=/var/tmp
            ''
            + pkgs.lib.optionalString pkgs.stdenv.isLinux ''
              export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-fuse-ld=mold"
              export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-fuse-ld=mold"
            '';
        };

        checks = {
          inherit package;

          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets --all-features -- -W clippy::all -D warnings";
            }
          );

          test = craneLib.cargoNextest (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );
        };
      }
    );
}
