{ den, ... }:
{
  den.aspects.ed-deploy = {
    includes = [
      den.aspects.application
      den.aspects.secrets
      den.aspects.routable

      (den.batteries.toLima {
        cpus = 4;
        memory = "12GiB";
        vmType = "qemu";
        mounts = [
          {
            location = "~/.config/sops/age/keys";
            writable = false;
          }
        ];
      })
    ];
  };

  # Only difference is no cloudflare tunnel.
  den.aspects.ed-local = {
    includes = [
      den.aspects.application
      den.aspects.secrets

      (den.batteries.toLima {
        cpus = 2;
        memory = "8GiB";
        vmType = "qemu";
        mounts = [
          {
            location = "~/.config/sops/age/keys";
            writable = false;
          }
        ];
      })
    ];
  };
}
