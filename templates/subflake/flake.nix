{
  inputs = {
    haumea = {
      url = "https://flakehub.com/f/nix-community/haumea/0.2.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nix-community/nixpkgs.lib";
  };

  outputs = { self, haumea, nixpkgs }: {
    lib = haumea.lib.load {
      src = ./src;
      inputs = {
        inherit (nixpkgs) lib;
      };
    };
  };
}
