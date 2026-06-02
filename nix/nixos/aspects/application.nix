{ lib, ... }:
{
  # Bundle BE+FE+database.
  den.aspects.application.nixos =
    { self', ... }:
    {
      services.postgresql = {
        enable = true;
        extensions = ps: [ ps.pgvector ];
        ensureDatabases = [ "ed" ];
        ensureUsers = [
          {
            name = "ed";
            ensureDBOwnership = true;
          }
        ];
      };

      systemd.services = {
        ed-migratedb = {
          description = "ed-migratedb";
          wantedBy = [ "multi-user.target" ];
          after = [ "postgresql.service" ];
          requires = [ "postgresql.service" ];
          serviceConfig = {
            Type = "oneshot";
            RemainAfterExit = true;
            User = "postgres";
            ExecStart = "${lib.getExe self'.packages.ed-migratedb}";
            Environment = "DATABASE_URL=postgres:///ed?host=/run/postgresql";
          };
        };

        ed-server = {
          description = "ed-server";
          wantedBy = [ "multi-user.target" ];

          after = [
            "postgresql.service"
            "ed-migratedb.service"
            "network.target"
          ];
          requires = [
            "postgresql.service"
            "ed-migratedb.service"
          ];

          environment = {
            DATABASE_URL = "postgres:///ed?host=/run/postgresql&user=ed";
          };

          serviceConfig = {
            User = "ed";
            Group = "users";
            Restart = "on-failure";
            RestartSec = 5;

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
    };
}
