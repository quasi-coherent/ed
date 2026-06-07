{
  den,
  lib,
  ...
}:
{
  # BE+FE
  den.aspects.application.nixos =
    { config, self', ... }:
    let
      svc = config.systemd.services;
    in
    {
      environment.sessionVariables.APP_PORT = "${toString den.aspects.ed-env.port}";

      systemd.services.ed-server = {
        description = "ed-server";
        wantedBy = [ "multi-user.target" ];

        after = [
          "${svc.postgresql.name}"
          "${svc.ed-migratedb.name}"
          "network.target"
        ];
        requires = [
          "${svc.postgresql.name}"
          "${svc.ed-migratedb.name}"
        ];

        serviceConfig = {
          Restart = "on-failure";
          RestartSec = 5;
          ExecStart = "${lib.getExe self'.packages.ed-app}";
          # hardening
          NoNewPrivileges = true;
          ProtectSystem = "strict";
          ProtectHome = true;
          PrivateTmp = true;
          PrivateDevices = true;
          ProtectKernelTunables = true;
          ProtectKernelModules = true;
          ProtectControlGroups = true;
          RestrictAddressFamilies = [
            "AF_INET"
            "AF_INET6"
            "AF_UNIX"
          ];
          LockPersonality = true;
        };
      };
    };
}
