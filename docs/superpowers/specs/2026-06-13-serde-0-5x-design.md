# SerDe 0.5.x Design

Date: 2026-06-13
Status: approved design
Scope: Rust-native serializer/deserializer milestones for `0.5.x`.

## Decision

The `0.5.x` serialization line starts with cache-first binary payload support.
The first milestone must provide the smallest useful binary serializer
foundation for internal cache storage before adding schema, JSON, or
cross-language formats. Cross-repo benchmarks and measured performance tuning
are deferred to a dedicated `0.5.5` milestone after the core adapters exist.

This design intentionally does not mechanically port Kotlin/JVM APIs. It uses
Rust-native contracts: typed errors, `Result`, `Option`, explicit target types,
small traits, additive feature flags, and no dynamic type loading by default.

## Reference Evidence

The Kotlin `bluetape4k-projects` serialization modules provide useful domain
boundaries but not direct API shapes:

- `io/io` defines a `BinarySerializer` boundary, trust profiles, compression
  composition, and Kryo/Fory/JDK implementations.
- `io/json` defines a bytes-first JSON serializer contract with string helpers.
- `io/jackson3` and `io/fastjson2` provide concrete JSON implementations; the
  Fastjson2 module also exposes JSONB byte helpers.
- `io/protobuf` uses Protobuf `Any` packing with an allowlist for type URLs and
  a fallback serializer for mixed stores.
- `io/avro` splits schema-first generic records, specific records, reflection,
  codec selection, and schema evolution tests.
- Apache Fory is valuable for cross-language payloads, but it needs separate
  compatibility and safety validation before becoming a production default.

## Milestone Split

### `0.5.0` - Core + Binary SerDe

Goal: provide the internal cache payload foundation.

Scope:

- Add the serialization crate boundary under `crates/serialization`.
- Define core traits for serialization and deserialization.
- Define binary payload contracts for cache storage and restoration.
- Define typed error, format id, content type, version, and trust profile
  vocabulary.
- Select and implement the first binary adapter.
- Keep compression composition explicit and compatible with the existing
  `0.4.0` compression crate.
- Add round-trip, invalid input, empty payload, version mismatch, and format
  mismatch tests.

Issue #108 bootstrap slice:

- Create `crates/serialization` with package name `bluetape-rs-serialization`
  and library name `bluetape_rs_serialization`.
- Register the crate in `[workspace].members` and `[workspace.dependencies]`.
- Add a root optional dependency and root facade feature:
  `serialization = ["dep:bluetape-rs-serialization"]`.
- Re-export the crate from `bluetape-rs` only behind
  `#[cfg(feature = "serialization")]`.
- Keep the root default feature set unchanged.
- Keep the serialization crate default feature set minimal and free of JSON,
  Protobuf, Avro, Fory, Testcontainers, SQL, and resilience dependencies.
- Stop at crate bootstrap, facade wiring, README/Rustdoc boundary text, and
  roadmap parity. The first binary adapter is still part of the `0.5.0`
  milestone, but not part of issue #108 unless the issue scope is expanded.

Feature policy:

- Format integrations must be additive, opt-in features when they are added in
  later milestones.
- Future `json`, `protobuf`, `avro`, and `fory` features must not be enabled by
  the root crate default feature set.
- Hidden global registries, env-selected adapters, and default serializers are
  not allowed in `0.5.0`.

Cache payload contract:

- The cache envelope records format id, content type, payload version, trust
  profile, adapter id, and payload size without requiring payload bytes in
  diagnostics.
- Decode failures are typed. At minimum, the plan must account for invalid
  payload, unsupported version, format mismatch, content-type mismatch, trust
  profile mismatch, oversized payload, and adapter failure cases.
- Corrupt, unknown-version, wrong-format, wrong-trust-profile, truncated, or
  trailing-byte payloads must not silently decode, fall back to `None`, or try
  alternate adapters. Eviction, cache flush, namespace migration, or rebuild is
  caller policy.
- Old-reader/new-writer and rollback behavior must be explicit: unknown versions
  fail with typed metadata diagnostics, and cache namespace/version migration is
  documented instead of hidden behind best-effort fallback.
- The first binary adapter must use caller-supplied target types only. Payloads
  never select Rust types dynamically.
- Unsafe deserialization, dynamic registries, unbounded collection/depth decode,
  and unbounded decompressed size are not allowed.
- The implementation plan must define buffer ownership, copy boundaries,
  compact header/envelope expectations, and small/medium/large payload runtime
  checks without claiming cross-repo benchmark superiority before `0.5.5`.

Out of scope:

- JSON, Protobuf, Avro, and Fory production adapters.
- Schema registry or schema evolution support.
- JSON as the primary cache payload format.
- Testcontainers or external service integration.
- SQL, SQLx, database adapters, or ORM integration.
- Resilience, retry, circuit-breaker, or fallback policy APIs.
- Hidden global registries, hidden default serializers, or env-selected
  adapters.
- Dynamic type loading.

### `0.5.1` - JSON SerDe

Goal: add a portable, human-readable SerDe adapter after the binary foundation.

Scope:

- Use `serde_json` as the default JSON backend.
- Provide bytes-first APIs plus UTF-8 string helpers.
- Keep typed decode explicit through `serde::Deserialize`.
- Add tests for malformed JSON, target type mismatch, UTF-8 boundaries, and
  pretty/compact output where supported.

Out of scope:

- Jackson/Fastjson-style module parity.
- JSONB or binary JSON adoption as a default.

### `0.5.2` - Protobuf SerDe

Goal: add typed Protobuf serialization without allowing payload-selected Rust
types by default.

Scope:

