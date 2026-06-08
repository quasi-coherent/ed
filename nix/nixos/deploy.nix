{ den, inputs, ... }:
{
  den.aspects.ed-deploy = {
    includes = [
      den.aspects.ed-env
      (den.batteries.toLima {
        image = "${inputs.limavm.packages.aarch64-linux.lima-base-image}/nixos.qcow2";
        cpus = 4;
        memory = "8GiB";
        vmType = "vz";
        portForwards = [
          {
            guestPort = 15625;
            hostPort = 8080;
          }
        ];
        mounts = [
          {
            location = toString ../..;
            writable = false;
          }
          {
            location = "~/.config/sops/age/lima-keys.txt";
            writable = false;
          }
        ];
        bootstrap = {
          flake = toString ../..;
          attr = "ed-deploy";
        };
      })
    ];
    port = 15625;
  };

  den.hosts.aarch64-linux.ed-deploy = { };
}
