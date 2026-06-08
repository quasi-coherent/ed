{ lib, ... }:
{
  perSystem =
    {
      rustTools,
      pkgs,
      self',
      ...
    }:
    let
      fmtt = pkgs.writeShellApplication {
        name = "fmtt";
        text = ''${lib.getExe self'.formatter} "$@"'';
      };
    in
    {
      devShells.default = pkgs.mkShell {
        dontDetectOcamlConflicts = true;
        RUST_SRC_PATH = "${rustTools.rust-src}/lib/rustlib/src/rust/library";
        packages = [
          fmtt
          pkgs.cachix
          pkgs.cloudflared
          pkgs.importNpmLock.hooks.linkNodeModulesHook
          pkgs.just
          pkgs.nh
          pkgs.nix-output-monitor
          pkgs.nixd
          pkgs.nodejs
          pkgs.ocamlPackages.dune
          pkgs.ocamlPackages.findlib
          pkgs.ocamlPackages.melange
          pkgs.ocamlPackages.ocaml
          pkgs.ocamlPackages.ppxlib
          pkgs.ocamlPackages.reason
          pkgs.ocamlPackages.reason-react
          pkgs.ocamlPackages.reason-react-ppx
          pkgs.sqlx-cli
          rustTools.toolchain
          self'.packages.ed-app
          self'.packages.ed-deploy
          self'.packages.ed-migratedb
          self'.packages.openapi-gen
          self'.packages.sqlx-prepare
        ];
        npmDeps =
          with pkgs;
          importNpmLock.buildNodeModules {
            npmRoot = ../frontend;
            inherit nodejs;
          };
        # linkNodeModulesHook drops node_modules in $PWD; relocate it to
        # frontend/ so vite, dune, etc. find it where the package.json lives.
        shellHook = ''
          if [ -n "$npmDeps" ]; then
            ln -sfn "$npmDeps/node_modules" frontend/node_modules
          fi
        '';
      };
    };
}
