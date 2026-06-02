{
  importNpmLock,
  lib,
  nodejs,
  ocamlPackages,
  stdenv,
}:
let
  # Exclude everything except what vite/dune actually reads so that the
  # store path doesn't change all the time.
  src = lib.cleanSourceWith {
    src = ../../frontend;
    filter =
      path: type:
      let
        base = baseNameOf path;
      in
      !(base == "_build" || base == "dist" || base == "node_modules");
  };

  npmDeps = importNpmLock.buildNodeModules {
    npmRoot = ../../frontend;
    inherit nodejs;
  };
in
stdenv.mkDerivation {
  pname = "ed-frontend";
  version = "0.1.0";
  inherit src;

  # False positive from the same dependency showing up twice.
  dontDetectOcamlConflicts = true;

  nativeBuildInputs = [
    importNpmLock.hooks.linkNodeModulesHook
    nodejs
    ocamlPackages.dune
    ocamlPackages.findlib
    ocamlPackages.melange
    ocamlPackages.ocaml
    ocamlPackages.ppxlib
    ocamlPackages.reason
    ocamlPackages.reason-react
    ocamlPackages.reason-react-ppx
  ];

  # linkNodeModulesHook reads $npmDeps and symlinks
  # $npmDeps/node_modules into $PWD before the build phase.
  inherit npmDeps;

  buildPhase = ''
    runHook preBuild
        dune build
        node_modules/.bin/vite build
        runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
        mkdir -p $out
        cp -r dist/. $out/
        runHook postInstall
  '';
}
