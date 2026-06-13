# Serialization Contracts Design

Date: 2026-06-13
Status: draft for Step 2-R review
Scope: issue #109, milestone `0.5.0`, `crates/serialization`

## Problem

Issue #108 created the `bluetape-rs-serialization` crate boundary and the
optional root facade, but the crate still exposes no serialization contracts.
Issue #109 must define the public vocabulary that later `0.5.0` work can build
on: explicit format identifiers, typed errors, payload metadata, trust profiles,
and safe configuration defaults.

This issue is not the first adapter implementation. Binary, JSON, Protobuf,
Avro, and Fory adapters remain in later milestone issues. The result of #109 is
a small Rust-native contract layer that makes those adapter issues implementable
without changing the public API shape again.

## Current Evidence

- `crates/serialization/src/lib.rs` currently contains only crate Rustdoc and a
  metadata smoke test. It explicitly says serializer traits, payload envelopes,
  and adapters are later reviewed issues.
- `WIP.md` lists typed serializer/deserializer contracts, typed errors, format
  id, content type, version, and trust profile vocabulary before the first
  binary adapter.
- `docs/superpowers/specs/2026-06-13-serde-0-5x-design.md` states that `0.5.0`
  is cache-first and binary-first, while JSON, Protobuf, Avro, Fory, and
  cross-repo benchmarks are separate `0.5.x` milestones.
- Official Serde documentation separates the data model traits
  `serde::Serialize` and `serde::Deserialize` from concrete data format crates.
  It also shows the usual format crate structure as separate `ser`, `de`, and
  `error` modules.
- Kotlin/JVM `bluetape4k-projects` has useful domain evidence for trust
  profiles, but prior security reviews show that dynamic class loading,
  fallback object deserialization, and allow-all defaults are real
  deserialization risks.
- `bluetape-go` has a small `serialization` package with `Serializer[T]`,
  `NamedSerializer[T]`, and a versioned envelope. It is useful as domain
  evidence, but this Rust crate must not mechanically port Go method names or
  interface shapes.
- Sibling Rust crates keep `lib.rs` concise and split implementation into
  focused modules. `crates/compression` uses `mod config`, `mod error`,
  `mod registry`, `mod stream`, and `mod traits`; #109 should follow that shape
  instead of expanding `lib.rs`.

## Goals

- Define explicit, typed format identifiers without hidden global defaults.
- Define typed serializer/deserializer contracts that are compatible with
  `serde::Serialize` and `serde::de::DeserializeOwned`.
- Define binary payload metadata for cache and infrastructure payloads:
  content type, payload version, format id, trust profile, adapter id, and
  payload size.
- Define typed error enums for encode, decode, config validation, format
  mismatch, content type mismatch, version mismatch, trust profile mismatch,
  malformed input, oversized payload, and adapter failures.
- Define trust profile vocabulary aligned with the bluetape ecosystem:
  trusted internal, allowlisted types, statically typed, and unsafe legacy
  compatibility.
- Define safe configuration defaults with documented rationale.
- Keep byte/string boundaries explicit.
- Keep compression, codec, and serialization concerns separate.

## Non-Goals

- No binary adapter implementation in #109.
- No JSON, Protobuf, Avro, Fory, schema registry, or cross-language production
  adapter.
- No compression composition implementation.
- No dynamic registry, hidden default serializer, environment-selected adapter,
  or payload-selected Rust type.
- No Testcontainers, Redis, database, SQLx, resilience, or benchmark harness.
- No Kotlin/JVM or Go API parity promise.

## Proposed Design

Use the approved B approach: Rust-native typed contract modules.

`lib.rs` remains the crate-level index and export surface:

- `mod config;`
- `mod error;`
- `mod format;`
- `mod metadata;`
- `mod traits;`
- `mod trust;`

Public exports come from those focused modules. Long explanations stay in
Rustdoc for each type plus README/spec text, not in `lib.rs`.

### Format Vocabulary

`SerializationFormat` is a small validated value type, not a global registry.
It represents a stable format id such as `binary`, `json`, `protobuf`, `avro`,
or adapter-specific ids added later.

Rules:

- Format ids are ASCII lowercase tokens.
- Allowed characters are `a-z`, `0-9`, `-`, `_`, `.`, and `/`.
- Empty, blank, control-character, uppercase, and overly long ids are rejected.
- The type stores an owned string so later adapters can define stable custom
  ids without changing the enum.

This avoids a closed enum that would need semver-visible changes for every
adapter, while still preventing arbitrary unvalidated metadata.

### Trust Profiles

`SerializationTrustProfile` is a closed enum:

- `TrustedInternal`: private caches or queues inside one trusted deployment.
- `AllowListedTypes`: formats that may carry type metadata but restrict it with
  explicit allowlists.
- `StaticallyTyped`: payloads do not select runtime types; callers supply the
  Rust target type.
- `UnsafeLegacyCompatibility`: explicit migration-only boundary for allow-all or
  legacy compatibility paths.

The #109 default config uses `StaticallyTyped`, because Rust callers should
provide target types through `DeserializeOwned` and because `0.5.0` must not
enable dynamic type loading.

### Metadata

`PayloadMetadata` describes payloads without exposing payload bytes:

- `format: SerializationFormat`
- `content_type: ContentType`
- `version: PayloadVersion`
- `trust_profile: SerializationTrustProfile`
- `adapter_id: AdapterId`
- `payload_size: usize`

`ContentType`, `PayloadVersion`, and `AdapterId` are validated value types:

- content type is a non-empty ASCII media type token such as
  `application/octet-stream`;
- payload version is a positive `u16`;
- adapter id is a non-empty safe ASCII token;
- payload size is metadata only and never logs or stores payload bytes.

