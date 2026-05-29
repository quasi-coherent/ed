{
  crane,
  pname,
  src,
  version,
}:
# The goal of this derivation is to include everything in the workspace
# dependencies.  That way everything that is needed to build crates here
# gets hashed and added to the nix store, which you combine with a caching
# solution like cachix and the result is instant builds for anyone who can
# pull from the cache.
crane.buildDepsOnly {
  inherit src pname version;
  strictDeps = true;
}
