{ inputs, ... }:
{
  den.aspects.secrets.nixos =
    { config, ... }:
    {
      imports = [ inputs.sops.nixosModules.sops ];

      sops = {
        defaultSopsFile = ../../../secrets/guest.yaml;
        age.keyFile = "~/.config/sops/age/lima/keys.txt";
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
          "DATABASE_URL" = {
            restartUnits = [ "postgresql.service" ];
            content = ''
              DATABASE_URL="postgresql://edapp:${config.sops.placeholder.pg_pass}@localhost:5432/edapp?sslmode=require"
            '';
          };
        };
      };
    };
}
