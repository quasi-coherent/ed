{
  ed-api,
  lib,
  writeShellApplication,
}:
writeShellApplication {
  name = "ed-migratedb";
  text = "${lib.getBin ed-api}/bin/ed-migratedb migrate apply-all";
}
