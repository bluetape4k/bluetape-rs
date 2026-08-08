# Serialization contract 설계

날짜: 2026-06-13
상태: Step 2-R 검토 완료, P0=0 P1=0
범위: 이슈 #109, milestone `0.5.0`, `crates/serialization`

## 문제

이슈 #108은 `bluetape-rs-serialization` crate 경계와 optional root facade를
생성했지만, crate는 아직 serialization contract를 노출하지 않습니다.
이슈 #109는 이후 `0.5.0` 작업이 기반으로 삼을 공개 vocabulary를 정의해야
합니다. 여기에는 명시적인 format identifier, typed error, payload metadata,
trust profile, 안전한 configuration default가 포함됩니다.

이 이슈는 첫 adapter 구현이 아닙니다. Binary, JSON, Protobuf, Avro, Fory
adapter는 이후 milestone 이슈에 남아 있습니다. #109의 결과물은 public API
형태를 다시 바꾸지 않고 해당 adapter 이슈를 구현할 수 있게 하는 작은
Rust-native contract layer입니다.

## 현재 근거

- 현재 `crates/serialization/src/lib.rs`에는 crate Rustdoc과 metadata smoke test만 있습니다. Serializer trait, payload envelope, adapter가 이후 검토 이슈라는 점도 명시되어 있습니다.
- `WIP.md`는 첫 binary adapter 전에 typed serializer/deserializer contract, typed error, format id, content type, version, trust profile vocabulary를 나열합니다.
- `docs/superpowers/specs/2026-06-13-serde-0-5x-design.md`는 `0.5.0`이 cache-first 및 binary-first이고 JSON, Protobuf, Avro, Fory, cross-repo benchmark가 별도 `0.5.x` milestone임을 명시합니다.
- 공식 Serde 문서는 data model trait인 `serde::Serialize` 및 `serde::Deserialize`를 구체적인 data format crate와 분리합니다. 일반적인 format crate 구조도 별도의 `ser`, `de`, `error` module로 제시합니다.
- Kotlin/JVM `bluetape4k-projects`에는 trust profile에 대한 유용한 domain 근거가 있지만, 이전 security review는 dynamic class loading, fallback object deserialization, allow-all default가 실제 deserialization 위험임을 보여줍니다.
- `bluetape-go`에는 `Serializer[T]`, `NamedSerializer[T]`, versioned envelope를 가진 작은 `serialization` package가 있습니다. Domain 근거로는 유용하지만, 이 Rust crate는 Go method name이나 interface shape를 기계적으로 포팅하지 않아야 합니다.
- Sibling Rust crate는 `lib.rs`를 간결하게 유지하고 구현을 목적별 module로 분리합니다. `crates/compression`은 `mod config`, `mod error`, `mod registry`, `mod stream`, `mod traits`를 사용하므로 #109도 `lib.rs`를 확장하는 대신 이 형태를 따라야 합니다.

## 목표

- Hidden global default 없이 명시적이고 typed인 format identifier를 정의합니다.
- `serde::Serialize` 및 `serde::de::DeserializeOwned`와 호환되는 typed serializer/deserializer contract를 정의합니다.
- Cache 및 infrastructure payload를 위한 binary payload metadata, 즉 content type, payload version, format id, trust profile, adapter id, payload size를 정의합니다.
- Encode, decode, config validation, format mismatch, content type mismatch, version mismatch, trust profile mismatch, malformed input, oversized payload, adapter failure에 대한 typed error enum을 정의합니다.
- Bluetape ecosystem에 맞는 trust profile vocabulary, 즉 trusted internal, allowlisted types, statically typed, unsafe legacy compatibility를 정의합니다.
- 근거를 문서화한 안전한 configuration default를 정의합니다.
- Byte/string 경계를 명시적으로 유지합니다.
- Compression, codec, serialization concern을 분리합니다.

## 범위 외

