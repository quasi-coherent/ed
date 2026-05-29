{ ... }:
{
  perSystem =
    { commonArgs, ... }:
    let
      inherit (commonArgs) crane;
    in
    {
      checks = {
        cargo-clippy = crane.cargoClippy {
          inherit (commonArgs)
            pname
            version
            cargoArtifacts
            src
            strictDeps
            ;
          cargoClippyExtraArgs = "--keep-going --all-targets -- -Dwarnings";
        };
      };
    };
}
