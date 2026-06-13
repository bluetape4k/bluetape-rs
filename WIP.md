# WIP

Snapshot: 2026-06-13 KST
Scope: `0.4.0` compression helpers and `0.5.x` serialization/benchmark planning.

## Current Target Release

`v0.4.0` - Compression helpers for backend services.

This release starts the Rust-native compression line and tracks benchmark
parity against `bluetape-go` and `bluetape4k-io` with shared fixtures.

## Current State

- The repository is a Rust 2024 workspace with root facade crate
  `bluetape-rs` plus focused crates for core, logging, test support,
  collections, async helpers, codec helpers, and compression helpers.
- The current release-prep line is `0.4.0`; `0.4.0` work is tracked in GitHub
  milestone `0.4.0` until the release readiness issue and epic are closed.
- Feasibility research exists under
  `docs/research/2026-06-08-backend-library-feasibility.md`.
- GitHub milestone `0.4.0` tracks compression epic #73 and release readiness
  issue #80 after the implementation, documentation, and benchmark child issues
  have been merged.
- Benchmark comparison data is preserved under `docs/benchmark`.
- The next serialization line is split across `0.5.x` milestones, starting with
  cache-first binary payload support in `0.5.0`.
- GitHub milestone `0.5.5` tracks same-environment, same-scenario SerDe
  benchmarks across `bluetape-rs`, `bluetape-go`, and `bluetape4k-projects`,
  followed by measured performance tuning.

## `0.1.0` Scope

1. Establish workspace layout, crate naming, feature-flag policy, release
   hygiene, and CI commands for Rust 2024.
2. Add `bluetape-rs-core` for small shared contracts: typed validation errors,
   validation helpers, string helpers, and small numeric checks.
3. Add `bluetape-rs-logging` for low-friction `tracing` setup, structured fields,
   request/task correlation, bounded correlation IDs, and scoped test capture.
4. Add `bluetape-rs-test` for async test helpers, `MultithreadingTester`,
   `SuspendedJobTester`, assertion helpers, temporary-resource cleanup, and
   future Testcontainers support boundaries.
5. Refresh public docs and examples so `README.md`, `README.ko.md`, and WIP
   status all describe the same package scope and caveats.

## Release Checklist

Branch policy:

- Use `develop` as the default branch and active development center.
- Use `main` as the latest stable release source branch.
- Promote the verified `develop` tree to `main` before creating a stable
  version tag and GitHub Release.
- Keep the operational release flow in `docs/release/release-guide.md`.

1. Workspace crates compile on Rust 2024 with additive feature flags.
2. Public APIs have English Rustdoc and tests for success, failure, boundary,
   and feature-flag behavior where applicable.
3. `bluetape-rs-logging` integrates with `tracing` without forcing a global
   subscriber in library code.
4. `bluetape-rs-test` avoids global mutable test state unless ownership and cleanup
   are explicit.
