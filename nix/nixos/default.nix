{ den, ... }:
{
  den = {
    default.includes = [
      den.batteries.self'
    ];
    # schema.includes = [ den.batteries.hostname ];

    hosts.aarch64-linux.ed-deploy = { };
    hosts.aarch64-linux.ed-local = { };
  };
}
