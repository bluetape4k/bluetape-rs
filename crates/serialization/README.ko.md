# bluetape-rs-serialization

`bluetape-rs`를 위한 Rust-native serialization 경계입니다.

이 crate는 `0.5.0` cache-first binary serialization milestone의 bootstrap
slice입니다. 먼저 package와 문서 경계를 만들고, 이후 issue에서 trait, typed
error, binary payload envelope, adapter를 review된 feature flag 뒤에 추가합니다.

## 사용법

직접 crate 의존:

```toml
# 0.5.0 publish 전:
bluetape-rs-serialization = { git = "https://github.com/bluetape4k/bluetape-rs", package = "bluetape-rs-serialization" }

# 0.5.0 publish 후:
bluetape-rs-serialization = "0.5"
```

```rust
use bluetape_rs_serialization as serialization;
```

Root facade 사용:

```toml
# 0.5.0 publish 전:
bluetape-rs = { git = "https://github.com/bluetape4k/bluetape-rs", features = ["serialization"] }

# 0.5.0 publish 후:
bluetape-rs = { version = "0.5", features = ["serialization"] }
```

```rust
use bluetape_rs::serialization;
```

Root facade는 `serialization` feature를 켠 경우에만 사용할 수 있습니다. 기본
`bluetape-rs` build는 변경하지 않습니다.

이 bootstrap crate는 아직 serializer trait이나 adapter를 노출하지 않습니다. 해당
API는 review된 후속 `0.5.0` issue에서 추가합니다.

## 경계

`0.5.0`은 cache-first binary payload support로 시작합니다. Payload metadata,
versioning, trust profile, typed failure, adapter contract는 후속 issue에서
추가합니다.

`Option<T>`는 값의 부재를 의미합니다. Empty bytes는 payload data이며 숨겨진
null convention으로 취급하지 않습니다.

향후 unsupported version, wrong format, wrong trust profile은 typed decode
failure입니다. `None`으로 decode하지 않고, 조용히 fallback하지 않으며, alternate
adapter retry도 하지 않습니다. Cache eviction, namespace migration, rebuild
policy는 caller가 소유합니다.

## Migration / Compatibility

기존 `bluetape-rs` 사용자는 issue #108 때문에 code나 Cargo 설정을 바꿀 필요가
없습니다. 필요한 caller만 `bluetape-rs-serialization`에 직접 의존하거나
`bluetape-rs`에서 `features = ["serialization"]`을 활성화합니다.

## Issue #108 Bootstrap Non-goals

- Serializer trait 또는 concrete adapter
- Runtime binary payload encoding
- Global serializer registry

## `0.5.0` Core/Binary Milestone 범위 밖

- JSON adapter
- Protobuf adapter
- Avro adapter
- Apache Fory adapter
- Testcontainers integration
- SQL 또는 SQLx integration
- Resilience, retry, circuit-breaker, fallback policy
- Unsafe deserialization
- Hidden global serializer
- Hidden default serializer
- Env-selected adapter
- Dynamic type loading
- Schema registry support
