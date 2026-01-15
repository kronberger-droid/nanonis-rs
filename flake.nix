{
  description = "Rust development shell with GUI support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default rust-overlay.overlays.default];
        };

        # Rust toolchain configuration
        # Pin to Rust 1.89.0
        rustTools = {
          stable = pkgs.rust-bin.stable."1.89.0".default.override {
            extensions = ["rust-src"];
          };
          analyzer = pkgs.rust-bin.stable."1.89.0".rust-analyzer;
        };

        # Development tools
        devTools = with pkgs; [
          cargo-expand
          rusty-man
          pkg-config
        ];

        # Core Rust development dependencies
        rustDeps =
          [
            rustTools.stable
            rustTools.analyzer
          ]
          ++ devTools;

        # Base shell configuration
        baseShellHook = ''
          echo "Using Rust toolchain: $(rustc --version)"
          export CARGO_HOME="$HOME/.cargo"
          export RUSTUP_HOME="$HOME/.rustup"


          mkdir -p "$CARGO_HOME" "$RUSTUP_HOME"
        '';
      in {
        devShells.default = pkgs.mkShell {
          name = "rust dev shell (clean)";
          buildInputs = rustDeps;
          shellHook = baseShellHook;
        };
      }
    );
}
