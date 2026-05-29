{ inputs, ... }:
let
  perSystem =
    { pkgs, ... }:
    let
      inherit (pkgs) lib;

      root = ../..;

      # Latest build of the stable toolchain.
      rustTools = pkgs.fenix.stable;

      crane = (inputs.crane.mkLib pkgs).overrideToolchain rustTools.toolchain;

      # We have to include the sql that's part of ed-db.
      src = lib.fileset.toSource {
        inherit root;
        fileset = lib.fileset.unions [
          (crane.fileset.commonCargoSources root)
          ../../.sqlx
          ../../crates/ed-migratedb/src/migrations
          ../../crates/ed-db/src/sql
        ];
      };

      workspace = crane.crateNameFromCargoToml { inherit src; };

      cargoArtifacts = pkgs.callPackage ./workspace-dependencies.nix {
        inherit (workspace) pname version;
        inherit crane src;
        strictDeps = true;
      };

      codegen = openapiYaml: pkgs.callPackage ./codegen.nix { inherit openapiYaml; };

      commonArgs = {
        inherit (workspace) pname version;
        inherit cargoArtifacts crane src;
        strictDeps = true;
      };
    in
    {
      _module.args = {
        inherit
          cargoArtifacts
          crane
          codegen
          commonArgs
          rustTools
          src
          workspace
          ;
      };
    };
in
{
  inherit perSystem;
}
