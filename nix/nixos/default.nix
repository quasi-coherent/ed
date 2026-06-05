{
  den,
  lib,
  ...
}:
{
  imports = [
    ./aspects/application.nix
    ./aspects/secrets.nix
    ./aspects/storage.nix
  ];

  # A deployment of the `ed` app.
  den.aspects.ed-deploy =
    {
      # deadnix: skip
      config,
      ...
    }:
    {
      includes = [
        den.aspects.application
        den.aspects.storage
        den.aspects.secrets

        (den.batteries.toLima {
          cpus = 3;
          memory = "10GiB";
          vmType = "qemu";
          portForwards = [
            { guestPort = den.aspects.ed-deploy.port; }
          ];
        })
      ];

      imports = [
        {
          options = with lib; {
            port = mkOption {
              type = types.int;
              default = 15625;
            };
          };
        }
      ];
    };

  den.hosts.aarch64-linux.ed-deploy = { };

  den.default = {
    nixos.system.stateVersion = lib.mkDefault "26.05";
    includes = [ den.batteries.self' ];
  };
}
