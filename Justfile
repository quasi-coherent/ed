system := `nix-instantiate --raw --strict --eval -E builtins.currentSystem`

check:
    nix flake check

openapi-gen:
    openapi-gen -g rust-axum -o ./crates/ed-axum
    fmtt

update-rs:
    nix flake update fenix
