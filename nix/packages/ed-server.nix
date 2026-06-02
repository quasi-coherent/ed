{
  buildPackage,
  commonArgs,
  ed-frontend,
  makeWrapper,
}:
let
  serveBin = buildPackage {
    inherit (commonArgs)
      cargoArtifacts
      src
      version
      strictDeps
      ;

    pname = "ed-server";
    cargoExtraArgs = "--bin ed-server";
    doCheck = false;
    nativeBuildInputs = [ makeWrapper ];
    postInstall = ''
      mkdir -p $out/share/ed-frontend
      cp -r --no-preserve=mode,ownership ${ed-frontend}/. $out/share/ed-frontend/
      wrapProgram $out/bin/ed-server \
        --set-default FRONTEND_DIR $out/share/ed-frontend
    '';
    meta.mainProgram = "ed-server";
  };
in
serveBin
