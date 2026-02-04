{
  description = "Vine SAT solver";

  inputs = {
    nixpkgs.follows = "vine/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    vine.url = "github:VineLang/vine/dev";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      vine,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        vineCli = vine.packages.${system}.vine;

        sat = pkgs.rustPlatform.buildRustPackage {
          pname = "vine-sat-solver";
          version = "0.1.0";
          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          doCheck = true;
          nativeCheckInputs = [ vineCli ];
          cargoTestFlags = [ "--locked" ];
          preCheck = ''
            export HOME=$TMPDIR
            export INSTA_UPDATE=no
          '';
        };
      in
      {
        formatter = pkgs.nixfmt-tree;

        checks = {
          cargo-test = sat;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            vineCli
            pkgs.rustc
            pkgs.cargo
          ];
        };
      }
    );
}
