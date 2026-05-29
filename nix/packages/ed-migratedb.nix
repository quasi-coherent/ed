{
  commonArgs,
  crane,
}:
crane.buildPackage {
  inherit (commonArgs)
    cargoArtifacts
    src
    strictDeps
    version
    ;
  pname = "ed-migratedb";
  cargoExtraArgs = "-p ed-migratedb --bin ed-migratedb";
}
