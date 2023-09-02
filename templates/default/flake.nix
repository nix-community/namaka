{
  inputs = {
    haumea = {
      url = "github:nix-community/haumea/v0.2.2";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    namaka = {
      url = "github:nix-community/namaka/v0.2.0";
      inputs = {
        haumea.follows = "haumea";
        nixpkgs.follows = "nixpkgs";
      };
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, haumea, namaka, nixpkgs }:
    let
      inherit (nixpkgs) lib;
      inherit (lib)
        genAttrs
        ;

      eachSystem = genAttrs [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    {
      checks = namaka.lib.load {
        src = ./tests;
        inputs = {
          inherit lib;
          foo = self.lib;
        };
      };

      devShells = eachSystem (system:
        let
          inherit (nixpkgs.legacyPackages.${system})
            mkShell
            ;
        in
        {
          default = mkShell {
            packages = [
              namaka.packages.${system}.default
            ];
          };
        });

      lib = haumea.lib.load {
        src = ./src;
        inputs = {
          inherit lib;
        };
      };
    };
}
