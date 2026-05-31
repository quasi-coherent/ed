# Tone & Style Simulator

The app is a Rust axum backend and a React frontend written in ReasonML.  The frontend is served
statically by a service routed to by axum.  The public API is documented in [openapi] format.

The main components are:

* *React app*: UI for ingesting a corpus, forwarding simulation results, and managing previous
inputs/outputs/scoring metrics.
* *Fingerprint calculator*: The evaluation metric -- initial uploads are processed and measured
  against numerous dimensions of "style" and "tone" like frequency of emoji use, exclamation
  points, punctuation, certain formal or informal keywords, sentence length, and more.
* *API service*: Rust clients for interacting with external APIs:
  - A processed message gets sent to the OpenAI embeddings API and the embedding vector in the
    response is persisted in PostgreSQL.
  - A simulation request is handled by calculating cosine distance from previous examples and
    forwarding the most similar in a prompt to the Anthropic messages API, storing and returning
    the result.
* *Request router/handler* The Rust axum app implementing the API.

## Architecture

The main flow can be visualized as:

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

### Approach

As someone who is relatively uneducated about what lower level parts would go into a solution
to the challenge, I relied on AI for idea generation in architecture and how to implement the
different parts of one, and we settled on the above with one unilateral human override: Claude
wanted to use Chroma--a vector search DB in the cloud--for preparing the prompt to Anthropic's
API.  I've used the PostgreSQL extension pgvector before and suspected it would be capable, so
rather than introducing a third bespoke API client and possible paywall, I chose to go with the
traditional.

#### Scoring

The "style fingerprint" scoring metric is the part with the biggest question mark in my mind.
It's entirely heuristic-based, and the samples we got in limited time did not clearly support
the claim that it's conclusive.  The challenge I see is my lack of imagination: I don't
immediately see some way to improve or add to it by way of more or less dimensions.

I think the very next step towards improving the UX would be to combine this purely numerical
measurement with human input: add a UI component for submitting a subjective rating of the
accuracy.  The "fingerprint" calculated off the Anthropic API-generated message, plus the user's
accuracy feedback, could be used to weight the dimensions based on which are most correlated
with higher human ratings.

### Considerations

The results can be nonsensical when the corpus is small, so the best we got here is providing
a warning when under a certain corpus size.  There could be more nuanced approaches where data
is synthesized with real human input to get to a minimum corpus size, and where gradually less
of the input is artificial.  I don't know if this would work, but it's an idea.

Distinguishing the authentic language of the author from the subject or topic they're writing
on is also something we didn't clearly explore.  The initial implementation excluded parts of
a corpus that was quoted, but the result was a multiple of complexity in implementation that I
decided was not acceptable.

One satisfying answer is that the solution should be able to scale for quite some time.  The
size of the corpus is not the driving factor here: on ingest, the messages are represented as
the embedding vector, and the performance of scoring/retrieval is bound to the database.
Improving it would amount to tuning a query, index, table structure, or resource granted to the
database server.  Other factors in scale are out of our control: the amount of data in an HTTP
request, either to us or to OpenAI for the embedding vector.

#### Security

There are certainly privacy and security concerns.  On a shortlist of next steps would be to harden
the app in these ways:

* Authentication at the front door: quick and easy would be OAuth2 via GitHub or Google as the
  provider.  This gets us the user's ID in exchange for the session, which would let us do the
  next things.
* Row-level ownership: We've got the `WHERE user_id = $1` but an extra defense that costs nearly
  nothing is a postgres feature:

```sql
ALTER TABLE ed_api.corpora ENABLE ROW LEVEL SECURITY;
CREATE POLICY user_id_pol ON ed_api.corpora
  USING (user_id = current_setting('ed_api.user_id')::uuid);
```

The Rust database client can be configured to prepend

```sql
set_config('ed_api.user_id', <user_id from JWT>, false);
```

which lasts from when the connection is acquired until it's released back to the pool.