- #109에서 binary adapter 구현
- JSON, Protobuf, Avro, Fory, schema registry, cross-language production adapter
- Compression composition 구현
- Dynamic registry, hidden default serializer, environment-selected adapter, payload-selected Rust type
- Testcontainers, Redis, database, SQLx, resilience, benchmark harness
- Kotlin/JVM 또는 Go API parity 약속

## 제안 설계

승인된 B 접근법인 Rust-native typed contract module을 사용합니다.

`lib.rs`는 crate-level index 및 export surface로 유지합니다.

- `mod config;`
- `mod error;`
- `mod format;`
- `mod metadata;`
- `mod traits;`
- `mod trust;`

공개 export는 이러한 목적별 module에서 가져옵니다. 긴 설명은 `lib.rs`가
아니라 각 type의 Rustdoc과 README/spec text에 둡니다.

### Format vocabulary

`SerializationFormat`은 global registry가 아니라 작고 검증된 value type입니다.
`binary`, `json`, `protobuf`, `avro` 또는 이후 추가되는 adapter-specific id와
같은 안정적인 format id를 나타냅니다.

규칙:

- Format id는 ASCII lowercase token입니다.
- 허용 문자는 `a-z`, `0-9`, `-`, `_`, `.`, `/`입니다.
- Empty, blank, control character, uppercase, 과도하게 긴 id는 거부합니다.
- Type은 owned string을 저장하므로 이후 adapter가 enum을 바꾸지 않고도 안정적인 custom id를 정의할 수 있습니다.

이 방식은 모든 adapter에 대해 semver-visible 변경이 필요한 closed enum을
피하면서도 임의의 검증되지 않은 metadata를 막습니다.

### Trust profile

`SerializationTrustProfile`은 closed enum입니다.

- `TrustedInternal`: 하나의 신뢰된 deployment 내부 private cache 또는 queue
- `AllowListedTypes`: type metadata를 담을 수 있지만 명시적인 allowlist로 제한하는 format
- `StaticallyTyped`: payload가 runtime type을 선택하지 않고 호출자가 Rust target type을 제공
- `UnsafeLegacyCompatibility`: allow-all 또는 legacy compatibility 경로를 위한 명시적인 migration-only 경계

#109 default config는 `StaticallyTyped`를 사용합니다. Rust 호출자가
`DeserializeOwned`를 통해 target type을 제공해야 하며 `0.5.0`에서 dynamic
type loading을 활성화하면 안 되기 때문입니다.

### Metadata

`PayloadMetadata`는 payload byte를 노출하지 않고 payload를 설명합니다.

- `format: SerializationFormat`
- `content_type: ContentType`
- `version: PayloadVersion`
- `trust_profile: SerializationTrustProfile`
- `adapter_id: AdapterId`
- `payload_size: usize`

`ContentType`, `PayloadVersion`, `AdapterId`는 검증된 value type입니다. 모든
metadata token limit은 public contract의 일부이므로 구현과 test가 서로 다른
경계를 만들어내지 않습니다.

- Format id 길이는 1..=64 byte이고 lowercase ASCII만 사용하며 허용 byte는 `a-z`, `0-9`, `-`, `_`, `.`, `/`입니다.
- Content type 길이는 1..=127 byte이고 lowercase ASCII만 사용하며 정확히 하나의 `/`, parameter 없음, whitespace 없음, control 없음, `a-z`, `0-9`, `!`, `#`, `$`, `&`, `^`, `_`, `.`, `+`, `-`와 같은 visible media type token byte만 허용합니다.
- Payload version은 양의 `u16`입니다.
- Adapter id 길이는 1..=64 byte이고 lowercase ASCII만 사용하며 허용 byte는 `a-z`, `0-9`, `-`, `_`, `.`입니다.
- Payload size는 metadata일 뿐이며 payload byte를 log하거나 저장하지 않습니다.

