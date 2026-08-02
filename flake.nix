{
  description = "Axo - a simple Nix package helper";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;

    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "axo";
            version = "0.1.0";

            src = ./.;

            cargoHash = "sha256-ufemHlN9rXX9KqwHdHpexaUU95YbYEKEBSUUkywIPWk=";
          };
        }
      );
    };
}
