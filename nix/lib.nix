{ inputs, ... }:
let
  perSystem =
    { pkgs, ... }:
    let
      inherit (pkgs) lib;

      root = ../.;

      # Latest build of the stable toolchain.
      rustTools = pkgs.fenix.stable;

      crane = (inputs.crane.mkLib pkgs).overrideToolchain rustTools.toolchain;

      # We have to include the sql that's part of ed-db.
      src = lib.fileset.toSource {
        inherit root;
        fileset = lib.fileset.unions [
          (crane.fileset.commonCargoSources root)
          ../.sqlx
          ../crates/ed-migratedb/src/migrations
          ../crates/ed-db/src/sql
        ];
      };

      workspace = crane.crateNameFromCargoToml { inherit src; };

      # The goal of this derivation is to include everything in the workspace
      # dependencies.  That way everything that is needed to build crates here
      # gets hashed and added to the nix store, which you combine with a caching
      # solution like cachix and the result is instant builds for anyone who can
      # pull from the cache.
      cargoArtifacts = crane.buildDepsOnly {
        inherit (workspace) pname version;
        inherit src;
        strictDeps = true;
      };

      commonArgs = {
        inherit (workspace) pname version;
        inherit cargoArtifacts crane src;
        strictDeps = true;
      };
    in
    {
      _module.args = {
        inherit
          crane
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