`SerializedPayload`는 encoded byte와 metadata를 함께 소유합니다. Constructor는
`metadata.payload_size == bytes.len()`을 도출하거나 검증하므로 호출자가 실제
encoded payload와 다른 metadata를 publish할 수 없습니다.

### Trait

공개 trait는 format-agnostic이며 bytes-first입니다.

```rust
pub trait Serializer<T>
where
    T: serde::Serialize,
{
    fn serialize(&self, value: &T) -> Result<SerializedPayload, SerializationError>;
}

pub trait Deserializer<T>
where
    T: serde::de::DeserializeOwned,
{
    fn deserialize(&self, payload: &SerializedPayload) -> Result<T, SerializationError>;
    fn expected_metadata(&self) -> PayloadMetadataPolicy;
}

pub trait BinarySerializer<T>: Serializer<T> + Deserializer<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
}
```

정확한 plan에서 method name은 다듬을 수 있지만 다음 제약은 유지해야
합니다.

- Raw byte input은 `SerializedPayload::try_from_parts` 또는 decode 전에 metadata를 검증하는 동등한 constructor로 계속 사용할 수 있어야 합니다.
- Encode output은 `Vec<u8>` byte를 담은 owned `SerializedPayload`입니다.
- Input value는 encode에서 borrowed입니다.
- Decode target type은 Rust generic을 통해 호출자가 제공합니다.
- Metadata payload size는 `bytes.len()`에서 도출하거나 그 값과 대조합니다.
- #109 API에는 `Any`, `TypeId`, dynamic type registry, payload-selected type이 나타나지 않습니다.

`PayloadMetadataPolicy`는 adapter가 생기기 전에 deserializer가 기대하는
내용을 표현합니다. Deterministic matching rule을 정의해야 합니다.

- `format`과 `content_type`은 exact match입니다.
- `trust_profile`은 exact match입니다. 이후 adapter가 자체 이슈에서 더 좁은 호환성 규칙을 명시적으로 설계한 경우에만 예외를 둘 수 있습니다.
- `max_supported_version`은 inclusive입니다. 관측값 `0`은 malformed이고 max보다 큰 관측값은 unsupported-version error를 반환합니다.
- Adapter id matching은 선택적 policy metadata이며 dynamic registry lookup이 아닙니다.
- Mismatch는 format, content type, trust profile, version, malformed metadata, oversized payload에 해당하는 typed error variant로 매핑합니다.

### Error

Typed `SerializationError` enum을 사용합니다. Variant는 안전한 진단 정보를
보존해야 합니다.

- Encode 또는 decode 방향
- 관련된 경우 expected 및 observed format id
- 관련된 경우 expected 및 observed content type
- 관련된 경우 expected 및 observed payload version
- 관련된 경우 expected 및 observed trust profile
- 관련된 경우 adapter id
- 관련된 경우 payload size와 limit
- Payload byte가 없는 malformed-input reason
- 이후 adapter가 제공할 수 있는 경우 adapter failure의 source error

공개 error는 `std::error::Error`, `Display`, `Debug`를 구현하고 source가
연결된 경우 `Send + Sync`여야 합니다. Adapter failure source는 source를
안전하게 표시할 수 있을 때 `Box<dyn std::error::Error + Send + Sync + 'static>`과
동등한 stored shape를 사용해야 합니다. Upstream error가 raw payload byte나
snippet을 포함할 수 있으면 adapter는 `SerializationError`에 연결하기 전에
redacted source로 감싸야 합니다. `Display`, `Debug`, `source()` traversal
test는 식별 가능한 payload marker가 노출되지 않음을 증명해야 합니다.
Workspace가 이미 `crates/compression`에서 `thiserror`를 사용하므로 이를
사용해도 됩니다.

### Config default

`SerializationConfig`는 이후 adapter를 위한 안전한 default를 정의합니다.

