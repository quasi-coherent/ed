{ config, ... }:
{
  sops = {
    defaultSopsFile = ./ed.secrets.yaml;
    age.keyFile = "/var/lib/sops-nix/age/keys.txt";
    age.generateKey = false;
    validateSopsFiles = true;
    secrets = {
      "hello" = { };
      "anthropic_api_key" = { };
      "openai_api_key" = { };
    };
    templates = {
      "ed-serve" = {
        path = "/run/secrets/ed-serve";
        owner = config.services.ed.user;
        mode = "0400";
        restartUnits = [ "ed-serve.service" ];
        content = ''
          ANTHROPIC_API_KEY=${config.sops.placeholder.anthropic_api_key}
          OPENAI_API_KEY=${config.sops.placeholder.openai_api_key}
        '';
      };
    };
  };
}
