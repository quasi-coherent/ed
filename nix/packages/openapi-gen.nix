{
  lib,
  openapi-generator-cli,
  openapiYaml,
  writeShellApplication,
}:
writeShellApplication {
  name = "openapi-gen";
  runtimeInputs = [ openapi-generator-cli ];
  text = ''
    export PATH=${lib.getBin openapi-generator-cli}/bin:$PATH
    declare -a args more

    gen=""
    out=""
    template=""

    while test -n "''${1:-}"; do
      first="$1"
      shift
      case "$first" in
        -g|--generator)
          gen="$1"
          shift
        ;;
        --output|-o)
          out="$1"
          shift
        ;;
        --template|-t)
          template="$1"
          shift
        ;;
        *)
          more+=("$first")
        ;;
      esac
    done

    if [[ -z $gen || -z $out ]]; then
      echo "-g/--generator and -o/--output are required"
      exit 1
    fi

    args+=("-g" "$gen" "-i" "${openapiYaml}" "-o" "$out")

    if [[ -n $template ]]; then
      args+=("-t" "$template")
    fi

    args+=("''${more[@]}")

    exec openapi-generator-cli generate "''${args[@]}"
  '';
}
