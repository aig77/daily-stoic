{
  description = ''
    An flake to configure and manage the development environment for an
    API to retrieve Daily Stoic quotes from Ryan Holiday's "The Daily Stoic".
  '';

  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devenv.url = "github:cachix/devenv";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = [ "x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, ... }: {
        devenv.shells.default = {
          name = "daily-stoic-api-rs";
          
          packages = with pkgs; [ cargo-watch ];

          env = {
            RUST_BACKTRACE = 1;
          };

          languages.rust = {
            enable = true;
            channel = "stable";
            components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
          };

          pre-commit.hooks = {
            rustfmt.enable = true;
            clippy.enable = true;
          };

          enterShell = ''
            echo "📚 Stay Stoic... and memory safe."
            echo "🦀 $(rustc --version)"
          '';
        };
      };
    };
  }