- Use the Rust Protobuf ecosystem, likely `prost`, for typed messages.
- Provide typed encode/decode APIs where the caller supplies the target type.
- Treat `Any` and type URL support as opt-in because dynamic type selection is a
  security boundary.
- Add compatibility fixtures and failure tests for corrupted messages and wrong
  target types.

Out of scope:

- gRPC transport concerns.
- General mixed-object fallback serialization.

### `0.5.3` - Avro SerDe

Goal: add schema-first serialization with explicit schema evolution tests.

Scope:

- Use the Rust Avro ecosystem, likely `apache-avro`.
- Support schema-bound records first.
- Define how writer and reader schemas are supplied.
- Add schema evolution fixtures equivalent to v1-to-v2 and v2-to-v1 tests.
- Add codec/compression interaction tests where Avro supports it directly.

Out of scope:

- A full schema registry.
- Reflection-like JVM parity.

### `0.5.4` - Apache Fory Cross-Language

Goal: evaluate and optionally add Fory for cross-language binary payloads.

Scope:

- Verify Rust, Go, Kotlin, Java, and Python interoperability claims with a
  compatibility matrix.
- Measure payload size and throughput against the `0.5.0` binary adapter.
- Document trust, compatibility mode, schema consistency, and upgrade
  constraints.
- Keep any adapter opt-in until production safety is proven.

Out of scope:

- Making Fory the default binary serializer without benchmark and compatibility
  evidence.

### `0.5.5` - Cross-Repo Benchmark and Performance Tuning

Goal: compare `bluetape-rs`, `bluetape-go`, and `bluetape4k-projects` SerDe
behavior under the same environment and scenarios, then tune only measured
bottlenecks.

Scope:

- Define shared payload fixtures and scenario matrix across Rust, Go, and
  Kotlin/JVM.
- Run benchmarks for the same scenario cells under the same machine/run
  conditions.
- Record repository commit SHA, toolchain/runtime versions, benchmark command,
  raw output path, timestamp, warmup/iteration settings, and environment.
- Compare payload size, encode time, decode time, throughput, allocation/GC
  notes, and compression interaction where practical.
- Publish a recommendation matrix for cache-internal, human-readable,
  schema-first, and cross-language use cases.
- Execute focused performance tuning follow-ups only when before/after
  measurements justify the change.

Out of scope:

- Declaring a global default serializer from one local benchmark run.
- Comparing unlike scenarios without caveats.
- Trading away correctness, security, compatibility, or API clarity for
  microbenchmark gains.

## API Direction

The public API should separate payload format from Rust type conversion:

- `Serializer<T>` and `Deserializer<T>` are typed contracts.
- `BinarySerializer<T>` is a bytes boundary for cache and infrastructure
  payloads.
- Format metadata is explicit and stable enough for cache invalidation,
  migration, rollback, and debugging.
- `Option<T>` represents absent values; empty bytes are treated as a payload
  boundary decision, not a hidden null convention.
- `SerializationTrustProfile` documents whether a format is trusted-internal,
  allowlisted, statically typed, or unsafe legacy compatibility.
- Error values expose safe metadata such as expected/observed format id,
  content type, payload version, trust profile, adapter id, and payload size
  without logging or returning payload bytes.

## GitHub Issue Rebalancing

Existing `0.5.0` issues should be narrowed instead of growing the first
milestone:

- Keep crate bootstrap, core contracts, and first binary adapter in `0.5.0`.
- Keep issue #108 limited to crate bootstrap, root facade gating, and
  documentation parity unless its GitHub scope is explicitly expanded.
- Move JSON adapter work to `0.5.1`.
- Split schema-drift checks into Protobuf and Avro follow-ups.
- Move Fory cross-language work to `0.5.4`.
- Add `0.5.5` issues for cross-repo benchmark fixtures, same-condition runners,
  report publication, and measured performance tuning.
- Keep documentation and release readiness issues milestone-specific.

## Acceptance Criteria

- `WIP.md` documents the `0.5.x` split and matches this design.
- Future implementation plans keep `0.5.0` cache-first and binary-first.
- Issue #108 implementation plans name `crates/serialization`,
  `bluetape-rs-serialization`, `bluetape_rs_serialization`, root
  `serialization` feature gating, and unchanged root defaults.
- Feature verification proves the root facade is unavailable by default and
  available only with `features = ["serialization"]`.
- Feature verification proves default builds do not pull JSON, Protobuf, Avro,
  Fory, Testcontainers, SQL, or resilience dependencies.
- `crates/serialization/README.md`, crate Rustdoc, `README.md`, `README.ko.md`,
  and `WIP.md` use the same crate name, root facade feature name, and `0.5.0`
  non-goal list.
- Public docs state that JSON, Protobuf, Avro, Fory, Testcontainers, SQL,
  resilience APIs, hidden globals, hidden default serializers, dynamic type
  loading, and schema registry support are out of `0.5.0` scope.
- Public docs show direct crate usage and root facade feature-gated usage, and
  explain `Option<T>`, empty bytes, version mismatch, format mismatch, and
  trust-profile mismatch behavior.
- Tests or verification tasks cover corrupt bytes, truncated bytes, trailing
  bytes, empty bytes, unknown format id, unsupported version, wrong target
  type, trust-profile mismatch, oversized payload, and compressed-invalid
  payloads before `0.5.0` release readiness is claimed.
- Existing root crate users need no migration for issue #108 because
  serialization is opt-in.
- No milestone claims Protobuf, Avro, or Fory production readiness before their
  dedicated validation milestones.
- Benchmark claims use the same-environment `0.5.5` benchmark track when they
  compare Rust, Go, and Kotlin/JVM project lines.
- Public docs preserve Rust-native positioning and avoid Kotlin/JVM API parity
  promises.
