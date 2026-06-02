{ ... }:
{
  den.aspects.routable.nixos =
    { config, ... }:
    {
      services.cloudflared = {
        enable = true;
        tunnels."ed-host" = {
          credentialsFile = "/run/secrets/cloudflare-tunnel";
          ingress = [
            {
              hostname = "????";
              service = "http://localhost:${config.port}";
            }
            { service = "http_status:404"; }
          ];
        };
      };
    };
}
