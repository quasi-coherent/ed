{
  inputs = {
    crane.url = "github:ipetkov/crane";
    den.url = "github:denful/den";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs-lib";
    };
    limavm = {
      url = "github:quasi-coherent/limavm.nix";
      # url = "path:/Users/daniel/d/git/hub/limavm.nix";
      inputs.flake-parts.follows = "flake-parts";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";
    nixpkgs-lib.follows = "nixpkgs";
    ocaml-overlay = {
      url = "github:nix-ocaml/nix-overlays";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    sops = {
      url = "github:Mic92/sops-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = [ "https://limavm-nix.cachix.org" ];
    extra-trusted-public-keys = [
      "limavm-nix.cachix.org-1:3tRE+cBpLSZlcb6Mjgxjif+QCG6mJXuDyjyMHHXgx8I="
    ];
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [ ./nix ];

      perSystem =
        { system, ... }:
        {
          _module.args = {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.ocaml-overlay.overlays.default
                inputs.fenix.overlays.default
              ];
            };
          };
        };
    };
}
