{
  inputs = {
    call-flake.url = "github:divnix/call-flake";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    namaka = {
      url = "https://flakehub.com/f/nix-community/namaka/0.2.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ call-flake, flake-parts, namaka, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      flake.checks = namaka.lib.load {
        src = ./tests;
        inputs = {
          foo = (call-flake ../.).lib;
        };
      };

      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      perSystem = { inputs', pkgs, ... }: {
        devShells.default = pkgs.mkShell {
          packages = [
            inputs'.namaka.packages.default
          ];
        };
      };
    };
}
