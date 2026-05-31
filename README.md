# `ed`

> A tone and style simulator application.

An [API](./api/openapi.yaml) for uploading a corpus and generating text in the "tone" and "style"
extracted from it.  The API is [implemented](./crates/rust-axum) in Rust using `axum` and has a
[frontend](./frontend) UI served statically from the `axum` service, written in ReasonML.

The write-up of the project is [here](./docs/writeup.md).

## Usage

The default devShell provides a way to deploy the app locally in a VM that is not complicated if you
already have nix.

First run

```console
> $ ed-lima start
```
to start the VM.  Next, the app needs developer API keys to function.  Run

```console
> $ ed-lima source --env OPENAI_API_KEY --env ANTHROPIC_API_KEY
```

to source these from your environment, then direct your browser to [localhost:8080](https://localhost:8080).
See the help for other commands: `ed-lima --help`.

If without nix, but with desire to run greater than desire to not have nix even temporarily, get a nix
[installer][lix], follow those instructions, then go up and do these instructions.

Afterwards, you have two options: You run `/nix/lix-installer uninstall` -- the story ends, you wake up in
your bed and believe whatever you want to believe.  You run `nix run nixpkgs#nix-tour` -- you stay in
Wonderland and see how deep the rabbit hole goes.  Remember: all that's offered is the truth, nothing  more.

## Etymology

The purpose of the app is mimicry of written human language.  Exotic birds are sometimes known for that.
When I was 7 years old, my parents bought me an [umbrella cockatoo][cockatoo], against all better
judgement.  This is a very serious bird for a 7 year old.  I still can't believe my parents did this.

I named him Ed.  Ed was my best friend.  Ed loved to talk to me and anyone else.  Of course, he was
just "parroting" things he'd heard, which meant he would say "I love you" a lot.

Ed was the best trained bird.  He didn't stay in a cage most of the time because most of the time he'd
be on my shoulder.  Often I'd be out in the back yard and he'd be watching from a perch I'd made for him.
One day I went inside briefly and when I came back he was gone.

I was devastated; I still am.  Ten thousand times before he'd be there pacing back and forth until he
saw me emerge from inside.  He'd settle down, tell me he loves me, and then go back to being a bird on
a perch.

I had him for 4 years.  Umbrella cockatoos can live well into their 70s.  I would like to think he's still
out there somewhere in the Seattle suburbs.  Of course I know that's not true.

Wherever he is though, I hope he knows I love him back.

[lix]: https://lix.systems/install
[cockatoo]: https://en.wikipedia.org/wiki/White_cockatoo
