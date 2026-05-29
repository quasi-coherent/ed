{ cargoArtifacts, crane }:
let
  src = crane.cleanCargoSource ../..;

  workspace = crane.crateNameFromCargoToml { inherit src; };
in
crane.buildPackage {
  inherit (workspace) pname version;
  inherit cargoArtifacts src;
  strictDeps = true;
}