- 기본 trust profile: `StaticallyTyped`
- 기본 content type: `application/octet-stream`
- 기본 payload version: `1`
- 기본 max payload size: `16 * 1024 * 1024` byte. `bytes.len() > max_payload_size`인 payload는 decode 전에 실패하고 정확히 같은 크기의 payload는 허용합니다.
- 구체적인 adapter에는 adapter id가 필요
- Fallback serializer 없음
- Hidden compression 없음

Config validation은 typed error를 반환하고 zero max size, zero payload
version, unsafe legacy compatibility default, blank adapter id, 잘못된
metadata token을 거부합니다.

### Dependency 정책

이슈 #109는 다음 dependency를 추가할 수 있습니다.

- Public trait bound를 위한 workspace dependency `serde`. `std` 지원을 사용하며 이 crate에서는 `derive` feature를 요구하지 않습니다.
- Typed error를 위한 `crates/serialization`의 `thiserror.workspace = true`

다음과 같은 adapter dependency는 추가하면 안 됩니다: `bincode`,
`serde_json`, `prost`, `apache-avro`, `fory`, Redis, Testcontainers,
compression adapter.

## 기각한 접근법

### Go `Serializer[T]` 직접 포팅

Go의 `Marshal`, `Unmarshal` name과 `NamedSerializer` interface는
`bluetape-go`에서 작고 검증되었지만, 직접 포팅하면 Rust의 `serde` trait
ecosystem과 ownership convention을 무시하게 됩니다. Rust API는 borrowed
encode input, 명시적인 `DeserializeOwned` decode target, Rust error type을
사용해야 합니다.

### 모든 format을 위한 closed enum

`enum SerializationFormat { Binary, Json, Protobuf, Avro, Fory }` 형태의
closed enum은 단순해 보이지만, 이후 adapter 또는 사용자 정의 format마다
공개 enum 변경이 필요합니다. 검증된 string newtype은 확장성을 유지하면서
안정적인 validation을 제공합니다.

### 지금 Dynamic registry 추가

Dynamic registry는 이후 adapter lookup을 편리하게 만들지만 hidden
default와 payload-selected 동작도 만듭니다. #109는 의도적으로 adapter
선택을 호출자가 소유하고 명시적으로 수행하도록 유지합니다.

## Cache rollout 및 Operator 지침

이슈 #109는 cache storage, cache key, cache eviction을 구현하지 않습니다.
그러나 metadata contract는 이후 cache 사용자가 format 및 version 변경을
안전하게 운영하는 방법을 알려야 합니다.

- `PayloadVersion`은 serialization payload contract를 설명하며 cache namespace 자체를 설명하지 않습니다.
- Format id, content type, trust profile, 호환되지 않는 payload version을 변경할 때 호출자는 cache namespace 또는 key prefix를 versioning해야 합니다.
- Mismatch는 안전한 metadata diagnostic과 함께 hard reject합니다. Contract는 조용히 decode하거나 다른 adapter로 fallback하거나 `None`을 반환해서는 안 됩니다.
- 권장 operator action을 명시합니다. Entry를 evict하고, source of truth에서 rebuild하며, namespace를 migrate하거나, 계획된 rollout 밖에서 mismatch가 나타나면 alert합니다.
- Rollback 동작을 명시합니다. 더 최신의 unsupported version을 본 older reader는 observed 및 maximum supported version metadata를 가진 typed unsupported-version error를 반환합니다.
- Observability field는 low-cardinality이고 payload-free여야 합니다. Error kind, direction, format id, content type, version relation, trust profile, adapter id, payload size bucket, configured size limit을 사용합니다. Raw payload byte와 unbounded payload snippet은 log, metric, trace field가 될 수 없습니다.
- `UnsafeLegacyCompatibility`는 완전히 신뢰할 수 있는 deployment만을 위한 임시 migration boundary이며 일반적인 production default가 아님을 문서화해야 합니다.

## 위험 및 실패 모드

