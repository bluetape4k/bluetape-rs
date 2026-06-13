# bluetape-rs-serialization

Rust-native serialization boundary for `bluetape-rs`.

This crate is the bootstrap slice for the `0.5.0` cache-first binary
serialization milestone. It creates the package and documentation boundary first
so later issues can add traits, typed errors, binary payload envelopes, and
adapters behind reviewed feature flags.

## Usage

Direct crate dependency:

```toml
# Until 0.5.0 is published:
bluetape-rs-serialization = { git = "https://github.com/bluetape4k/bluetape-rs", package = "bluetape-rs-serialization" }

# After 0.5.0 is published:
bluetape-rs-serialization = "0.5"
```

```rust
use bluetape_rs_serialization as serialization;
```

Root facade usage:

```toml
# Until 0.5.0 is published:
bluetape-rs = { git = "https://github.com/bluetape4k/bluetape-rs", features = ["serialization"] }

# After 0.5.0 is published:
bluetape-rs = { version = "0.5", features = ["serialization"] }
```

```rust
use bluetape_rs::serialization;
```

The root facade is unavailable unless the `serialization` feature is enabled.
Default `bluetape-rs` builds remain unchanged.

The bootstrap crate exposes no serializer traits or adapters yet. Those arrive
in later reviewed `0.5.0` issues.

## Boundary

`0.5.0` starts with cache-first binary payload support. Payload metadata,
versioning, trust profiles, typed failures, and adapter contracts are added in
follow-up issues.

`Option<T>` represents absent values. Empty bytes are payload data and must not
be treated as a hidden null convention.

Future unsupported-version, wrong-format, and wrong-trust-profile cases are
typed decode failures. They must not decode as `None`, silently fall back, or
try alternate adapters. Cache eviction, namespace migration, and rebuild policy
belong to callers.

## Migration / Compatibility

Existing `bluetape-rs` users do not need code or Cargo changes for issue #108.
Callers only opt in by depending on `bluetape-rs-serialization` directly or by
enabling `features = ["serialization"]` on `bluetape-rs`.

## Issue #108 Bootstrap Non-goals

- Serializer traits or concrete adapters
- Runtime binary payload encoding
- Global serializer registry

## Not In The `0.5.0` Core/Binary Milestone

- JSON adapter
- Protobuf adapter
- Avro adapter
- Apache Fory adapter
- Testcontainers integration
- SQL or SQLx integration
- Resilience, retry, circuit-breaker, or fallback policies
- Unsafe deserialization
- Hidden global serializers
- Hidden default serializers
- Env-selected adapters
- Dynamic type loading
- Schema registry support
