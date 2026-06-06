{
  cargo,
  lib,
  sqlx-cli,
  writeShellApplication,
}:
writeShellApplication {
  name = "sqlx-prepare";
  text = ''
    if [ -z "$DATABASE_URL" ]; then
      echo "expected DATABASE_URL to be set"
      exit 1
    fi

    export PATH=${
      lib.makeBinPath [
        cargo
        sqlx-cli
      ]
    }:$PATH

    exec cargo sqlx prepare --workspace
  '';
}
