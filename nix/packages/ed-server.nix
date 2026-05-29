{
  commonArgs,
  crane,
  frontend,
  makeWrapper,
}:
let
  serveBin = crane.buildPackage {
    inherit (commonArgs)
      cargoArtifacts
      crane
      src
      version
      strictDeps
      ;

    pname = "ed-server";
    cargoExtraArgs = "--bin ed-serve";
    doCheck = false;
    nativeBuildInputs = [ makeWrapper ];
    postInstall = ''
      mkdir -p $out/share/ed-frontend
      cp -r --no-preserve=mode,ownership ${frontend}/. $out/share/ed-frontend/
      wrapProgram $out/bin/ed-serve \
        --set-default FRONTEND_DIR $out/share/ed-frontend
    '';
    meta.mainProgram = "ed-serve";
  };
in
serveBin
