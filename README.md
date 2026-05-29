# `ed`

> A tone and style simulator application.

## Overview

This repository houses a Rust backend, spread over several [crates](./crates), and a ReasonML
[frontend](./frontend) that uses bindings to React and builds static assets served by the axum app.

The openapi specification can be found [here](./api/openapi.yaml).

It's also nix-ified (mostly), so all of what you see is a [nix flake](./nix) output and should build on any
operating system, as long as you have nix installed, which I guess is maybe not all that likely.

#### Spike

I suppose the whole nix flake of it would be my spike, but it's not quite complete... My goal was to
define `nixosConfigurations` (a linux distribution as a nix derivation), which would be the local dev
environment via something called [lima](https://lima-vm.io/docs/), but also the exact same configuration
would define the "production" environment.

There's also the suggestion in not-quite-complete nix configs that this would manage storing API keys and
database passwords, owned by nix as well, using a framework called sops that I personally use for all my local
secret storage needs.  Since a nix flake is a pure function, the secrets have to be checked into version control
in some form or fashion, and sops does this by storing a public fingerprint of them alongside a public age key
with some scope for what secrets it can decrypt (which happens ephemerally so no vars sourced from env stick around).

One advantage of this is that the age decryption keys can be distributed, scoped by user.  CD steps can pull from a
real thing (AWS SecretsManager or such) and populate the public version and deploy.

In the end what was supposed to happen is that the same config and nearly the same command deploys the
app+runtime dependencies+infra dependencies to both a VM locally and the remote system for a production environment.
This was also going to include an OpenTelemetry collector defined in the nixos config, which I'd be able to see at
some endpoint I'd exposed for myself, but boy did I not get to that.

Really I was unable to find a place that hosts and supports this pattern. Started using personal AWS account; got too late.

### Architecture

As someone who is pretty much brand new to what's _actually_ happening start-to-finish here, I leaned on Claude to
guide me through what's going on and what a full flow might look like.  They wanted a cloud solution for the vector DB,
but I thought that was just one random API to learn too far.  So I chose postgres and the pgvector extension, which can
calculate the cosine similarity you need to find the most alike parts of the corpus to turn around and give to an LLM.
This was pretty much the only deviation from Claude's higher level vision.

The flow ended up being:

```
   Ingest (POST /data/{user_id})
   ─────────────────────────────
        corpus text
             │
             ▼
   ┌───────────────────┐      ┌─────────────────────┐
   │  ed-axum (API)    │─────▶│  OpenAI embeddings  │
   └───────────────────┘      └─────────────────────┘
             │                          │
             │   chunks + vectors  ◀────┘
             ▼
   ┌───────────────────┐
   │ postgres+pgvector │
   └───────────────────┘


   Simulate (POST /simulate/{user_id})
   ───────────────────────────────────
        prompt
          │
          ▼
   ┌───────────────────┐      ┌─────────────────────┐
   │  ed-axum (API)    │─────▶│  OpenAI embeddings  │ (embed prompt)
   └───────────────────┘      └─────────────────────┘
          │   ▲                       │
          │   │ top-k chunks   ◀──────┘
          ▼   │  (cosine sim)
   ┌───────────────────┐
   │ postgres+pgvector │
   └───────────────────┘
          │
          │  prompt + retrieved style exemplars
          ▼
   ┌───────────────────┐
   │   Claude (LLM)    │──▶ generated text in the user's "voice"
   └───────────────────┘
```

#### Style "fingerprint"

The calculation we relied on is this so-called "fingerprint" which accumulates ratios, counts, and running
"gauges" of a number of dimensions: sentence length, frequency of certain punctuation, and so on.

It was hard to define an invariant for what style and tone even is.  Early iterations were either
totally uninspiring and/or bad.  I think this is still my biggest open question--how to evaluate this confidently.
Calculating it on the generated response from the Anthropic message API and comparing with the input examples
(the closest in cosine distance) seems like the most natural but I wasn't able to get a strong indicator of how
well it tracked in the time allotted.

### Etymology

The purpose of the app is mimicry of human language.  Exotic birds are sometimes known for that.
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

[cockatoo]: https://en.wikipedia.org/wiki/White_cockatoo
