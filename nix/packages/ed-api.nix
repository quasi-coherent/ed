{
  commonArgs,
  crane,
}:
crane.buildPackage {
  inherit (commonArgs)
    cargoArtifacts
    src
    pname
    version
    strictDeps
    ;
}
