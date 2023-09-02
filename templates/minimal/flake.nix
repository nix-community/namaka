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
    nixpkgs.url = "github:nix-community/nixpkgs.lib";
  };

  outputs = { self, haumea, namaka, nixpkgs }: {
    checks = namaka.lib.load {
      src = ./tests;
      inputs = {
        inherit (nixpkgs) lib;
        foo = self.lib;
      };
    };
    lib = haumea.lib.load {
      src = ./src;
      inputs = {
        inherit (nixpkgs) lib;
      };
    };
  };
}
