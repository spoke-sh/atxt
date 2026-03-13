{
  description = "atext development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    keel = {
      url = "git+ssh://git@github.com/spoke-sh/keel.git?ref=main&rev=91a2bb745112da11412b508d52abd1a8d312a754";
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
        keelCargoToml = pkgs.lib.importTOML "${keelSrc}/Cargo.toml";
        keelRustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        keelPkg = keelRustPlatform.buildRustPackage {
          pname = "keel";
          version = keelCargoToml.package.version;
          src = keelSrc;
          cargoLock = {
            lockFile = "${keelSrc}/Cargo.lock";
            outputHashes = {
              "txtplot-0.1.0" = "sha256-PXj4ntPJ1UXda++7gcE+yk2cCLy/CFBMBGxgfBGSH5c=";
            };
          };
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
          nativeCheckInputs = [
            pkgs.git
          ];
          buildInputs = [
            pkgs.zstd
          ];
          # Mirror dojo: keep the CLI available even while upstream doctests are failing.
          doCheck = false;
        };
        siftPkg = sift.packages.${system}.sift;
        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
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
              export CARGO_TARGET_DIR="$HOME/.cache/cargo-target/atext"
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
