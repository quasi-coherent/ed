{
  # The base host just sets the ed user and adds a few tools.
  den.aspects.base = {
    nixos =
      { pkgs, ... }:
      {
        environment.systemPackages = with pkgs; [
          age
          git
          emacs30
          fd
          jq
          postgresql
          rg
          sops
          ssh
        ];

        users.users.ed = {
          isNormalUser = true;
          extraGroups = [ "wheel" ];
        };

        system.stateVersion = "26.05";
      };
  };
}
