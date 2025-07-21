{
  description = "Advent of Code 2020 in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer

            # Additional development tools
            cargo-watch
            cargo-edit
            cargo-audit

            # Git and other useful tools
            git
            ripgrep
            fd

            # Optional: Editor support
            nil # Nix language server
          ];

          shellHook = ''
            echo "🎄 Advent of Code 2020 - Rust Development Environment"
            echo ""
            echo "Available commands:"
            echo "  cargo build --release          # Build optimized binary"
            echo "  cargo run --release --         # Run all days"
            echo "  cargo run --release -- 5       # Run day 5"
            echo "  cargo test                     # Run all tests"
            echo "  cargo test day05               # Test specific day"
            echo "  cargo clippy                   # Lint code"
            echo "  cargo fmt                      # Format code"
            echo "  cargo watch -x check           # Watch for changes"
            echo ""
            echo "Happy coding! 🦀"
          '';
        };

        # For compatibility with older nix versions
        devShell = self.devShells.${system}.default;
      }
    );
}
