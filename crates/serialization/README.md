# bluetape-rs-serialization

Rust-native serialization contracts for `bluetape-rs`.

This crate defines the `0.5.0` contract layer for cache-first serialization:
validated metadata, safe defaults, typed errors, payload envelopes, and
`serde`-compatible serializer/deserializer traits. Concrete binary, JSON,
Protobuf, Avro, Apache Fory, and benchmark work remain in follow-up issues.

## Usage

Direct crate dependency:

```toml
# Until 0.5.0 is published:
bluetape-rs-serialization = { git = "https://github.com/bluetape4k/bluetape-rs", package = "bluetape-rs-serialization" }

# After 0.5.0 is published:
bluetape-rs-serialization = "0.5"
```

Root facade usage:

```toml
# Until 0.5.0 is published:
bluetape-rs = { git = "https://github.com/bluetape4k/bluetape-rs", features = ["serialization"] }

# After 0.5.0 is published:
bluetape-rs = { version = "0.5", features = ["serialization"] }
```

## Contracts

- `SerializationFormat`, `ContentType`, `AdapterId`, and `PayloadVersion`
  validate stable metadata tokens.
- `SerializationConfig` applies safe defaults: statically typed trust,
  `application/octet-stream`, payload version `1`, and a 16 MiB payload limit.
- `SerializedPayload` owns bytes and metadata together so `payload_size` matches
  `bytes.len()`.
- `PayloadMetadataPolicy` rejects format, content type, version, trust profile,
  adapter id, and size mismatches with typed errors.
- `Serializer<T>`, `Deserializer<T>`, and `BinarySerializer<T>` are
  `serde`-compatible contracts; callers still supply the Rust target type.

## Example

```rust
use bluetape_rs_serialization::{
    AdapterId, PayloadMetadataPolicy, SerializationConfig, SerializationErrorKind,
    SerializationFormat, SerializedPayload,
};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let config = SerializationConfig::new(
    SerializationFormat::new("binary")?,
    AdapterId::new("binary.primary")?,
)?;
let bytes = vec![1, 2, 3];
let metadata = config.metadata_for_size(bytes.len())?;
let payload = SerializedPayload::new(bytes, metadata)?;

let policy = PayloadMetadataPolicy::from_config(&config);
if let Err(error) = policy.validate(payload.metadata()) {
    match error.kind() {
        SerializationErrorKind::UnsupportedVersion => {
            // Evict, rebuild, migrate namespace, or alert.
        }
        _ => return Err(error.into()),
    }
}
# Ok(())
# }
```

## Safety Boundary

There is no dynamic registry, hidden default serializer, environment-selected
adapter, fallback adapter, or payload-selected Rust type.

`UnsafeLegacyCompatibility` is migration-only vocabulary for fully trusted
deployments. It is not a default and must not be used for shared or untrusted
payload boundaries without a separate adapter review.

## Cache Rollout Guidance

Version cache namespaces or key prefixes when changing format id, content type,
trust profile, or incompatible payload versions. Mismatches are hard typed
failures. Caller-owned actions are evict, rebuild from the source of truth,
migrate namespace, or alert during unexpected mismatches.

Payload-free diagnostic fields are: error kind, operation, format id, content
type, version relation, trust profile, adapter id, payload size bucket, and
configured size limit.

## Not In This Contract Issue

- Binary adapter
- JSON adapter
- Protobuf adapter
- Avro adapter
- Apache Fory adapter
- Testcontainers integration
- SQL or SQLx integration
- Resilience, retry, circuit-breaker, or fallback policies
- Unsafe deserialization by default
- Hidden global serializers
- Hidden default serializers
- Env-selected adapters
- Dynamic type loading
- Schema registry support
