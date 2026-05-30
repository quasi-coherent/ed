{
  lima,
  writeShellApplication,
}:
let
  limaConfig = ../../lima/nixos.yaml;
  instance = "ed";
in
writeShellApplication {
  name = "ed-lima";
  runtimeInputs = [ lima ];
  text = ''
    config="${limaConfig}"
    name="${instance}"

    cmd="''${1:-start}"
    if [ $# -gt 0 ]; then shift; fi

    case "$cmd" in
      start)
        if limactl list --quiet | grep -qx "$name"; then
          exec limactl start "$name" "$@"
        else
          exec limactl start --name="$name" "$config" "$@"
        fi
        ;;
      shell|ssh)
        exec limactl shell "$name" "$@"
        ;;
      install-key)
        if [ $# -lt 1 ]; then
          echo "usage: ed-lima install-key <host-path-to-age-key>" >&2
          exit 2
        fi
        key="$1"
        if [ ! -r "$key" ]; then
          echo "ed-lima: cannot read $key" >&2
          exit 1
        fi
        exec limactl shell "$name" sudo install -D -m 0400 -o root -g root \
          /dev/stdin /var/lib/sops-nix/age/keys.txt < "$key"
        ;;
      rebuild)
        # Pull the flake from the host's repo (mounted read-only at ~) and
        # switch the VM to the ed-lima nixosConfiguration matching its arch.
        exec limactl shell "$name" sudo nixos-rebuild switch \
          --flake "${toString ../..}#ed-lima-$(limactl shell "$name" uname -m | sed 's/arm64/aarch64/')" \
          "$@"
        ;;
      *)
        exec limactl "$cmd" "$@"
        ;;
    esac
  '';
  meta = {
    description = "limactl wrapper pinned to the ed nixos-lima config";
    mainProgram = "ed-lima";
    inherit (lima.meta) platforms;
  };
}