1. **너무 넓은 공개 API.** #109가 지금 adapter-specific 동작을 정의하면 #111 및 이후 adapter 이슈가 잘못된 abstraction을 상속합니다. 완화책: Contract만 정의하고 adapter 구현은 하지 않습니다.
2. **Vocabulary drift로 인한 security regression.** Trust profile name이 Kotlin/JVM 문서와 달라지면 cross-repo 지침이 혼란스러워집니다. 완화책: 네 가지 bluetape trust profile을 Rust-native 이름으로 사용합니다.
3. **Format id의 semver trap.** Closed enum은 향후 format id를 enum variant에 강제로 넣습니다. 완화책: id에는 validated newtype을 사용합니다.
4. **숨겨진 payload leak.** Error message가 실수로 raw byte를 포함할 수 있습니다. 완화책: Error variant에는 metadata와 reason string만 저장하고, adapter source error는 표시해도 안전하거나 redaction한 경우에만 연결합니다.
5. **불명확한 cache migration semantics.** Version mismatch가 fallback decode를 의미할 수 있습니다. 완화책: #109는 mismatch를 typed error로 기록하고 namespace migration, eviction, rebuild, alert 경로를 호출자 소유 operator policy로 문서화합니다.

## Acceptance criteria

- `crates/serialization/src/lib.rs`는 간결하게 유지하고 목적별 module을 export합니다.
- Public contract가 Rust 2024에서 컴파일되고 `serde` 호환 bound를 사용합니다.
- Public value type이 format id, content type, adapter id, payload version을 검증합니다.
- Public error가 안전한 metadata context를 보존하며 payload byte를 노출하지 않습니다.
- Config default를 문서화하고 test로 검증합니다.
- Trust profile vocabulary가 trusted internal, allowlisted types, statically typed, unsafe legacy compatibility를 포함합니다.
- `SerializedPayload` 또는 동등한 공개 construction이 metadata `payload_size`가 encoded byte length와 달라지는 것을 막습니다.
- `PayloadMetadataPolicy`가 exact format/content/trust matching, inclusive max-version 동작, optional adapter-id matching, mismatch error mapping을 정의합니다.
- Format id, content type, adapter id의 max length와 허용 byte를 test로 검증합니다.
- Test가 valid default, invalid metadata token, version mismatch, format mismatch, content type mismatch, trust profile mismatch, payload size limit, config validation failure, metadata byte-length consistency, 안전한 `Display`/`Debug`/`source()` 동작을 다룹니다.
- Rustdoc에 config construction, adapter implementation 형태, encode/decode 사용, metadata policy validation, typed error matching을 위한 compile-checked example이 있습니다.
- `README.md`와 `README.ko.md`는 동기화 상태를 유지하고 #109가 contract만 추가하며 adapter는 이후 이슈이고 dynamic registry 또는 payload-selected type이 없으며 `UnsafeLegacyCompatibility`가 명시적인 migration-only vocabulary임을 설명합니다.
- Operator guidance가 mixed-version deploy, cache namespace/key-prefix versioning, mismatch action, rollback 동작, payload-free diagnostic field를 다룹니다.
- JSON, Protobuf, Avro, Fory, Testcontainers, Redis, SQLx, resilience, benchmark dependency를 추가하지 않습니다.

## 검증 계획

- `cargo fmt --all --check`
- `cargo test -p bluetape-rs-serialization --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `git diff --check`

## Step 2 DoD

| 항목 | 상태 |
|---|---|
| 이슈 #109 범위를 adapter 구현과 분리 | Required |
| Rust-native module 분할 지정 | Required |
| Serde 호환성 및 dependency 정책 지정 | Required |
| Trust profile vocabulary 지정 | Required |
| Typed metadata/error/config 요구 사항 지정 | Required |
| Payload size 일관성과 cache rollout 지침 지정 | Required |
| Test 및 검증 명령 지정 | Required |
