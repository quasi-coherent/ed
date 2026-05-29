{ config, ... }:
{
  sops = {
    defaultSopsFile = ./secrets.yaml;
    age.keyFile = "${config.xdg.configHome}/sops/age/key.txt";
    age.sshKeyPaths = [ ];
    age.generateKey = false;
    validateSopsFiles = true;
    secrets = {
      "hello" = { };
      "anthropic_api_key" = { };
      "openai_api_key" = { };
      "ed-serve" = {
        owner = config.services.ed.user;
        mode = "0400";
        # restartUnits = [ "ed-serve.service" ];
      };
    };
    templates = { };
  };
}
