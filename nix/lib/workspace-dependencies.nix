{ crane, src }:
let
  workspace = crane.crateNameFromCargoToml { inherit src; };
in
crane.buildDepsOnly {
  inherit (workspace) pname version;
  inherit src;
  strictDeps = true;
}
