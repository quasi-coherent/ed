{
  crane,
  frontend,
  lib,
  makeWrapper,
  stdenv,
}:
let
  src = crane.cleanCargoSource ../..;
  workspace = crane.crateNameFromCargoToml { inherit src; };

  cargoExtraArgs = "--bin ed-serve";

  # Deps-only build scoped to the ed-serve binary so the broken sibling
  # crates (ed-db, ed-axum) aren't pulled in.
  cargoArtifacts = crane.buildDepsOnly {
    inherit (workspace) pname version;
    inherit src cargoExtraArgs;
    pnameSuffix = "-serve-deps";
    strictDeps = true;
  };

  serveBin = crane.buildPackage {
    inherit (workspace) pname version;
    inherit src cargoArtifacts cargoExtraArgs;
    strictDeps = true;
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
