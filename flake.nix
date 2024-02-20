{
    description = "Rust development environment";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
        let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
                inherit system overlays;
            };
        in with pkgs; {
            devShells.default = mkShell {
                buildInputs = [
                    (lib.hiPrio (rust-bin.stable.latest.minimal.override {
                        extensions = [ "rust-docs" "clippy" "rust-analyzer" "rust-src" ];
                    }))
                    (rust-bin.selectLatestNightlyWith (toolchain:
                        toolchain.minimal.override {
                            extensions = [ "rustfmt" ];
                        }
                    ))

                    pkgs.bashInteractive
                ];
            };
        }
    );
}
