{
  description = "Snapshot testing for Nix based on haumea";

  inputs = {
    haumea = {
      url = "github:nix-community/haumea/v0.2.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, haumea, nixpkgs }:
    let
      inherit (nixpkgs) lib;
      inherit (lib)
        genAttrs
        importTOML
        licenses
        maintainers
        sourceByRegex
        ;

      eachSystem = f: genAttrs
        [
          "aarch64-darwin"
          "aarch64-linux"
          "x86_64-darwin"
          "x86_64-linux"
        ]
        (system: f nixpkgs.legacyPackages.${system});

      src = sourceByRegex self [
        "src(/.*)?"
        ''Cargo\.(toml|lock)''
        ''build\.rs''
      ];
    in
    {
      checks = self.lib.load {
        src = ./tests;
        inputs = {
          namaka = self.lib;
        };
      };

      formatter = eachSystem (pkgs: pkgs.nixpkgs-fmt);

      herculesCI.ciSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      lib = haumea.lib.load {
        src = ./nix;
        inputs = {
          inherit lib;
          haumea = haumea.lib;
        };
      };

      packages = eachSystem (pkgs:
        let
          inherit (pkgs)
            installShellFiles
            oniguruma
            pkg-config
            rustPlatform
            ;

          inherit (importTOML (src + "/Cargo.toml")) package;
        in
        {
          default = rustPlatform.buildRustPackage {
            pname = package.name;
            inherit (package) version;

            inherit src;

            cargoLock = {
              lockFile = src + "/Cargo.lock";
            };

            nativeBuildInputs = [
              installShellFiles
              pkg-config
            ];

            buildInputs = [
              oniguruma
            ];

            env = {
              GEN_ARTIFACTS = "artifacts";
              RUSTONIG_SYSTEM_LIBONIG = true;
            };

            postInstall = ''
              installManPage artifacts/*.1
              installShellCompletion artifacts/namaka.{bash,fish} --zsh artifacts/_namaka
            '';

            meta = {
              inherit (package) description;
              license = licenses.mpl20;
              maintainers = with maintainers; [ figsoda ];
            };
          };
        });

      templates = {
        default = {
          path = ./templates/default;
          description = "A Nix library";
        };
        minimal = {
          path = ./templates/minimal;
          description = "A Nix library that does not depend on the entirety of nixpkgs";
        };
      };
    };
}
