{ lib, ... }:
{

  den.aspects.storage.nixos =
    { config, self', ... }:
    {
      services.postgresql = {
        enable = true;
        extensions = ps: [ ps.pgvector ];
        enableTCPIP = true;
        ensureDatabases = [ "edapp" ];
        settings.ssl = true;
        authentication = lib.mkOverride 10 ''
          #type database  DBuser  auth-method
          local all       all     trust
          host  sameuser    all     127.0.0.1/32 scram-sha-256
          host  sameuser    all     ::1/128 scram-sha-256
        '';
        ensureUsers = [
          {
            name = "edapp";
            ensureDBOwnership = true;
            ensureClauses = {
              login = true;
              password = "SCRAM-SHA256$4096:CpmwbuywcTbo9bv+v4H77g==$+Ld57Z7xbCSHyNlPQR0+0irxlYvo+zVHCIQ4p3YMeNU=:MRNW3u4GDuGlOhsscWoYd3bJl/9Kwq5ttn6u4tkRi4E=";
            };
          }
        ];
        settings = {
          log_connections = true;
          log_statement = "all";
          log_destination = lib.mkForce "syslog";
        };
      };

      systemd.services.ed-migratedb = {
        description = "ed-migratedb";
        wantedBy = [ "multi-user.target" ];
        after = [ "${config.systemd.services.postgresql.name}" ];
        requires = [ "${config.systemd.services.postgresql.name}" ];
        serviceConfig = {
          Type = "oneshot";
          RemainAfterExit = true;
          ExecStart = "${lib.getExe self'.packages.ed-migratedb}";
          Environment = "DATABASE_URL=${config.sops.templates."DATABASE_URL".path}";
        };
      };
    };
}
