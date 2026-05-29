{ ... }:
{
  perSystem =
    {
      cargoArtifacts,
      crane,
      crateName,
      self',
      src,
      ...
    }:
    let
      inherit (crateName { }) pname version;
    in
    {
      checks = {
        # Build all workspace members as part of checks.
        inherit (self'.packages) facade-rs other-rs;

        cargo-clippy = crane.cargoClippy {
          inherit
            pname
            version
            cargoArtifacts
            src
            ;
          strictDeps = true;
          cargoClippyExtraArgs = "--all-features --all-targets -- -Dwarnings";
        };

        cargo-test = crane.cargoTest {
          inherit
            pname
            version
            cargoArtifacts
            src
            ;
          strictDeps = true;
          cargoTestExtraArgs = "--all-features --all-targets";
        };
      };
    };
}
