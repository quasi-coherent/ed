{
  config,
  lib,
  ed-migratedb,
  ed-server,
  ...
}:
let
  cfg = config.services.ed;
in
{
  options.services.ed = {
    enable = lib.mkEnableOption "ed-server (axum API + bundled frontend)";

    user = lib.mkOption {
      type = lib.types.str;
      default = "ed";
      description = "Unix user that runs ed-server and owns the postgres role.";
    };

    database = lib.mkOption {
      type = lib.types.str;
      default = "ed";
      description = ''
        Postgres database name. Must equal `user` so that
        services.postgresql.ensureUsers can take ownership.
      '';
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 3000;
      description = "Local port axum binds to (nginx forwards :80 to this).";
    };

    hostName = lib.mkOption {
      type = lib.types.str;
      default = "localhost";
      description = "nginx server_name for the public vhost.";
    };

    openFirewall = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Open port 80 in the firewall.";
    };

    environmentFile = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      example = "/run/secrets/ed-server";
      description = ''
        Path to an EnvironmentFile holding OPENAI_API_KEY and ANTHROPIC_API_KEY.
        Typically the sops secret path: config.sops.secrets."ed-server".path.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.${cfg.user} = {
      isSystemUser = true;
      group = cfg.user;
      description = "ed-server service user";
    };
    users.groups.${cfg.user} = { };

    services.postgresql = {
      enable = true;
      extensions = ps: [ ps.pgvector ];
      ensureDatabases = [ cfg.database ];
      ensureUsers = [
        {
          name = cfg.user;
          ensureDBOwnership = true;
        }
      ];
    };

    systemd.services.ed-migratedb = {
      description = "ed-migratedb";
      wantedBy = [ "multi-user.target" ];
      after = [ "postgresql.service" ];
      requires = [ "postgresql.service" ];
      serviceConfig = {
        Type = "oneshot";
        RemainAfterExit = true;
        User = "postgres";
        ExecStart = "${lib.getExe ed-migratedb}";
        Environment = "DATABASE_URL=postgres:///${cfg.database}?host=/run/postgresql";
      };
    };

    systemd.services.ed-server = {
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
        DATABASE_URL = "postgres:///${cfg.database}?host=/run/postgresql&user=${cfg.user}";
        PORT = toString cfg.port;
      };

      serviceConfig = {
        ExecStart = "${lib.getExe ed-server}";
        User = cfg.user;
        Group = cfg.user;
        Restart = "on-failure";
        RestartSec = 5;
        EnvironmentFile = lib.mkIf (cfg.environmentFile != null) cfg.environmentFile;

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

    services.nginx = {
      enable = true;
      recommendedProxySettings = true;
      recommendedOptimisation = true;
      virtualHosts.${cfg.hostName} = {
        default = true;
        locations."/" = {
          proxyPass = "http://127.0.0.1:${toString cfg.port}";
        };
      };
    };

    networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [ 80 ];
  };
}
