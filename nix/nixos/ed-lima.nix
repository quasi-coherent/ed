{
  lib,
  modulesPath,
  pkgs,
  ...
}:
{
  imports = [
    (modulesPath + "/profiles/qemu-guest.nix")
  ];

  networking.hostName = lib.mkDefault "ed-lima";

  services.lima.enable = true;

  nix.settings.experimental-features = [
    "nix-command"
    "flakes"
  ];
  nix.settings.trusted-users = [ "@wheel" ];

  services.openssh.enable = true;
  security.sudo.wheelNeedsPassword = false;

  # Lima exposes :80 to the host via portForwards; no need to widen
  # the guest firewall beyond what `services.ed.openFirewall` does.
  services.ed.hostName = lib.mkDefault "ed-lima";

  boot = {
    kernelParams = [ "console=tty0" ];
    kernelPackages = pkgs.linuxPackages_latest;
    loader.grub = {
      device = "nodev";
      efiSupport = true;
      efiInstallAsRemovable = true;
    };
  };
  fileSystems."/boot" = {
    device = lib.mkForce "/dev/vda1";
    fsType = "vfat";
  };
  fileSystems."/" = {
    device = "/dev/disk/by-label/nixos";
    autoResize = true;
    fsType = "ext4";
    options = [
      "noatime"
      "nodiratime"
      "discard"
    ];
  };

  environment.systemPackages = with pkgs; [
    emacs30
    sops
    age
    git
  ];

  system.stateVersion = "25.11";
}
