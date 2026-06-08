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
* *Authorization*: OAuth2 client and router handling the authorization flow.
* *Authorized request router/handler*: The Rust axum app implementing the API for authorized users.
* *Infrastructure*: The "spike" described [below](#spike).

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

There are unavoidable privacy and security questions.  The app here is hardened in these ways:

* A user must be authorized.  This is done with OAuth2 and Google as the provider.  This gives
  us a `user_id` we can save and use in the next thing.
* Row-level ownership: We've got the `WHERE user_id = $1` but an extra defense that costs nearly
  nothing is a postgres feature:

```sql
ALTER TABLE ed_api.corpora ENABLE ROW LEVEL SECURITY;
CREATE POLICY user_id_pol ON ed_api.corpora
  USING (user_id = current_setting('ed_api.user_id')::uuid);
```

The Rust database client prepends

```sql
set_config('ed_api.user_id', <user_id from JWT>, false);
```

which lasts until a connection is yielded back to the pool.

On the shortlist of next steps: encryption-at-rest.  One possible implementation would be to store
a strong private key per user that is encrypted by a master key, which is fetched only when handling
requests.  On insert and retrieval, the master key decrypts the user's key, which in turn encrypts or
decrypts a payload.

In the end, there shouldn't be a point in time where a private key exists in a decrypted form, nor
user data, since the frontend is served by the backend.  If the frontend were deployed separately, we
would use service-runtime access tokens for authorization/authentication within the "interior."

There are also really interesting possibilities that are more theoretical.  It'd be fun to explore
possible applications of [homomorphic encryption], for instance.

### Spike

The personal "spike" lives at the stage where the app would be deployed.  I've been a full-time nix
user for almost a decade, so nix boilerplate is the common preamble.  This project was more elaborate
because of the frontend bit, but I wanted to build on that to encompass how and where something runs--
a "fuller stack" application if you will--which is a significant part of the SDLC that's often
"someone else's job."  I happen to think it should be anyone's job but the developer.

The following command shows which "packages" the repo outputs:

```console
> $ nix flake show

...snip...

└───packages
    ├───aarch64-darwin
    │   ├───default: package 'fmtt'
    │   ├───ed-app: package 'ed-app'
    │   ├───ed-deploy: package 'ed-deploy-lima'
    │   ├───ed-deploy-yaml: package 'ed-deploy-lima.yaml'
    │   ├───ed-migratedb: package 'ed-migratedb-0.1.0'
    │   ├───openapi-gen: package 'openapi-gen'
    │   └───sqlx-prepare: package 'sqlx-prepare'
    ├───aarch64-linux
    │   ├───default omitted (use '--all-systems' to show)
    │   ├───ed-app omitted (use '--all-systems' to show)
    │   ├───ed-deploy omitted (use '--all-systems' to show)
    │   ├───ed-deploy-yaml omitted (use '--all-systems' to show)
    │   ├───ed-migratedb omitted (use '--all-systems' to show)
    │   ├───openapi-gen omitted (use '--all-systems' to show)
    │   └───sqlx-prepare omitted (use '--all-systems' to show)
    ├───x86_64-darwin
    │   ├───default omitted (use '--all-systems' to show)
    │   ├───ed-app omitted (use '--all-systems' to show)
    │   ├───ed-deploy omitted (use '--all-systems' to show)
    │   ├───ed-deploy-yaml omitted (use '--all-systems' to show)
    │   ├───ed-migratedb omitted (use '--all-systems' to show)
    │   ├───openapi-gen omitted (use '--all-systems' to show)
    │   └───sqlx-prepare omitted (use '--all-systems' to show)
    └───x86_64-linux
        ├───default omitted (use '--all-systems' to show)
        ├───ed-app omitted (use '--all-systems' to show)
        ├───ed-deploy omitted (use '--all-systems' to show)
        ├───ed-deploy-yaml omitted (use '--all-systems' to show)
        ├───ed-migratedb omitted (use '--all-systems' to show)
        ├───openapi-gen omitted (use '--all-systems' to show)
        └───sqlx-prepare omitted (use '--all-systems' to show)
```

A brief definition of these:
* This "flake" is defined generically over the OS, so it would build on any of those listed.  My local
  machine is an Apple Silicon laptop, so the system it specializes to in my case is `aarch64-darwin`.
* The `ed-app` output is the startup [script](../nix/packages/ed-app.nix) for the combined BE/FE.  For the
  fact that it includes both, building it involves building both.
  - Building the Rust portion has an intermediate derivation that represents the complete set of workspace
    dependencies.  This is an intentional "checkpoint" that in practice you could take advantage of with a
    nix binary cache like [cachix]. The pre-built version of not just the compiled Rust dependencies, but
    _anything_ declared here, would be available to anyone with access.
  - For Rust in particular, this is often a huge deal (this giant Cargo.toml builds in about 0.5 seconds).
* Utility packages: `sqlx-prepare` and `openapi-gen` are for certain workflows that need you to "update" or
  "generate" something: changes to the OpenAPI spec will lead to failure without running codegen; same
  with `sqlx-prepare` and changes to the "query library" found in [`ed-db`](../crates/ed-db/src/query.rs).
* The service relies on a database, so it relies on a migration tool to manage the state of the DB over time.
  The `ed-migratedb` package is a self-contained migration runner tracking [these](../ed-migratedb/src/migrations)
  migrations.
  - This runs as a systemd service in the deployment that depends on the postgres service and is depended on by
    the application.
  - This also makes use of the crate [tern](https://crates.io/crates/tern), which I'm the creator/maintainer of.
    I think it's pretty slick and also can do really useful things I haven't found anywhere else.

The other two packages, `ed-deploy` and `ed-deploy-yaml`, come from the dependency (a.k.a. "input") [limavm.nix].
This is also a personal project.  This one didn't exist until now.  Can never have too many.  In looking for a way
to encapsulate a full deployment in this repo, I found that it was difficult to both create a NixOS system
(Linux distribution defined in nix expression) for a real deployment and also a local developer environment,
especially for MacOS users.

The result is the `limavm.nix` flake, which exposes functionality for taking an existing `nixosSystem`
configuration and constructing a bootable VM from it that can work on any host system.  This means that
"production" and "local" nearly coincide ("nearly" because there's some part that's specific to a VM that wouldn't
apply to a bare-metal deployment).  For convenience, these outputs exist:

* `ed-deploy`: Wraps [`limactl`](https://lima-vm.io/docs/reference/limactl/) specialized to the system named
  `ed-deploy`.  This can start/stop/interact with the running VM in general.
* `ed-yaml`: Constructs the configuration that is the input for `limactl`.  Modular in that you can pass any base
  image to it.

The `ed-deploy` __system_ is an output as well:

```nix
> $ nix flake show
...snip...
├───nixosConfigurations
│   └───ed-deploy: NixOS configuration
...snip...
```

This declares a few things:

* [Application](../nix/nixos/aspects/application.nix): The `ed-app` package executed as the command for a
  systemd service, so this is the running app.
* [Storage](../nix/nixos/aspects/storage.nix): The storage layer for the app.  Declares two more
  systemd services, one a complete postgresql instance and another for the migrations.
* [Secrets](../nix/nixos/aspects/secrets.nix): The app needs a number of secrets at startup.  Rather
  than providing them in plaintext as environment variables, this defines them as encrypted objects in
  the nix store, which are at most ephemeral in their decrypted form only for a process that owns the
  key they were encrypted with.

Anyway, I really like nix, it's cool.

[openapi]: ../api/openapi.yaml
[homomorphic encryption]: https://en.wikipedia.org/wiki/Homomorphic_encryption
[cachix]: https://docs.cachix.org
[Lima]: https://github.com/lima-vm/lima
[limavm.nix]: https://github.com/quasi-coherent/limavm.nix
