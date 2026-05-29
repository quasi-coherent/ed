{
  config,
  lib,
  ed-server,
  ...
}:
let
  cfg = config.services.ed;
in
{
  options.services.ed = {
    enable = lib.mkEnableOption "ed-serve (axum API + bundled frontend)";

    user = lib.mkOption {
      type = lib.types.str;
      default = "ed";
      description = "Unix user that runs ed-serve and owns the postgres role.";
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
      example = "/run/secrets/ed-serve";
      description = ''
        Path to an EnvironmentFile holding OPENAI_API_KEY and ANTHROPIC_API_KEY.
        Typically the sops secret path: config.sops.secrets."ed-serve".path.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.${cfg.user} = {
      isSystemUser = true;
      group = cfg.user;
      description = "ed-serve service user";
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

    # The pgvector extension must be created in the target database before
    # ed-migratedb runs, so the V*__*.sql migrations that reference `vector`
    # can succeed.
    systemd.services.postgresql.postStart = lib.mkAfter ''
      $PSQL -d ${cfg.database} -c 'CREATE EXTENSION IF NOT EXISTS vector;'
    '';

    systemd.services.ed-serve = {
      description = "ed-serve (axum API + frontend)";
      wantedBy = [ "multi-user.target" ];
      after = [
        "postgresql.service"
        "network.target"
      ];
      requires = [ "postgresql.service" ];

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
