{ ... }:
let
  perSystem =
    {
      codegen,
      commonArgs,
      crane,
      pkgs,
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
      ed-migratedb = pkgs.callPackage ./ed-migratedb.nix { inherit commonArgs crane; };
      ed-server = pkgs.callPackage ./ed-server.nix { inherit commonArgs crane; };
    in
    {
      packages = {
        inherit
          openapiCodegen
          ed-api
          ed-migratedb
          ed-server
          frontend
          openapiYaml
          ;
      };
    };
in
{
  inherit perSystem;
}