### Traits

The public traits are format-agnostic and bytes-first:

```rust
pub trait Serializer<T>
where
    T: serde::Serialize,
{
    fn serialize(&self, value: &T) -> Result<Vec<u8>, SerializationError>;
    fn metadata(&self, payload_size: usize) -> PayloadMetadata;
}

pub trait Deserializer<T>
where
    T: serde::de::DeserializeOwned,
{
    fn deserialize(&self, bytes: &[u8]) -> Result<T, SerializationError>;
    fn expected_metadata(&self) -> PayloadMetadataPolicy;
}

pub trait BinarySerializer<T>: Serializer<T> + Deserializer<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
}
```

The exact plan may refine method names, but it must keep these constraints:

- byte input is `&[u8]`;
- encode output is owned `Vec<u8>`;
- input values are borrowed for encode;
- decode target type is supplied by the caller through Rust generics;
- no `Any`, `TypeId`, dynamic type registry, or payload-selected type appears
  in the #109 API.

`PayloadMetadataPolicy` captures what a deserializer expects before an adapter
exists. It should support format, content type, max supported version, and trust
profile checks without requiring a concrete envelope implementation in #109.

### Errors

Use a typed `SerializationError` enum. Variants must preserve safe diagnostics:

- encode or decode direction;
- expected and observed format id when relevant;
- expected and observed content type when relevant;
- expected and observed payload version when relevant;
- expected and observed trust profile when relevant;
- adapter id when relevant;
- payload size and limit when relevant;
- malformed-input reason without payload bytes;
- source error for adapter failures where an adapter can provide one later.

The public error must implement `std::error::Error`, `Display`, `Debug`, and
`Send + Sync` where sources are attached. `thiserror` is acceptable because the
workspace already uses it for `crates/compression`.

### Config Defaults

`SerializationConfig` defines safe defaults for later adapters:

- default trust profile: `StaticallyTyped`;
- default content type: `application/octet-stream`;
- default payload version: `1`;
- default max payload size: bounded constant, documented as a safety guard;
- adapter id required for concrete adapters;
- no fallback serializer;
- no hidden compression.

Config validation returns typed errors and rejects zero max sizes, zero payload
versions, unsafe legacy compatibility defaults, blank adapter ids, and invalid
metadata tokens.

### Dependency Policy

Issue #109 may add:

- `serde` as a workspace dependency for public trait bounds;
- `thiserror.workspace = true` in `crates/serialization` for typed errors.

It must not add adapter dependencies such as `bincode`, `serde_json`, `prost`,
`apache-avro`, `fory`, Redis, Testcontainers, or compression adapters.

## Rejected Approaches

### Port Go `Serializer[T]` Directly

Go's `Marshal` and `Unmarshal` names and `NamedSerializer` interface are small
and proven in `bluetape-go`, but directly porting them would ignore Rust's
`serde` trait ecosystem and ownership conventions. The Rust API should use
borrowed encode input, explicit `DeserializeOwned` decode targets, and Rust
error types.

### Closed Enum For Every Format

A closed `enum SerializationFormat { Binary, Json, Protobuf, Avro, Fory }`
looks simple, but every future adapter or user-defined format would require a
public enum change. A validated string newtype gives stable validation while
remaining extensible.

### Dynamic Registry Now

A dynamic registry would make future adapter lookup convenient, but it also
creates hidden defaults and payload-selected behavior. #109 intentionally keeps
adapter selection caller-owned and explicit.

## Risks And Failure Modes

1. **Over-wide public API.** If #109 defines adapter-specific behavior now,
   #111 and later adapter issues will inherit the wrong abstraction. Mitigation:
   contracts only, no adapter implementation.
2. **Security regression by vocabulary drift.** If trust profile names differ
   from Kotlin/JVM docs, cross-repo guidance becomes confusing. Mitigation: use
   the four bluetape trust profiles with Rust-native naming.
3. **Semver trap in format ids.** A closed enum would force future format ids
   into enum variants. Mitigation: validated newtypes for ids.
4. **Hidden payload leaks.** Error messages could accidentally include raw
   bytes. Mitigation: error variants store metadata and reason strings only.
5. **Unclear cache migration semantics.** Version mismatch could imply fallback
   decoding. Mitigation: #109 records mismatch as a typed error; cache eviction,
   namespace migration, or rebuild remains caller policy.

## Acceptance Criteria

- `crates/serialization/src/lib.rs` stays concise and exports focused modules.
- Public contracts compile with Rust 2024 and use `serde`-compatible bounds.
- Public value types validate format id, content type, adapter id, and payload
  version.
- Public errors preserve safe metadata context and never expose payload bytes.
- Config defaults are documented and test-covered.
- Trust profile vocabulary covers trusted internal, allowlisted types,
  statically typed, and unsafe legacy compatibility.
- Tests cover valid defaults, invalid metadata tokens, version mismatch, format
  mismatch, content type mismatch, trust profile mismatch, payload size limit,
  and safe error display.
- README/Rustdoc state that #109 adds contracts only and that adapters remain
  later issues.
- No JSON, Protobuf, Avro, Fory, Testcontainers, Redis, SQLx, resilience, or
  benchmark dependency is added.

## Verification Plan

- `cargo fmt --all --check`
- `cargo test -p bluetape-rs-serialization --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `git diff --check`

## Step 2 DoD

| Item | Status |
|---|---|
| Issue #109 scope isolated from adapter implementation | Required |
| Rust-native module split specified | Required |
| Serde compatibility and dependency policy specified | Required |
| Trust profile vocabulary specified | Required |
| Typed metadata/error/config requirements specified | Required |
| Tests and verification commands specified | Required |
