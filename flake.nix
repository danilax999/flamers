{
  description = "Basic cargo package flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = fenix.packages.${system}.latest.withComponents [
          "toolchain"
          "rustfmt"
        ];
        craneLib = crane.lib.${system}.overrideToolchain rustToolchain;
        makePkgConfigPath = pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig";
      in
      rec {
        packages.flamers = craneLib.buildPackage rec {
          src = craneLib.cleanCargoSource ./.;
          cargoArtifacts = craneLib.buildDepsOnly { inherit src; };
          checks = [
            (craneLib.cargoClippy {
              inherit cargoArtifacts src;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            })
          ];
        };
        packages.default = packages.flamers;

        apps.flamers = flake-utils.lib.mkApp { drv = packages.flamers; };
        apps.default = apps.flamers;

        devShells.default = pkgs.mkShell rec {
          packages = with pkgs; [ rustToolchain ];
          buildInputs = with pkgs; [];
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH"
            export PKG_CONFIG_PATH="${makePkgConfigPath buildInputs}:$PKG_CONFIG_PATH"
          '';
        };
      }
    );
}
