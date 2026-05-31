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
            ed-migratedb = self.packages.${system}.ed-migratedb;
          };
          modules = [
            ../modules/ed.nix
            { services.ed.enable = true; }
          ]
          ++ modules;
        };
    in
    {
      ed-host = mkSystem {
        system = "x86_64-linux";
        modules = [
          inputs.sops.nixosModules.sops
          ./secrets.nix
          { services.ed.environmentFile = "/run/secrets/ed-server"; }
          ./ed-host.nix
        ];
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
