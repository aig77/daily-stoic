{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nix-community/naersk";
    git-hooks.url = "github:cachix/git-hooks.nix";
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [inputs.git-hooks.flakeModule];

      systems = ["x86_64-linux" "aarch64-darwin"];

      perSystem = {system, ...}: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.default];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        naerskLib = pkgs.callPackage inputs.naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in {
        packages.default = naerskLib.buildPackage {src = ./.;};

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            rust-analyzer
            cargo-watch
          ];

          RUST_BACKTRACE = 1;

          shellHook = ''
            echo "📚 Stay Stoic... and memory safe."
            echo "🦀 $(rustc --version)"
          '';
        };

        pre-commit = {
          check.enable = true;
          settings.hooks = {
            rustfmt.enable = true;
            clippy = {
              enable = true;
              settings.offline = false;
            };
          };
        };
      };
    };
}
