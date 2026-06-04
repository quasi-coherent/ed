{ inputs, ... }:
{
  imports = [
    inputs.den.flakeModules.default
    inputs.limavm.flakeModules.den
    inputs.treefmt-nix.flakeModule

    ./checks.nix
    ./lib.nix
    ./nixos
    ./packages
    ./shells.nix
  ];

  perSystem =
    {
      lib,
      pkgs,
      self',
      ...
    }:
    {
      apps.default = {
        meta = "Format project source";
        program = pkgs.writeShellApplication {
          name = "fmtt";
          text = ''${lib.getExe self'.formatter} "$@"'';
        };
      };

      treefmt = {
        projectRootFile = "flake.nix";
        programs = {
          rustfmt.enable = true;
          nixfmt.enable = true;
          taplo.enable = true;
          typos.enable = true;
        };
        settings.excludes = [
          "secrets/*"
          ".direnv/*"
        ];
        settings.formatter = {
          "refmt" = {
            command = "${pkgs.bash}/bin/bash";
            options = [
              "-euc"
              ''
                for file in "$@"; do
                  ${pkgs.ocamlPackages.reason}/bin/refmt --in-place $file
                done
              ''
              "--"
            ];
            includes = [ "*.re" ];
          };
        };
      };
    };
}
