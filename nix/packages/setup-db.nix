{
  lib,
  sqlx-cli,
  writeShellApplication,
}:
writeShellApplication {
  name = "setup-db";
  text = ''
    DOCKERHOST_URL="postgres://postgres:password@localhost:6432/ed"
    export DATABASE_URL="''${DOCKERHOST_URL:-}"
    cargo ${lib.getBin sqlx-cli}/bin/sqlx
  '';
}
