{ ... }:
let
  perSystem =
    {
      codegen,
      commonArgs,
      crane,
      pkgs,
      rustTools,
      ...
    }:
    let
      openapiYaml = pkgs.writeTextFile {
        name = "openapi.yaml";
        text = builtins.readFile ../../api/openapi.yaml;
      };
      openapiCodegen = codegen openapiYaml;

      frontend = pkgs.callPackage ./frontend.nix { };
      ed-api = pkgs.callPackage ./ed-api.nix { inherit commonArgs crane; };
      ed-lima = pkgs.callPackage ./ed-lima.nix { };
      ed-migratedb = pkgs.callPackage ./ed-migratedb.nix { inherit commonArgs crane; };
      ed-server = pkgs.callPackage ./ed-server.nix { inherit commonArgs crane frontend; };

      sqlx-prepare = pkgs.callPackage ./sqlx-prepare.nix { inherit (rustTools) cargo; };
    in
    {
      packages = {
        inherit
          openapiCodegen
          ed-api
          ed-lima
          ed-migratedb
          ed-server
          frontend
          openapiYaml
          sqlx-prepare
          ;
      };
    };
in
{
  inherit perSystem;
}
