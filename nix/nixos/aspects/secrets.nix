{ inputs, ... }:
{
  den.aspects.secrets.nixos =
    { config, ... }:
    {
      imports = [
        inputs.sops.nixosModules.sops
      ];

      sops = {
        defaultSopsFile = ./ed.secrets.yaml;
        age.keyFile = "/var/lib/sops-nix/age/keys.txt";
        age.generateKey = false;
        validateSopsFiles = true;
        secrets = {
          "hello" = { };
          "anthropic_api_key" = { };
          "openai_api_key" = { };
          "cloudflared" = {
            path = "/run/secrets/cloudflared";
            owner = "ed";
            group = "users";
            mode = "0400";
          };
          "ed-server" = {
            path = "/run/secrets/ed-server";
            owner = "ed";
            mode = "0400";
            restartUnits = [ "ed-server.service" ];
          };
        };
        templates."ed-server".content = ''
          ANTHROPIC_API_KEY=${config.sops.placeholder.anthropic_api_key}
          OPENAI_API_KEY=${config.sops.placeholder.openai_api_key}
        '';
      };
    };
}
