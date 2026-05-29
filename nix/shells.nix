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
        inputsFrom = [
          self'.packages.openapiYaml
        ];
        packages = [
          fmtt
          pkgs.cachix
          pkgs.ocamlPackages.ocaml
          pkgs.ocamlPackages.findlib
          pkgs.ocamlPackages.dune
          pkgs.importNpmLock.hooks.linkNodeModulesHook
          pkgs.just
          pkgs.ocamlPackages.melange
          pkgs.nixd
          pkgs.nix-output-monitor
          pkgs.nodejs
          pkgs.ocamlPackages.ppxlib
          pkgs.ocamlPackages.reason-react
          pkgs.ocamlPackages.reason-react-ppx
          pkgs.ocamlPackages.reason
          pkgs.sqlx-cli
          self'.packages.ed-migratedb
          self'.packages.openapiCodegen
          rustTools.toolchain
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
