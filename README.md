# `ed`

> A tone and style simulator application.

An [API](./api/openapi.yaml) for uploading a corpus and generating text in the
"tone" and "style" extracted from it.  The API is
[implemented](./crates/rust-axum) in Rust using `axum` and has a
[frontend](./frontend) UI served statically from the `axum` service, written in
ReasonML.

The write-up of the project is [here](./docs/writeup.md).

## Usage

If you have nix

```nix
> $ nix build .#ed-deploy
```

builds the `nixosSystem` derivation defined [here][ed-deploy] having the app and
environment to run it.  This also outputs a package attribute that exposes a CLI
to create and interact with this NixOS system as a VM (managed by [Lima]):

```nix
> $ ./result/bin/ed-deploy-lima --help
```

The `start` subcommand starts the boot procedure.  For the app to function from
within the VM, there's an additional requirement of developer API keys for the
Anthropic and OpenAI APIs.  For security, these are stored and accessed using
[sops.nix] for secret provisioning.  The VM boot sequence mounts a path on the
host that presumably has the age key to decrypt these at startup:

```nix
> $ mkdir -p ~/.config/sops/age/lima
> $ nix run nixpkgs#age -- age-keygen -o ~/.config/sops/age/lima/keys.txt
> $ # This outputs the public age key; add it to ./.sops.yaml like the others.
```

If without nix, but with desire to run greater than desire to not have nix even
temporarily, get a nix [installer][lix], follow those instructions, then go up
and do these instructions.

Afterwards, you have two options: You run `/nix/lix-installer uninstall` -- the
story ends, you wake up in your bed and believe whatever you want to believe.
You run `nix run nixpkgs#nix-tour` -- you stay in Wonderland and see how deep
the rabbit hole goes.  Remember: all that's offered is the truth, nothing more.

## Etymology

The purpose of the app is mimicry of human language.  Exotic birds are sometimes
known for that.  When I was 7 years old, my parents bought me an [umbrella
cockatoo][cockatoo], against all better judgement.  This is a very serious bird
for a 7 year old.  I still can't believe my parents did this.

I named him Ed.  Ed was my best friend.  Ed loved to talk to me and anyone else.
Of course, he was just "parroting" things he'd heard, which meant he would say
"I love you" a lot.

Ed was the best trained bird.  He didn't stay in a cage most of the time because
most of the time he'd be on my shoulder.  Often I'd be out in the back yard and
he'd be watching from a perch I'd made for him.  One day I went inside briefly
and when I came back he was gone.

I was devastated; I still am.  Ten thousand times before he'd be there pacing
back and forth until he saw me emerge from inside.  He'd settle down, tell me he
loves me, and then go back to being a bird on a perch.

I had him for 4 years.  Umbrella cockatoos can live well into their 70s.  I
would like to think he's still out there somewhere in the Seattle suburbs.  Of
course I know that's not true.

Wherever he is though, I hope he knows I love him back.

[ed-deploy]: ./nix/nixos/default.nix
[Lima]: https://github.com/lima-vm/lima
[sops.nix]: https://github.com/mic92/sops-nix
[lix]: https://lix.systems/install
[cockatoo]: https://en.wikipedia.org/wiki/White_cockatoo
