{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    parts,
    nixpkgs,
    naersk,
    fenix,
    ...
  }: let
    # crate name
    crateName = "malaysia-prayertimes";
  in
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];
      imports = [parts.flakeModules.easyOverlay];
      perSystem = {
        config,
        pkgs,
        system,
        ...
      }: let
        # custom toolchain for the latest stable from fenix
        toolchain = fenix.packages.${system}.stable.toolchain;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        builder = {release ? true}:
          naersk'.buildPackage {
            src = self;

            # dependencies required to build
            nativeBuildInputs = with pkgs; [pkg-config];
            buildInputs = with pkgs; [];

            inherit release;
          };
      in rec {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
          config.allowUnfree = true;
        };

        packages.default = packages.${crateName};
        packages.${crateName} = packages.release;
        packages.release = builder {release = true;};
        packages.debug = builder {release = false;};

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            toolchain
            just
            ripgrep
            stdenv.cc

            openssl

            # testing apparatus
            cargo-machete
          ];
          inputsFrom = [packages.default];
        };

        # export overlay using easyOverlays
        overlayAttrs = {
          # dynamic variables are not allowed
          inherit (config.packages) malaysia-prayertimes;
        };
      };
      flake = {
        homeManagerModules = {
          ${crateName} = import ./nix/hm-module.nix inputs.self;
          default = inputs.self.homeManagerModules.${crateName};
        };
      };
    };
}
