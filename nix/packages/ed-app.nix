{
  ed-server,
  sops,
  lib,
  writeShellApplication,
}:
writeShellApplication {
  name = "ed-app";
  text = ''
    export PATH=${
      lib.makeBinPath [
        sops
        ed-server
      ]
    }/bin:$PATH
    export APP_FRONTEND_DIR="${ed-server}/share/ed-frontend"
    app_env="${./../..}/secrets/guest.yaml"

    sops decrypt --output-type json $app_env | ed-server
  '';
}
