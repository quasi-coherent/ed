{ ... }:
{
  perSystem =
    {
      cargoArtifacts,
      crane,
      fullSource,
      ...
    }:
    let
      workspace = crane.crateNameFromCargoToml { src = fullSource; };
    in
    {
      checks = {
        cargo-clippy = crane.cargoClippy {
          inherit (workspace) pname version;
          inherit cargoArtifacts;
          # Targeting all of the workspace needs the wider fileset.
          src = fullSource;
          strictDeps = true;
          cargoClippyExtraArgs = "--all-targets -- -Dwarnings";
        };
      };
    };
}
