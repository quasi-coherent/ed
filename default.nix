inputs:
inputs.flake-parts.lib.mkFlake { inherit inputs; } {
  systems = import inputs.systems;
  imports = [ ./nix ];

  perSystem =
    { system, ... }:
    {
      _module.args = {
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.ocaml-overlay.overlays.default
            inputs.fenix.overlays.default
          ];
        };
      };
    };
}
