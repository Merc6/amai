{
  description = "hyper-optimized bevy-flake for Rust-nightly development on linux";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    wild = {
      url = "github:davidlattimore/wild";
      flake = false;
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      wild,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          (import wild)
        ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        wildStdenv = pkgs.useWildLinker pkgs.stdenv;
      in
      {
        devShells.default =
          with pkgs;
          mkShell.override { stdenv = wildStdenv; } {
            buildInputs = [
              (rust-bin.nightly.latest.default.override {
                extensions = [
                  "rust-src"
                  "rustc-codegen-cranelift-preview"
                ];
              })
              clang
            ];

            RUST_SRC_PATH = "${rust-bin.nightly.latest.rust-src}/lib/rustlib/src/rust/library";
            RUSTFLAGS =
              "-C linker=clang " + "-C link-arg=-fuse-ld=wild " + "-Z share-generics=y " + "-C target-cpu=native";
          };
      }
    );
}
