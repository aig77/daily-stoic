{pkgs ? import <nixpkgs> {}}: let
  pre-commit-hooks = import (
    builtins.fetchTarball
    "https://github.com/cachix/pre-commit-hooks.nix/tarball/master"
  );

  pre-commit-check = pre-commit-hooks.run {
    src = ./.;
    hooks = {
      rustfmt.enable = true;
      clippy.enable = true;
    };
  };
in
  pkgs.mkShell {
    packages = with pkgs; [
      rustc
      cargo
      clippy
      rustfmt
      rust-analyzer
      cargo-watch
    ];

    RUST_BACKTRACE = 1;

    shellHook = ''
      ${pre-commit-check.shellHook}
      echo "📚 Stay Stoic... and memory safe."
      echo "🦀 $(rustc --version)"
    '';
  }