5. Validation passes with `cargo fmt --all`, `cargo test --workspace`,
   `cargo test --workspace --all-features`,
   `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and
   `git diff --check`.
6. README parity is maintained for public behavior, package scope, and roadmap
   changes.

## Milestone Roadmap

This roadmap follows the broad shape of `bluetape-go` while keeping package
contracts Rust-native. GitHub milestone `0.4.0` is the active compression
milestone; the next serialization track is split across `0.5.x` so the cache
payload foundation can land before JSON, Protobuf, Avro, Fory, and cross-repo
benchmark follow-ups.

| Milestone | Theme | Notes |
|---|---|---|
| `0.1.0` | Core helpers, logging, and test support | Rust workspace hygiene, `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-test`, docs parity. |
| `0.1.1` | Retrospective quality closure | Fill missing specs, review evidence, CI metadata, and docs gaps found after `0.1.0`. |
| `0.2.0` | Collections and async/concurrency helpers | Focused iterator/map helpers, Tokio task helpers, bounded concurrency, cancellation/deadline helpers. |
| `0.3.0` | Codec helpers | Base encoders, hex, URL-safe codecs, and small binary/text codec helpers. |
| `0.4.0` | Compression helpers | Opt-in compression helpers, safe defaults, streaming boundaries, and registry-style codec selection. |
| `0.5.0` | Core + binary serialization | Cache-first serializer/deserializer traits, binary payload contract, typed errors, format metadata, trust profiles, and first binary adapter. |
| `0.5.1` | JSON serialization | `serde_json` bytes-first adapter, UTF-8 string helpers, explicit typed decode, and malformed-input tests. |
| `0.5.2` | Protobuf serialization | Typed Protobuf adapter, opt-in `Any`/type URL handling, static target decode, and compatibility fixtures. |
| `0.5.3` | Avro serialization | Schema-first Avro adapter, writer/reader schema handling, codec interaction, and schema evolution tests. |
| `0.5.4` | Fory cross-language evaluation | Apache Fory compatibility matrix across Rust, Go, Kotlin, Java, and Python; benchmark and safety evidence before any default adoption. |
| `0.5.5` | Cross-repo SerDe benchmark and performance tuning | Same-environment benchmark scenarios across `bluetape-rs`, `bluetape-go`, and `bluetape4k-projects`, comparable reports, and measured safe optimizations. |
| `0.6.0` | Testcontainers fixtures | PostgreSQL, Redis, MySQL, NATS, Kafka, and emulator fixture boundaries behind explicit features. |
| `0.7.0` | Relational SQL DSL and repository helpers | Inspectable SQL AST, dialect rendering, bind separation, SQLx adapter; no ORM claim. |
| `0.8.0` | Resilience primitives | Retry, timeout, circuit breaker, bulkhead, backoff, policy composition, observability hooks. |
| `0.9.0` | Leader election | Redis, RDB, etcd, and Kubernetes Lease leader election with fencing tokens; depends on fixture support and SQL foundations. |
| `0.10.0` | Cache and coordination | Local TTL cache, same-key load collapse, Redis locks, rate limiting, coordination examples. |
| `0.11.0` | Portable utility packages | IDs, JWT, measured values, money, probabilistic structures, and utility stabilization. |
| `0.12.0` | Research and crypto/encryption gate | Tink/encryption feasibility and research gates for larger domains. |
| `0.13.0` | AWS helper packages and examples | Thin helpers around the official AWS SDK for Rust plus emulator-backed examples. |
| `0.14.0` | Text packages | Aho-Corasick search, blockword masking, tokenizer feasibility, language detection. |
| `0.15.0` | Audit and event packages | Snapshot, diff, outbox, event-stream primitives inspired by audit workloads. |
| `0.16.0` | Graph packages and examples | Graph abstraction, graph I/O, and selected backend adapters after driver maturity review. |
| `0.17.0` | Rule engine research and implementation | Rule model, evaluation contracts, and integration examples if research justifies the scope. |

## Task Queue

### `0.1.0` - Foundation

- Create the workspace structure and crate naming policy.
- Define MSRV, Rust 2024, feature-flag, and release hygiene rules.
- Add `bluetape-rs-core` for typed validation errors, validation helpers,
  string helpers, and small numeric checks.
- Add `bluetape-rs-logging` for `tracing` setup helpers, structured fields,
  bounded correlation IDs, and scoped test capture.
- Add `bluetape-rs-test` for eventual/consistent assertions,
  `MultithreadingTester`, `SuspendedJobTester`, async test helpers, and temporary
  resource cleanup.
- Keep `README.md`, `README.ko.md`, and WIP synchronized.

#### `bluetape-rs-core` Scope

Evidence used:

- `bluetape-go/core` is deliberately narrow: validation helpers, pointer-like
  optional value helpers, zero/default helpers, string helpers, and small numeric
  checks.
- `bluetape4k-core` is much broader: validation, codec, collections,
  concurrency, ranges, Java time, Apache wrappers, functional helpers, and
  runtime utilities. Those categories should not be copied into `0.1.0`.

Include in `0.1.0`:

- Typed validation errors: field/name, invalid value when useful, and stable
  error messages with `std::error::Error` support.
- Validation helpers: non-empty string, non-blank string, inclusive range,
  half-open range, positive number, non-negative number.
- Option/result helpers are intentionally absent in `0.1.0`; standard
  combinators remain clearer for the current scope.
- String helpers: `has_text`, empty/blank fallback helpers, UTF-8 byte-boundary
  truncation, prefixed hex format checks.
- Numeric helpers: checked clamp with invalid-range errors and small hex digit
  predicates.
- Public Rustdoc examples and tests for success, failure, boundary, Unicode,
  and invalid-range cases.

Defer out of `0.1.0`:

- Codec/base encoders to `0.3.0`.
- General-purpose collections and async/concurrency helpers to `0.2.0`.
- Compression to `0.4.0`.
- Serialization to `0.5.0`.
- Testcontainers to `0.6.0`.
- ID generation, time DSLs, money, measurements, probabilistic structures, and
  other portable utilities to `0.11.0`.
- JVM/Kotlin-specific concepts such as Kotlin contracts, Apache wrapper APIs,
  Java reflection helpers, virtual threads, Reactor, and Java Time DSLs.

### `0.2.0` - Collections and Async/Concurrency Helpers

- Add focused collection helpers only where `std`, `itertools`, or existing
  crates do not already provide the obvious answer. Implemented as
  `bluetape-rs-collections` with iterator, map, slice, and page value helpers.
- Add Tokio-first task group and bounded concurrency helpers. Implemented
  initially as `bluetape-rs-async` with `try_map_bounded` and
  `map_bounded_collect`.
- Add cancellation, timeout, shutdown, and deadline helpers. Implemented as
  `with_timeout`, `with_deadline`, `run_until_cancelled`,
  `with_timeout_or_cancel`, `CancellationSource`, `CancellationToken`, and
  shutdown signal pairs.
- Add deterministic async test patterns using `bluetape-rs-test`. Implemented
  integration coverage with paused time, bounded stress, and eventual/consistent
  assertions.
- Prove no task leaks or unbounded resource growth in tests. Implemented
  abort/drain and drop-counter checks for started sibling futures.
- Deferred from `0.2.0`: codec, compression, serialization, Testcontainers,
  SQL, resilience, leader election, cache/coordination, AWS, text, audit, graph,
  and rule-engine work.

### `0.3.0` - Codec Helpers

- Implemented as `bluetape-rs-codec` with strict hex, Base64 standard,
  Base64 URL-safe, Bitcoin Base58, byte-oriented Base62, and UTF-8 text
  boundary helpers.
- Kept codec APIs explicit about allocation and typed decode error contracts.
- Added crate-boundary examples and tests; benchmarks remain deferred until
  usage patterns justify a stable measurement surface.

### `0.4.0` - Compression Helpers

- Implemented the initial `bluetape-rs-compression` crate with gzip, zlib,
  deflate, zstd, lz4, and snappy compressors behind additive feature flags.
- Added typed configuration and error contracts with registry-style algorithm
  selection.
- Added config-aware decompression safety limits, a 64 MiB default decode
  safety limit, stream copy helpers, and direct stream reader/writer
  constructors for `Read`/`Write` boundaries.
- Added the same-condition benchmark runner and report comparing `bluetape-rs`,
  `bluetape-go`, and `bluetape4k-io` on JSON, text, binary, and random
  payloads.

### `0.5.0` - Core + Binary Serialization

- Add the serialization crate boundary for Rust-native serializer/deserializer
  contracts.
- Define typed `Serializer`/`Deserializer` and binary payload traits suitable
  for internal cache storage and restoration.
- Define typed errors, format id, content type, version, and trust profile
  vocabulary.
- Select and implement the first binary adapter with explicit format selection
  and no magic global defaults.
- Keep compression composition explicit and compatible with the existing
  `0.4.0` compression crate.
- Cover round-trip, invalid input, empty payload, version mismatch, format
  mismatch, and cache payload boundary cases with focused tests.
- Defer JSON, Protobuf, Avro, and Fory production adapters to later `0.5.x`
  milestones.

### `0.5.1` - JSON Serialization

- Add a `serde_json`-based bytes-first adapter with UTF-8 string helpers.
- Keep typed decode explicit through `serde::Deserialize`.
- Cover malformed JSON, target type mismatch, UTF-8 boundary, and compact output
  cases with focused tests.
- Defer JSONB or binary JSON adoption until usage evidence justifies it.

### `0.5.2` - Protobuf Serialization

- Add typed Protobuf serialization, likely around `prost`.
- Require the caller to supply the target type by default; serialized bytes must
  not choose arbitrary Rust types.
- Treat `Any` and type URL support as opt-in because dynamic type selection is a
  trust-boundary decision.
- Add fixtures for corrupted messages, wrong target types, and compatibility
  behavior.
- Keep gRPC transport concerns out of scope.

### `0.5.3` - Avro Serialization

- Add schema-first Avro serialization, likely around `apache-avro`.
- Define how writer and reader schemas are supplied.
- Add v1-to-v2 and v2-to-v1 schema evolution fixtures.
- Cover Avro codec/compression interaction where the chosen Rust Avro backend
  supports it directly.
- Keep schema registry support out of scope.

### `0.5.4` - Fory Cross-Language Evaluation

- Evaluate Apache Fory for Rust, Go, Kotlin, Java, and Python payload
  interoperability.
- Build a compatibility matrix and compare payload size/throughput against the
  `0.5.0` binary adapter.
- Document trust, compatibility mode, schema consistency, and upgrade
  constraints before exposing any adapter as production-ready.
- Keep Fory opt-in until benchmark, compatibility, and safety evidence justify a
  stronger default.

### `0.5.5` - Cross-Repo SerDe Benchmark and Performance Tuning

- Define shared SerDe benchmark fixtures and scenario matrix for `bluetape-rs`,
  `bluetape-go`, and `bluetape4k-projects`.
- Run Rust, Go, and Kotlin/JVM benchmarks under the same machine/run conditions
  and record repository commit SHA, toolchain/runtime versions, benchmark
  command, raw output path, timestamp, and run settings.
- Compare payload size, encode throughput, decode throughput, latency where
  available, allocation/GC notes, and compression interaction where relevant.
- Publish a recommendation matrix for cache-internal, human-readable,
  schema-first, and cross-language use cases.
- Use benchmark results to create or execute only focused, safe performance
  improvements that preserve correctness, trust boundaries, compatibility, and
  docs.
- Avoid declaring a global default serializer from one benchmark run or
  comparing unlike scenarios without caveats.

### `0.6.0` - Testcontainers Fixtures

- Add reusable fixture helpers for PostgreSQL and Redis first.
- Add MySQL, NATS, Kafka, and local emulator fixtures only after the fixture
  contract is stable.
- Keep all container-backed behavior behind explicit features.
- Prove cleanup behavior on success, failure, and cancellation paths.

### `0.7.0` - Relational SQL

- Start with an inspectable SQL AST and dialect renderer.
- Preserve SQL text and bind values separately.
- Add SQLx execution adapters after rendering behavior is testable.
- Use PostgreSQL Testcontainers before claiming database support.
- Do not claim ORM support until lifecycle, relation loading, migrations, and
  transaction semantics are explicitly designed and tested.

### `0.8.0` - Resilience

- Add retry, timeout, circuit breaker, bulkhead, and backoff policies.
- Keep policies first-party and composable; avoid wrapping a full external
  resilience framework.
- Add cancellation/deadline tests and observability hooks.
- Add HTTP/service examples only after the core policy contract is stable.

### `0.9.0` - Leader Election

- Treat leader election as a large multi-backend track, not an early helper.
- Add Redis leader election only after Redis Testcontainers fixtures are usable.
- Add RDB-backed leader election only after relational SQL and PostgreSQL
  Testcontainers foundations are usable.
- Evaluate etcd and Kubernetes Lease backends separately before accepting them
  into the milestone scope.
- Define owner tokens, fencing tokens, renewal, resign, lookup, and shutdown
  semantics.
- Add real backend lifecycle tests before claiming support.

### `0.10.0` - Cache and Coordination

- Add local TTL cache interfaces and same-key load collapse.
- Add Redis lock and rate-limit helpers only behind explicit features.
- Keep cross-process cache invalidation separate from local cache semantics.
- Add Testcontainers-backed smoke tests for Redis-backed behavior.

### `0.11.0` - Portable Utilities

- Add ID generation helpers where Rust-native crates provide strong building
  blocks.
- Add JWT helpers with explicit algorithms and key selection.
- Add measured values, money, and probabilistic structures if dependency and
  API costs remain acceptable.
- Keep provider-backed adapters deferred unless usage evidence is clear.

### `0.12.0` - Research and Encryption Gate

- Evaluate Tink/encryption support, Rust crate maturity, and key-management
  boundaries.
- Collect research for SQL, AWS, text, audit, graph, and rule-engine tracks
  before implementation milestones consume them.
- Create specs/plans before substantial implementation.

### `0.13.0` - AWS

- Prefer thin helpers around the official AWS SDK for Rust.
- Add local emulator examples for S3, SQS, DynamoDB, or equivalent services.
- Keep credentials, retries, and region behavior explicit.

### `0.14.0` - Text

- Add Aho-Corasick search and blockword masking first.
- Research tokenizer and language-detection crates before adopting them.
- Keep large model/runtime dependencies out of default features.

### `0.15.0` - Audit and Events

- Add snapshot, diff, outbox, and event-stream primitives.
- Keep the design inspired by audit workloads, not dependent on JaVers.
- Define ordering, idempotency, and serialization contracts before adapters.

### `0.16.0` - Graph

- Re-evaluate Rust graph driver maturity before implementation.
- Add graph abstraction and graph I/O only after backend costs are clear.
- Keep graph examples narrow and backend-specific.

### `0.17.0` - Rule Engine

- Research rule model, expression language, and evaluation safety first.
- Add implementation only if the model remains small, testable, and useful for
  backend services.
- Keep dynamic execution and untrusted input boundaries explicit.

## Deferred

- Full ORM or DAO lifecycle.
- Spring Boot style auto-configuration.
- Mechanical Kotlin/JVM or Go API parity.
- SQL, SQLx, ORM, or repository abstractions before the general helper,
  logging, test-support, codec/compression/serialization, and Testcontainers
  foundations are proven.
- Leader election before relational SQL, resilience, and Redis/PostgreSQL
  Testcontainers foundations are available.
- Broad AWS, graph, text, or audit coverage before the core foundation is
  stable.
- Public release automation beyond the minimum needed for `0.1.0`.

## Decision Log

- Keep `README.md` and `README.ko.md` synchronized when package scope, roadmap,
  install guidance, or development commands change.
- Use `develop` as the integration branch and default branch.
- Use `main` as the latest stable release branch; promote `develop` to `main`
  before tagging stable releases.
- Use `bluetape-rs-*` as Cargo package names and accept the Rust import form
  `bluetape_rs_*` for library targets.
- Keep public APIs Rust-native: `Result`, `Option`, ownership-aware builders,
  narrow traits, additive feature flags, and explicit error enums.
- Prefer small crates with clear backend service value over broad utility bags.
- Use `tokio` as the default async runtime for infrastructure-facing packages.
- Avoid `unsafe`; if it ever becomes necessary, isolate it, document invariants,
  and add tests around the safe boundary.
- Use Testcontainers-backed tests before claiming PostgreSQL, Redis, Kafka,
  AWS emulator, graph database, or other infrastructure support.
- Keep early milestones boring and broadly useful: helper functions, logging,
  and test support before database abstractions.
- Keep codec, compression, serialization, Testcontainers, and leader election
  split into separate milestones so `0.1.0` remains small.
- Split serialization across `0.5.x`: start with cache-first binary payload
  support in `0.5.0`, then add JSON, Protobuf, Avro, and Fory validation in
  separate milestones, followed by same-environment cross-repo benchmarks and
  measured performance tuning in `0.5.5`.
- Implement relational SQL before resilience because repository/database
  ergonomics should be proven before higher-level runtime policies.
- Implement leader election after relational SQL and resilience because the
  backend matrix is larger: Redis, RDB, etcd, Kubernetes Lease, and possibly
  other coordination systems.
- When SQL starts, treat it as an inspectable AST plus dialect renderer and
  SQLx adapter; do not claim ORM support until lifecycle, relation loading,
  migrations, and transaction semantics are explicitly designed and tested.

## Evidence

- Repo-local feasibility research:
  `docs/research/2026-06-08-backend-library-feasibility.md`
- Source wiki record:
  `../bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md`
