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
      sqlx-prepare = pkgs.callPackage ./sqlx-prepare.nix { inherit (rustTools) cargo; };
      openapiYaml = pkgs.writeTextFile {
        name = "openapi.yaml";
        text = builtins.readFile ../../api/openapi.yaml;
      };
      openapiCodegen = codegen openapiYaml;
      ed-migratedb = pkgs.callPackage ./ed-migratedb.nix { inherit commonArgs crane; };

      ed-api = pkgs.callPackage ./ed-api.nix { inherit commonArgs crane; };
      frontend = pkgs.callPackage ./frontend.nix { };
      ed-server = pkgs.callPackage ./ed-server.nix { inherit commonArgs crane frontend; };

      ed-lima = pkgs.callPackage ./ed-lima.nix { edLimaTopLevel = ed-server; };
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

        # Alternative to nix + VM.
        ed-docker = pkgs.dockerTools.buildImage {
          name = "ed";
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "ed-bundle";
            paths = [ ed-server ];
            pathsToLink = [ "/ed" ];
          };
          config.Cmd = [ "/ed/bin/ed-server" ];
        };
      };
    };
in
{
  inherit perSystem;
}
