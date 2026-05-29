{ self, inputs, ... }:
{
  flake.nixosConfigurations =
    let
      mkSystem =
        {
          system,
          modules,
        }:
        inputs.nixpkgs.lib.nixosSystem {
          inherit system;
          specialArgs = {
            inherit inputs;
            ed-server = self.packages.${system}.ed-server;
          };
          modules = [
            inputs.sops.nixosModules.sops
            ../modules/ed.nix
            ./secrets.nix
            {
              services.ed.enable = true;
              services.ed.environmentFile = "/run/secrets/ed-serve";
            }
          ]
          ++ modules;
        };
    in
    {
      ed-host = mkSystem {
        system = "x86_64-linux";
        modules = [ ./ed-host.nix ];
      };
      ed-lima-aarch64 = mkSystem {
        system = "aarch64-linux";
        modules = [
          inputs.nixos-lima.nixosModules.lima
          ./ed-lima.nix
        ];
      };
      ed-lima-x86_64 = mkSystem {
        system = "x86_64-linux";
        modules = [
          inputs.nixos-lima.nixosModules.lima
          ./ed-lima.nix
        ];
      };
    };
}
