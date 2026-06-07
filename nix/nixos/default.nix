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
    ./deploy.nix
  ];

  # Environment that can run the app.
  den.aspects.ed-env =
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

  den.default = {
    nixos.system.stateVersion = lib.mkDefault "26.05";
    includes = [ den.batteries.self' ];
  };
}
