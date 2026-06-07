{ den, inputs, ... }:
let
  baseImage = inputs.limavm.packages.aarch64-linux.lima-base-image;

  local = inputs.nixpkgs.lib.nixosSystem {
    system = "aarch64-linux";
    modules = [
      inputs.limavm.nixosModules.guest
      {
        lima = {
          enable = true;
          cpus = 4;
          memory = "8GiB";
          vmType = "vz";
          image = "${baseImage}/nixos.qcow2";
          mounts = [
            {
              # This flake has to be visible inside the VM.
              location = toString ../.;
              writable = false;
            }
            {
              location = "~/.config/sops/age/lima-keys.txt";
              writable = false;
            }
          ];
          bootstrap = {
            flake = toString ../.;
            # nixosConfigurations.ed-deploy is the nixOS system with an
            # environment for the app.
            attr = "ed-deploy";
          };
        };
      }
    ];
  };
in
{
  den.aspects.ed-deploy = {
    includes = [ den.aspects.ed-env ];
    port = 15625;
  };

  den.hosts.aarch64-linux.ed-deploy = { };

  perSystem =
    { pkgs, ... }:
    {
      packages =
        let
          guestPkgs = inputs.limavm.lib.mkGuestPackages {
            inherit pkgs;
            inherit (local.config.lima) arch image;

            name = "ed-deploy";
            settings = local.config.system.build.limaSettings;
          };
        in
        {
          ed-lima = guestPkgs.start;
          ed-lima-yaml = guestPkgs.yaml;
        };
    };
}
