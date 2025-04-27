{
  description = "Rust dev shell for local project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system;
          overlays = overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.rust-analyzer
            pkgs.pkg-config
            pkgs.openssl
          ];

          shellHook = ''
            echo "Welcome to your Rust dev shell 🦀"
          '';
        };
      }
    );
}
