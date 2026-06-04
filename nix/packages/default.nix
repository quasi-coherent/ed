{ ... }:
let
  perSystem =
    {
      commonArgs,
      crane,
      pkgs,
      rustTools,
      self',
      ...
    }:
    let
      inherit (pkgs) lib;

      ed-frontend = pkgs.callPackage ./ed-frontend.nix { };

      ed-server = pkgs.callPackage ./ed-server.nix {
        inherit commonArgs ed-frontend;
        inherit (crane) buildPackage;
      };
    in
    {
      packages = {
        inherit ed-server ed-frontend;

        default = pkgs.writeShellApplication {
          name = "fmtt";
          text = "${lib.getExe self'.formatter}";
        };

        ed-migratedb = crane.buildPackage {
          inherit (commonArgs)
            cargoArtifacts
            src
            strictDeps
            version
            ;
          pname = "ed-migratedb";
          cargoExtraArgs = "-p ed-migratedb --bin ed-migratedb";
          meta.mainProgram = "ed-migratedb";
        };

        openapiGen =
          let
            openapiYaml = pkgs.writeTextFile {
              name = "openapi.yaml";
              text = builtins.readFile ../../api/openapi.yaml;
            };
          in
          pkgs.callPackage ./openapi-gen.nix { inherit openapiYaml; };

        sqlx-prepare = pkgs.callPackage ./sqlx-prepare.nix { inherit (rustTools) cargo; };
      };
    };
in
{
  inherit perSystem;
}