Encryption-at-rest for the data is a bit more tricky when OAuth2 is the mechanism, since there are
fewer options that stably identify/authorize.  A decent approach would be to store a strong private
key per user that is encrypted by a master key, which is fetched only when handling requests.  On
insert and retrieval, the master key decrypts the user's key and it encrypts/decrypts the payloads
on-demand.

There are also really interesting, but more theoretical, possibilities.  It'd be fun to explore
possible applications of [homomorphic encryption], for instance.

### Spike

The personal "spike" is kind of a cheat.  I've been a full-time nix user for almost a decade, so nix
boilerplate is the common preamble.  This was more elaborate because of the frontend bit, but I tried
to add some more substantial components.

First, the ordinary parts:

```console
> $ nix flake show

...snip...

└───packages
    ├───aarch64-darwin
    │   ├───ed-api: package 'ed-api-0.1.0'
    │   ├───ed-lima: package 'ed-lima' - 'limactl wrapper pinned to the ed nixos-lima config'
    │   ├───ed-migratedb: package 'ed-migratedb-0.1.0'
    │   ├───ed-server: package 'ed-server-0.1.0'
    │   ├───frontend: package 'ed-frontend-0.1.0'
    │   ├───openapiCodegen: package 'openapi-gen'
    │   ├───openapiYaml: package 'openapi.yaml'
    │   └───sqlx-prepare: package 'sqlx-prepare'
...
...
```
* The frontend, built on its own in `frontend` creates JS/etc.  This is input to the package `ed-server`
  which is the complete app: the reason-react app transpiled to JS, the Rust server compiled to binary, the
  former placed where the latter needs it to be at runtime.
  - This can be built and ran with one command.  That's true on any operating system since this "flake" is
    defined generically over all systems.
* The `ed-api` package on its own is not very useful, but it represents the complete set of Cargo workspace
  dependencies.
  - This can be cached in [cachix] and then build instantly for anything that depends on it for anyone with
    access to the particular cache(s) it goes to.
* The openapi is in the nix "store" and is an input to the main dependencies, so it is not possible to create
  a situation where things are out of sync.

The "step further" is this part of the `nix flake show` output:

```console
...
├───nixosConfigurations
│   ├───ed-host: NixOS configuration
│   ├───ed-lima-aarch64: NixOS configuration
│   └───ed-lima-x86_64: NixOS configuration
...
```

A `nixosConfigurations` is an entire linux system, unique up to the set of unique dependencies.  It can be deployed
from scratch, it can be rebuilt where only the "diff" is the thing actually built, or it can be reverted to any
previous version just as easily.

The `nixosConfigurations` config `ed-host` is meant to be a "production-ish" target, a definition for a minimal
system that has the app running inside.  This isn't quite complete since I couldn't find a place to host this setup
in time.  The other two define the same for a VM via [Lima] for an ARM or x86_64 host.

Here "full stack" means:

* The backend+frontend is defined in the `nixosConfigurations` as a systemd unit.
* The VM versions include the configuration, users, extensions, networking, etc. for a real PostgreSQL database.
  - The backend+frontend points to _a_ database, this one in the VM setting, but either way depends on a
    systemd oneshot that runs database migrations on the same database.

The app also deals with API keys and (in a theoretical future) cryptographic secrets for securing inputs/outputs.

* Secrets are managed via SOPS: encrypted, can only be decrypted by a process that has an age key that it was
  encrypted with.
  - If the process that decrypts the secret handles it in a safe way, i.e., storing in-app with some utility
    like `libsodium` for memory-protection, there is no point in time where the secret exists in a decrypted
    form.
* Works exactly the same in both VM/cloud scenarios.

In the end, it's as close as you can get to local/production parity, and I envisioned it being convenient in
this case to "write once, deploy twice" even if that didn't quite come true.  Anyway, I really like nix, it's
cool.

[openapi]: ../api/openapi.yaml
[homomorphic encryption]: https://en.wikipedia.org/wiki/Homomorphic_encryption
[cachix]: https://docs.cachix.org
[Lima]: https://github.com/lima-vm/lima
