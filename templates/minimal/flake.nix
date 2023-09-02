{
  inputs = {
    haumea = {
      url = "https://flakehub.com/f/nix-community/haumea/0.2.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    namaka = {
      url = "https://flakehub.com/f/nix-community/namaka/0.2.tar.gz";
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
