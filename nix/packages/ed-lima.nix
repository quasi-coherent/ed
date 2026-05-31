{
  lima,
  nix,
  writeShellApplication,
  edLimaTopLevel,
}:
let
  limaConfig = ../../lima/nixos.yaml;
  instance = "ed";
in
writeShellApplication {
  name = "ed-lima";
  runtimeInputs = [
    lima
    nix
  ];
  text = ''
    export LIMA_INSTANCE="${instance}"
    config="${limaConfig}"
    toplevel="${edLimaTopLevel}"

    usage() {
      cat <<EOF
    ed-lima — limactl wrapper for the ed nixos-lima VM (instance: ${instance})

    Custom commands:
      start                      Create the VM and activate the baked-in toplevel,
                                 or resume it if it already exists.
      rebuild                    Push the current flake's toplevel to the VM and
                                 switch to it. Re-derive via \`nix build .#ed-lima\`
                                 first to pick up flake edits.
      source --env KEY [...]     Read each KEY from the host env, write
                                 /var/lib/ed/env on the VM, restart ed-server.
      -h, --help, help           Show this message.

    Anything else is forwarded to limactl with LIMA_INSTANCE=${instance}.
    EOF
    }

    push_and_switch() {
      NIX_SSHOPTS="-F $HOME/.lima/${instance}/ssh.config" \
        nix copy --to "ssh-ng://lima-${instance}" "$toplevel"
      limactl shell sudo "$toplevel/bin/switch-to-configuration" switch
    }

    case "''${1:-start}" in
      -h|--help|help)
        usage
        ;;
      start)
        shift || true
        if limactl list --quiet | grep -qx "${instance}"; then
          exec limactl start --tty=false "$@"
        fi
        limactl start --tty=false --name="${instance}" "$config" "$@"
        push_and_switch
        ;;
      rebuild)
        push_and_switch
        ;;
      source)
        shift
        vars=()
        while [ "''${1:-}" = "--env" ]; do
          shift
          if [ $# -lt 1 ]; then
            echo "usage: ed-lima source --env KEY [--env KEY ...]" >&2
            exit 2
          fi
          vars+=("$1")
          shift
        done
        if [ ''${#vars[@]} -eq 0 ]; then
          echo "usage: ed-lima source --env KEY [--env KEY ...]" >&2
          exit 2
        fi
        payload=""
        for v in "''${vars[@]}"; do
          val="''${!v-}"
          if [ -z "$val" ]; then
            echo "ed-lima: \$$v is unset or empty in host environment" >&2
            exit 1
          fi
          payload="$payload$v=$val"$'\n'
        done
        printf '%s' "$payload" | limactl shell sudo install -D \
          -m 0400 -o ed -g ed /dev/stdin /var/lib/ed/env
        exec limactl shell sudo systemctl restart ed-server
        ;;
      *)
        exec limactl "$@"
        ;;
    esac
  '';
  meta = {
    description = "limactl wrapper pinned to the ed nixos-lima VM";
    mainProgram = "ed-lima";
    inherit (lima.meta) platforms;
  };
}
