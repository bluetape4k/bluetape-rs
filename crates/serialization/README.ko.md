# bluetape-rs-serialization

`bluetape-rs`를 위한 Rust-native serialization contract입니다.

이 crate는 cache-first serialization을 위한 `0.5.0` contract layer를 정의합니다:
검증된 metadata, safe default, typed error, payload envelope, `serde` 호환
serializer/deserializer trait입니다. Concrete binary, JSON, Protobuf, Avro,
Apache Fory, benchmark 작업은 후속 issue 범위입니다.

## 사용법

직접 crate 의존:

```toml
# 0.5.0 publish 전:
bluetape-rs-serialization = { git = "https://github.com/bluetape4k/bluetape-rs", package = "bluetape-rs-serialization" }

# 0.5.0 publish 후:
bluetape-rs-serialization = "0.5"
```

Root facade 사용:

```toml
# 0.5.0 publish 전:
bluetape-rs = { git = "https://github.com/bluetape4k/bluetape-rs", features = ["serialization"] }

# 0.5.0 publish 후:
bluetape-rs = { version = "0.5", features = ["serialization"] }
```

## Contracts

- `SerializationFormat`, `ContentType`, `AdapterId`, `PayloadVersion`는 stable
  metadata token을 검증합니다.
- `SerializationConfig` 기본값은 statically typed trust,
  `application/octet-stream`, payload version `1`, 16 MiB payload limit입니다.
- `SerializedPayload`는 bytes와 metadata를 함께 소유해 `payload_size`가
  `bytes.len()`과 일치하도록 합니다.
- `PayloadMetadataPolicy`는 format, content type, version, trust profile,
  adapter id, size mismatch를 typed error로 거부합니다.
- `Serializer<T>`, `Deserializer<T>`, `BinarySerializer<T>`는 `serde` 호환
  contract입니다. Rust target type은 caller가 제공합니다.

## Example

```rust
use bluetape_rs_serialization::{
    AdapterId, PayloadMetadataPolicy, SerializationConfig, SerializationErrorKind,
    SerializationFormat, SerializedPayload,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}
```

## Safety Boundary

Dynamic registry, hidden default serializer, environment-selected adapter,
fallback adapter, payload-selected Rust type은 없습니다.

`UnsafeLegacyCompatibility`는 완전히 trusted deployment의 migration-only
vocabulary입니다. 기본값이 아니며 shared/untrusted payload boundary에서는 별도
adapter review 없이는 사용하지 않습니다.

## Cache Rollout Guidance

Format id, content type, trust profile, incompatible payload version이 바뀌면
cache namespace나 key prefix를 versioning합니다. Mismatch는 hard typed
failure입니다. Caller-owned action은 evict, source of truth 기반 rebuild,
namespace migration, 예상 밖 mismatch alert입니다.

Payload-free diagnostic field는 error kind, operation, format id, content type,
version relation, trust profile, adapter id, payload size bucket, configured size
limit입니다.

## 이 Contract Issue 범위 밖

- Binary adapter
- JSON adapter
- Protobuf adapter
- Avro adapter
- Apache Fory adapter
- Testcontainers integration
- SQL 또는 SQLx integration
- Resilience, retry, circuit-breaker, fallback policy
- Default unsafe deserialization
- Hidden global serializer
- Hidden default serializer
- Env-selected adapter
- Dynamic type loading
- Schema registry support
