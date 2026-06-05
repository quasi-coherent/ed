{ inputs, ... }:
{
  den.aspects.secrets.nixos =
    { config, ... }:
    {
      imports = [ inputs.sops.nixosModules.sops ];

      sops = {
        defaultSopsFile = ../../../secrets/guest.yaml;
        validateSopsFiles = true;
        secrets = {
          "hello" = { };
          "anthropic_api_key" = { };
          "openai_api_key" = { };
          "pg_pass" = { };
          "google_client_secret" = { };
          "google_client_id" = { };
        };
        templates = {
          "DATABASE_URL".content = ''
            DATABASE_URL="postgresql://edapp:${config.sops.placeholder.pg_pass}@localhost:5432/edapp?sslmode=require"
          '';
          "app_env.sh" = {
            path = "/run/secrets/app_env.sh";
            mode = "0644";
            restartUnits = [ "ed-server.service" ];
            content = ''
              DATABASE_URL="postgresql://edapp:${config.sops.placeholder.pg_pass}@localhost:5432/edapp?sslmode=require"
              ANTHROPIC_API_KEY="${config.sops.placeholder.anthropic_api_key}"
              OPENAI_API_KEY="${config.sops.placeholder.openai_api_key}"
            '';
          };
        };
      };
    };
}
