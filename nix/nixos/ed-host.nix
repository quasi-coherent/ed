{ lib, modulesPath, ... }:
{
  imports = [
    (modulesPath + "/profiles/qemu-guest.nix")
  ];
  # TODO
  networking.hostName = lib.mkDefault "ed-host";

  boot.loader.grub = {
    enable = lib.mkDefault true;
    device = lib.mkDefault "nodev";
    efiSupport = lib.mkDefault true;
    efiInstallAsRemovable = lib.mkDefault true;
  };

  fileSystems."/" = lib.mkDefault {
    device = "/dev/disk/by-label/nixos";
    fsType = "ext4";
  };

  services.openssh.enable = true;

  system.stateVersion = "25.11";
}
