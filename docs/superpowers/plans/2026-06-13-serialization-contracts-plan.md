# 직렬화 계약 구현 계획

> **에이전트 작업자:** 필수 하위 스킬: 이 계획을 작업별로 구현하려면
> superpowers:subagent-driven-development(권장) 또는 superpowers:executing-plans를
> 사용한다. 단계 추적에는 checkbox(`- [ ]`) 문법을 사용한다.

**목표:** 구체적인 adapter를 추가하지 않고 이슈 #109에 Rust 네이티브
serialization contract API를 추가한다. 대상은 format id, trust profile, payload
metadata, typed error, config default 및 `serde` 호환 trait다.

**아키텍처:** `crates/serialization/src/lib.rs`는 간결한 export surface로
유지하고 공개 계약을 `format`, `trust`, `metadata`, `error`, `config`, `traits`
집중 모듈로 나눈다. Encode는 bytes와 metadata를 소유하는
`SerializedPayload`를 생성하므로 payload size가 `bytes.len()`과 달라질 수
없다. Decode는 `serde::de::DeserializeOwned`를 통해 호출자 타입을 유지하고,
adapter 선택은 registry나 fallback 없이 명시적으로 수행한다.

**기술 스택:** Rust 2024, `serde` 공개 trait bound, `thiserror` typed error,
Cargo workspace dependency, 컴파일 검사를 거치는 Rustdoc 예제,
`crates/serialization/tests` 아래 integration test.

---

## 파일 구조

- `Cargo.toml` 수정: workspace `serde = "1.0"`을 추가하고 adapter dependency는
  제외한다.
- `crates/serialization/Cargo.toml` 수정: `serde.workspace = true` 및
  `thiserror.workspace = true`를 추가하고 default feature는 비워 둔다.
- `crates/serialization/src/lib.rs` 수정: 간결한 module declaration과 public
  re-export만 둔다.
- `crates/serialization/src/format.rs` 생성: `SerializationFormat`,
  `ContentType`, `AdapterId`, `PayloadVersion`, validation constant, constructor,
  `Display`, `AsRef<str>`.
- `crates/serialization/src/trust.rs` 생성: `SerializationTrustProfile`과
  안전성 중심 Rustdoc.
- `crates/serialization/src/metadata.rs` 생성: `PayloadMetadata`,
  `SerializedPayload`, `PayloadMetadataPolicy`, payload-size 일관성 및
  메타데이터 정책 검증.
- `crates/serialization/src/error.rs` 생성: `SerializationError`,
  `SerializationErrorKind`, `SerializationOperation`, 우회할 수 없는
  adapter-source wrapper, 타입 지정 mismatch/config/malformed/limit variant.
- `crates/serialization/src/config.rs` 생성: `SerializationConfig`,
  `DEFAULT_MAX_PAYLOAD_SIZE`, 안전한 기본값 및 validation.
- `crates/serialization/src/traits.rs` 생성: `Serializer<T>`, `Deserializer<T>`,
  `BinarySerializer<T>`.
- `crates/serialization/tests/contracts.rs` 생성: 검증, 기본값, 메타데이터 정책,
  불일치 오류, payload-size 일관성 및 source redaction에 대한 public API 테스트.
- `crates/serialization/README.md` 수정: contract API, 지원하지 않는 adapter,
  dynamic registry 없음, payload-selected type 없음, cache rollout 지침을
  문서화한다.
- `crates/serialization/README.ko.md` 수정: 영어 README와 한국어 README를
  동기화한다.
- `README.md` 및 `README.ko.md` 수정: serialization package 행/지침을
  bootstrap-only에서 contracts-only로 갱신한다.

## 작업 1: Cargo dependency 경계

**복잡도:** 낮음
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 수정: `Cargo.toml`
- 수정: `crates/serialization/Cargo.toml`

- [ ] **단계 1: dependency 경계 기대치 작성**

편집 전에 실행한다.

```bash
rg -n 'serde|thiserror|bincode|serde_json|prost|apache-avro|fory|redis|testcontainers|sqlx' Cargo.toml crates/serialization/Cargo.toml
```

변경 전 예상 결과:

- root `Cargo.toml`에는 `thiserror`가 있지만 `serde`는 없다.
- `crates/serialization/Cargo.toml`에는 dependency가 없다.
- `crates/serialization`에는 adapter dependency가 없다.

- [ ] **단계 2: 허용된 workspace dependency만 추가**

root `Cargo.toml`의 `[workspace.dependencies]`에서 다른 third-party dependency
근처에 다음을 추가한다.

```toml
serde = "1.0"
```

`crates/serialization/Cargo.toml`의 빈 `[dependencies]` 섹션을 다음으로
교체한다.

```toml
[dependencies]
serde.workspace = true
thiserror.workspace = true

[dev-dependencies]
serde = { workspace = true, features = ["derive"] }
```

`serde_json`, `bincode`, `prost`, `apache-avro`, Fory, Redis, Testcontainers,
SQLx, compression 또는 resilience dependency는 추가하지 않는다.

- [ ] **단계 3: dependency 경계 검증**

실행한다.

```bash
cargo check -p bluetape-rs-serialization --all-features
rg -n 'serde_json|bincode|prost|apache-avro|fory|redis|testcontainers|sqlx|bluetape-rs-compression' crates/serialization Cargo.toml
```

예상 결과:

- `cargo check`가 성공하고 필요하면 `Cargo.lock`을 갱신한다.
- `rg`가 `crates/serialization`에서 adapter dependency를 반환하지 않는다.

## 작업 2: Validation newtype 및 trust profile

**복잡도:** 중간
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 생성: `crates/serialization/src/error.rs`
- 생성: `crates/serialization/src/format.rs`
- 생성: `crates/serialization/src/trust.rs`
- 생성: `crates/serialization/tests/contracts.rs`
- 수정: `crates/serialization/src/lib.rs`

- [ ] **단계 1: 유효·무효 metadata token의 실패 테스트 작성**

이 초기 테스트 집합으로 `crates/serialization/tests/contracts.rs`를 생성한다.

```rust
use bluetape_rs_serialization::{
    AdapterId, ContentType, PayloadVersion, SerializationFormat, SerializationTrustProfile,
    MAX_ADAPTER_ID_LEN, MAX_CONTENT_TYPE_LEN, MAX_FORMAT_ID_LEN,
};

#[test]
fn accepts_valid_metadata_tokens() {
    assert_eq!(SerializationFormat::new("binary").unwrap().as_str(), "binary");
    assert_eq!(
        SerializationFormat::new("custom.binary/v1").unwrap().as_str(),
        "custom.binary/v1"
    );
    assert_eq!(
        ContentType::new("application/octet-stream").unwrap().as_str(),
        "application/octet-stream"
    );
    assert_eq!(AdapterId::new("binary.primary").unwrap().as_str(), "binary.primary");
    assert_eq!(PayloadVersion::new(1).unwrap().get(), 1);
    assert_eq!(
        SerializationTrustProfile::default(),
        SerializationTrustProfile::StaticallyTyped
    );
}

#[test]
fn rejects_invalid_metadata_tokens() {
    assert!(SerializationFormat::new("").is_err());
    assert!(SerializationFormat::new("Binary").is_err());
    assert!(SerializationFormat::new("binary payload").is_err());
    assert!(SerializationFormat::new("x".repeat(MAX_FORMAT_ID_LEN + 1)).is_err());

    assert!(ContentType::new("").is_err());
    assert!(ContentType::new("application").is_err());
    assert!(ContentType::new("/json").is_err());
    assert!(ContentType::new("application/").is_err());
    assert!(ContentType::new("application/json; charset=utf-8").is_err());
    assert!(ContentType::new("application/\noctet-stream").is_err());
    assert!(ContentType::new("x/".to_owned() + &"y".repeat(MAX_CONTENT_TYPE_LEN)).is_err());

    assert!(AdapterId::new("").is_err());
    assert!(AdapterId::new("Binary").is_err());
    assert!(AdapterId::new("binary adapter").is_err());
    assert!(AdapterId::new("x".repeat(MAX_ADAPTER_ID_LEN + 1)).is_err());

    assert!(PayloadVersion::new(0).is_err());
}
```

- [ ] **단계 2: RED 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

RED 예상 결과:

- `AdapterId`, `ContentType`, `PayloadVersion`, `SerializationFormat`,
  `SerializationTrustProfile`이 정의되지 않아 테스트 컴파일이 실패한다.

- [ ] **단계 3: 최소 error scaffold, validation newtype 및 trust enum 구현**

Task 2 동안 token constructor가 크레이트 error type을 반환할 수 있도록 먼저
최소 `crates/serialization/src/error.rs` scaffold를 생성한다.

```rust
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SerializationErrorKind {
    InvalidMetadata,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SerializationError {
    #[error("invalid {field}: {reason}")]
    InvalidMetadata {
        field: &'static str,
        reason: &'static str,
    },
}

impl SerializationError {
    #[must_use]
    pub fn kind(&self) -> SerializationErrorKind {
        match self {
            Self::InvalidMetadata { .. } => SerializationErrorKind::InvalidMetadata,
        }
    }

    #[must_use]
    pub fn invalid_metadata(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidMetadata { field, reason }
    }
}
```

`crates/serialization/src/format.rs`를 생성한다.

```rust
use crate::SerializationError;
use std::fmt;

pub const MAX_FORMAT_ID_LEN: usize = 64;
pub const MAX_CONTENT_TYPE_LEN: usize = 127;
pub const MAX_ADAPTER_ID_LEN: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SerializationFormat(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentType(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AdapterId(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PayloadVersion(u16);

impl SerializationFormat {
    pub fn new(value: impl Into<String>) -> Result<Self, SerializationError> {
        let value = value.into();
        validate_token("format", &value, MAX_FORMAT_ID_LEN, is_format_byte)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ContentType {
    pub fn new(value: impl Into<String>) -> Result<Self, SerializationError> {
        let value = value.into();
        validate_token("content_type", &value, MAX_CONTENT_TYPE_LEN, is_content_type_byte)?;
        if value.bytes().filter(|byte| *byte == b'/').count() != 1 {
            return Err(SerializationError::invalid_metadata(
                "content_type",
                "content type must contain exactly one slash",
            ));
        }
        if value.starts_with('/') || value.ends_with('/') {
            return Err(SerializationError::invalid_metadata(
                "content_type",
                "content type must have non-empty type and subtype",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn octet_stream() -> Self {
        Self("application/octet-stream".to_owned())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AdapterId {
    pub fn new(value: impl Into<String>) -> Result<Self, SerializationError> {
        let value = value.into();
        validate_token("adapter_id", &value, MAX_ADAPTER_ID_LEN, is_adapter_id_byte)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PayloadVersion {
    pub fn new(version: u16) -> Result<Self, SerializationError> {
        if version == 0 {
            return Err(SerializationError::invalid_metadata(
                "version",
                "payload version must be positive",
            ));
        }
        Ok(Self(version))
    }

    #[must_use]
    pub fn get(self) -> u16 {
        self.0
    }
}

impl fmt::Display for SerializationFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for AdapterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn validate_token(
    field: &'static str,
    value: &str,
    max_len: usize,
    allowed: fn(u8) -> bool,
) -> Result<(), SerializationError> {
    if value.is_empty() {
        return Err(SerializationError::invalid_metadata(field, "value must not be empty"));
    }
    if value.len() > max_len {
        return Err(SerializationError::invalid_metadata(field, "value exceeds max length"));
    }
    if !value.is_ascii() || !value.bytes().all(allowed) {
        return Err(SerializationError::invalid_metadata(field, "value contains unsupported bytes"));
    }
    Ok(())
}

fn is_format_byte(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'/')
}

fn is_adapter_id_byte(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.')
}

fn is_content_type_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'a'..=b'z'
            | b'0'..=b'9'
            | b'!'
            | b'#'
            | b'$'
            | b'&'
            | b'^'
            | b'_'
            | b'.'
            | b'+'
            | b'-'
            | b'/'
    )
}
```

`crates/serialization/src/trust.rs`를 생성한다.

```rust
/// Describes how much serialized data can influence deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SerializationTrustProfile {
    /// Private caches or queues controlled by one trusted deployment boundary.
    TrustedInternal,
    /// Payloads may carry type metadata, but only through explicit allowlists.
    AllowListedTypes,
    /// Payloads do not select runtime types; callers provide the Rust target type.
    #[default]
    StaticallyTyped,
    /// Temporary migration-only boundary for fully trusted legacy data.
    UnsafeLegacyCompatibility,
}
```

모듈과 export를 추가하도록 `crates/serialization/src/lib.rs`를 수정한다.

```rust
mod error;
mod format;
mod trust;

pub use error::{SerializationError, SerializationErrorKind};
pub use format::{
    AdapterId, ContentType, PayloadVersion, SerializationFormat, MAX_ADAPTER_ID_LEN,
    MAX_CONTENT_TYPE_LEN, MAX_FORMAT_ID_LEN,
};
pub use trust::SerializationTrustProfile;
```

- [ ] **단계 4: token test GREEN 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

예상 결과:

- 최소 error scaffold로 metadata token test가 통과한다.

## 작업 3: Typed error 및 비식별화한 adapter source

**복잡도:** 중간
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 생성: `crates/serialization/src/error.rs`
- 수정: `crates/serialization/tests/contracts.rs`
- 수정: `crates/serialization/src/lib.rs`

- [ ] **단계 1: typed error 및 redaction 테스트 확장**

`crates/serialization/tests/contracts.rs`에 추가한다.

```rust
use bluetape_rs_serialization::{SerializationError, SerializationErrorKind};
use bluetape_rs_serialization::SerializationOperation;
use std::error::Error;

#[derive(Debug)]
struct PayloadLeakingError;

impl std::fmt::Display for PayloadLeakingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SECRET_PAYLOAD_MARKER")
    }
}

impl Error for PayloadLeakingError {}

#[test]
fn reports_mismatches_without_payload_bytes() {
    let expected = SerializationFormat::new("binary").unwrap();
    let observed = SerializationFormat::new("json").unwrap();
    let error = SerializationError::format_mismatch(
        SerializationOperation::Deserialize,
        expected,
        observed,
    );

    assert_eq!(error.kind(), SerializationErrorKind::FormatMismatch);
    assert_eq!(error.operation(), Some(SerializationOperation::Deserialize));
    assert!(error.to_string().contains("format mismatch"));
    assert!(!error.to_string().contains("SECRET_PAYLOAD_MARKER"));
}

#[test]
fn safe_adapter_source_errors_preserve_cause_chain() {
    #[derive(Debug)]
    struct SafeAdapterError;

    impl std::fmt::Display for SafeAdapterError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("safe adapter diagnostic")
        }
    }

    impl Error for SafeAdapterError {}

    let adapter = AdapterId::new("binary.primary").unwrap();
    let error = SerializationError::safe_adapter_failure(
        adapter,
        SerializationOperation::Deserialize,
        SafeAdapterError,
    );

    assert_eq!(error.kind(), SerializationErrorKind::AdapterFailure);
    assert_eq!(error.operation(), Some(SerializationOperation::Deserialize));
    assert_eq!(
        error.source().map(ToString::to_string).unwrap_or_default(),
        "safe adapter diagnostic"
    );
}

#[test]
fn adapter_source_errors_are_redacted_when_needed() {
    let adapter = AdapterId::new("binary.primary").unwrap();
    let error = SerializationError::redacted_adapter_failure(
        adapter,
        SerializationOperation::Deserialize,
        PayloadLeakingError,
    );

    assert_eq!(error.kind(), SerializationErrorKind::AdapterFailure);
    assert!(!error.to_string().contains("SECRET_PAYLOAD_MARKER"));
    assert!(!format!("{error:?}").contains("SECRET_PAYLOAD_MARKER"));
    assert!(
        !error
            .source()
            .map(ToString::to_string)
            .unwrap_or_default()
            .contains("SECRET_PAYLOAD_MARKER")
    );
}
```

- [ ] **단계 2: RED 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

RED 예상 결과:

- error constructor와 variant가 구현되지 않아 컴파일이 실패한다.

- [ ] **단계 3: `SerializationError` 구현**

다음 내용으로 `crates/serialization/src/error.rs`를 생성한다.

```rust
use crate::{AdapterId, ContentType, PayloadVersion, SerializationFormat, SerializationTrustProfile};
use std::error::Error as StdError;
use thiserror::Error;

type AdapterSource = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SerializationOperation {
    Serialize,
    Deserialize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SerializationErrorKind {
    InvalidMetadata,
    InvalidConfig,
    FormatMismatch,
    ContentTypeMismatch,
    UnsupportedVersion,
    TrustProfileMismatch,
    AdapterIdMismatch,
    PayloadSizeLimitExceeded,
    MalformedInput,
    AdapterFailure,
}

#[derive(Debug)]
pub struct AdapterFailureSource {
    source: AdapterSource,
}

impl AdapterFailureSource {
    #[must_use]
    pub fn safe<E>(source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            source: Box::new(source),
        }
    }

    #[must_use]
    pub fn redacted<E>(_source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            source: Box::new(RedactedAdapterSource),
        }
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SerializationError {
    #[error("invalid {field}: {reason}")]
    InvalidMetadata {
        field: &'static str,
        reason: &'static str,
    },
    #[error("invalid serialization config {field}: {reason}")]
    InvalidConfig {
        field: &'static str,
        reason: &'static str,
    },
    #[error("serialization {operation:?} format mismatch: expected {expected}, observed {observed}")]
    FormatMismatch {
        operation: SerializationOperation,
        expected: SerializationFormat,
        observed: SerializationFormat,
    },
    #[error("serialization {operation:?} content type mismatch: expected {expected}, observed {observed}")]
    ContentTypeMismatch {
        operation: SerializationOperation,
        expected: ContentType,
        observed: ContentType,
    },
    #[error("unsupported serialization {operation:?} payload version: observed {observed}, max supported {max_supported}")]
    UnsupportedVersion {
        operation: SerializationOperation,
        max_supported: PayloadVersion,
        observed: PayloadVersion,
    },
    #[error("serialization {operation:?} trust profile mismatch: expected {expected:?}, observed {observed:?}")]
    TrustProfileMismatch {
        operation: SerializationOperation,
        expected: SerializationTrustProfile,
        observed: SerializationTrustProfile,
    },
    #[error("serialization {operation:?} adapter id mismatch: expected {expected}, observed {observed}")]
    AdapterIdMismatch {
        operation: SerializationOperation,
        expected: AdapterId,
        observed: AdapterId,
    },
    #[error("serialization {operation:?} payload size {actual} exceeded limit {limit}")]
    PayloadSizeLimitExceeded {
        operation: SerializationOperation,
        limit: usize,
        actual: usize,
    },
    #[error("malformed serialization {operation:?} input from {adapter_id}: {reason}")]
    MalformedInput {
        operation: SerializationOperation,
        adapter_id: AdapterId,
        reason: &'static str,
    },
    #[error("serialization {operation:?} failed in adapter {adapter_id}")]
    AdapterFailure {
        adapter_id: AdapterId,
        operation: SerializationOperation,
        #[source]
        source: AdapterFailureSource,
    },
}

impl SerializationError {
    #[must_use]
    pub fn kind(&self) -> SerializationErrorKind {
        match self {
            Self::InvalidMetadata { .. } => SerializationErrorKind::InvalidMetadata,
            Self::InvalidConfig { .. } => SerializationErrorKind::InvalidConfig,
            Self::FormatMismatch { .. } => SerializationErrorKind::FormatMismatch,
            Self::ContentTypeMismatch { .. } => SerializationErrorKind::ContentTypeMismatch,
            Self::UnsupportedVersion { .. } => SerializationErrorKind::UnsupportedVersion,
            Self::TrustProfileMismatch { .. } => SerializationErrorKind::TrustProfileMismatch,
            Self::AdapterIdMismatch { .. } => SerializationErrorKind::AdapterIdMismatch,
            Self::PayloadSizeLimitExceeded { .. } => SerializationErrorKind::PayloadSizeLimitExceeded,
            Self::MalformedInput { .. } => SerializationErrorKind::MalformedInput,
            Self::AdapterFailure { .. } => SerializationErrorKind::AdapterFailure,
        }
    }

    #[must_use]
    pub fn operation(&self) -> Option<SerializationOperation> {
        match self {
            Self::InvalidMetadata { .. } | Self::InvalidConfig { .. } => None,
            Self::FormatMismatch { operation, .. }
            | Self::ContentTypeMismatch { operation, .. }
            | Self::UnsupportedVersion { operation, .. }
            | Self::TrustProfileMismatch { operation, .. }
            | Self::AdapterIdMismatch { operation, .. }
            | Self::PayloadSizeLimitExceeded { operation, .. }
            | Self::MalformedInput { operation, .. }
            | Self::AdapterFailure { operation, .. } => Some(*operation),
        }
    }

    #[must_use]
    pub fn invalid_metadata(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidMetadata { field, reason }
    }

    #[must_use]
    pub fn invalid_config(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidConfig { field, reason }
    }

    #[must_use]
    pub fn format_mismatch(
        operation: SerializationOperation,
        expected: SerializationFormat,
        observed: SerializationFormat,
    ) -> Self {
        Self::FormatMismatch {
            operation,
            expected,
            observed,
        }
    }

    #[must_use]
    pub fn content_type_mismatch(
        operation: SerializationOperation,
        expected: ContentType,
        observed: ContentType,
    ) -> Self {
        Self::ContentTypeMismatch {
            operation,
            expected,
            observed,
        }
    }

    #[must_use]
    pub fn unsupported_version(
        operation: SerializationOperation,
        max_supported: PayloadVersion,
        observed: PayloadVersion,
    ) -> Self {
        Self::UnsupportedVersion {
            operation,
            max_supported,
            observed,
        }
    }

    #[must_use]
    pub fn trust_profile_mismatch(
        operation: SerializationOperation,
        expected: SerializationTrustProfile,
        observed: SerializationTrustProfile,
    ) -> Self {
        Self::TrustProfileMismatch {
            operation,
            expected,
            observed,
        }
    }

    #[must_use]
    pub fn adapter_id_mismatch(
        operation: SerializationOperation,
        expected: AdapterId,
        observed: AdapterId,
    ) -> Self {
        Self::AdapterIdMismatch {
            operation,
            expected,
            observed,
        }
    }

    #[must_use]
    pub fn payload_size_limit_exceeded(
        operation: SerializationOperation,
        limit: usize,
        actual: usize,
    ) -> Self {
        Self::PayloadSizeLimitExceeded {
            operation,
            limit,
            actual,
        }
    }

    #[must_use]
    pub fn safe_adapter_failure<E>(
        adapter_id: AdapterId,
        operation: SerializationOperation,
        source: E,
    ) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::AdapterFailure {
            adapter_id,
            operation,
            source: AdapterFailureSource::safe(source),
        }
    }

    #[must_use]
    pub fn redacted_adapter_failure<E>(
        adapter_id: AdapterId,
        operation: SerializationOperation,
        _source: E,
    ) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::AdapterFailure {
            adapter_id,
            operation,
            source: AdapterFailureSource::redacted(_source),
        }
    }

    #[must_use]
    pub fn malformed_input(
        operation: SerializationOperation,
        adapter_id: AdapterId,
        reason: &'static str,
    ) -> Self {
        Self::MalformedInput {
            operation,
            adapter_id,
            reason,
        }
    }
}

#[derive(Debug)]
struct RedactedAdapterSource;

impl std::fmt::Display for RedactedAdapterSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("adapter source redacted")
    }
}

impl StdError for RedactedAdapterSource {}

impl std::fmt::Display for AdapterFailureSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(f)
    }
}

impl StdError for AdapterFailureSource {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.as_ref())
    }
}
```

raw `AdapterSource` alias나 raw `Box<dyn Error>` source field를 public API에
노출하지 않는다. 공개 adapter failure는 명시적인 safe-source 또는
redacted-source constructor로만 생성해야 하며, Step 6-R에서 unwrapped raw
adapter source를 받는 public API가 없는지 확인해야 한다.

전체 error 구현 후 `crates/serialization/src/lib.rs`의 error export를 수정한다.

```rust
pub use error::{
    AdapterFailureSource, SerializationError, SerializationErrorKind, SerializationOperation,
};
```

- [ ] **단계 4: error test GREEN 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

예상 결과:

- token 및 error test가 통과한다.

## 작업 4: Config, metadata 및 policy 계약

**복잡도:** 높음
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 생성: `crates/serialization/src/config.rs`
- 생성: `crates/serialization/src/metadata.rs`
- 수정: `crates/serialization/src/lib.rs`
- 수정: `crates/serialization/tests/contracts.rs`

- [ ] **단계 1: config default 및 payload 일관성 실패 테스트 작성**

`crates/serialization/tests/contracts.rs`에 추가한다.

```rust
use bluetape_rs_serialization::{
    DEFAULT_MAX_PAYLOAD_SIZE, PayloadMetadata, PayloadMetadataPolicy, SerializedPayload,
    SerializationConfig, SerializationOperation,
};

#[test]
fn config_defaults_are_safe_and_explicit() {
    let config = SerializationConfig::new(
        SerializationFormat::new("binary").unwrap(),
        AdapterId::new("binary.primary").unwrap(),
    )
    .unwrap();

    assert_eq!(config.format().as_str(), "binary");
    assert_eq!(config.content_type().as_str(), "application/octet-stream");
    assert_eq!(config.version().get(), 1);
    assert_eq!(config.trust_profile(), SerializationTrustProfile::StaticallyTyped);
    assert_eq!(config.max_payload_size(), DEFAULT_MAX_PAYLOAD_SIZE);
}

#[test]
fn config_rejects_unsafe_defaults_and_zero_limits() {
    let config = SerializationConfig::new(
        SerializationFormat::new("binary").unwrap(),
        AdapterId::new("binary.primary").unwrap(),
    )
    .unwrap();

    assert!(config.clone().with_max_payload_size(0).is_err());
    let legacy = config.with_unsafe_legacy_compatibility_for_migration();
    assert_eq!(
        legacy.trust_profile(),
        SerializationTrustProfile::UnsafeLegacyCompatibility
    );
}

#[test]
fn serialized_payload_derives_size_from_bytes() {
    let metadata = SerializationConfig::new(
        SerializationFormat::new("binary").unwrap(),
        AdapterId::new("binary.primary").unwrap(),
    )
    .unwrap()
    .metadata_for_size(3)
    .unwrap();

    let payload = SerializedPayload::new(vec![1, 2, 3], metadata).unwrap();
    assert_eq!(payload.bytes(), &[1, 2, 3]);
    assert_eq!(payload.metadata().payload_size, 3);
}

#[test]
fn serialized_payload_rejects_metadata_size_mismatch() {
    let metadata = SerializationConfig::new(
        SerializationFormat::new("binary").unwrap(),
        AdapterId::new("binary.primary").unwrap(),
    )
    .unwrap()
    .metadata_for_size(9)
    .unwrap();

    let error = SerializedPayload::new(vec![1, 2, 3], metadata).unwrap_err();
    assert_eq!(error.kind(), SerializationErrorKind::InvalidMetadata);
    assert_eq!(error.operation(), None);
}

#[test]
fn metadata_policy_rejects_mismatches() {
    let config = SerializationConfig::new(
        SerializationFormat::new("binary").unwrap(),
        AdapterId::new("binary.primary").unwrap(),
    )
    .unwrap();
    let metadata = config.metadata_for_size(10).unwrap();
    let policy = PayloadMetadataPolicy::from_config(&config);

    assert!(policy.validate(&metadata).is_ok());

    let wrong_format = PayloadMetadata::new(
        SerializationFormat::new("json").unwrap(),
        metadata.content_type().clone(),
        metadata.version(),
        metadata.trust_profile(),
        metadata.adapter_id().clone(),
        metadata.payload_size(),
    );
    assert_eq!(
        policy.validate(&wrong_format).unwrap_err().kind(),
        SerializationErrorKind::FormatMismatch
    );

    let wrong_content_type = PayloadMetadata::new(
        metadata.format().clone(),
        ContentType::new("application/json").unwrap(),
        metadata.version(),
        metadata.trust_profile(),
        metadata.adapter_id().clone(),
        metadata.payload_size(),
    );
    assert_eq!(
        policy.validate(&wrong_content_type).unwrap_err().kind(),
        SerializationErrorKind::ContentTypeMismatch
    );

    let wrong_trust_profile = PayloadMetadata::new(
        metadata.format().clone(),
        metadata.content_type().clone(),
        metadata.version(),
        SerializationTrustProfile::TrustedInternal,
        metadata.adapter_id().clone(),
        metadata.payload_size(),
    );
    assert_eq!(
        policy.validate(&wrong_trust_profile).unwrap_err().kind(),
        SerializationErrorKind::TrustProfileMismatch
    );

    let wrong_adapter = PayloadMetadata::new(
        metadata.format().clone(),
        metadata.content_type().clone(),
        metadata.version(),
        metadata.trust_profile(),
        AdapterId::new("binary.secondary").unwrap(),
        metadata.payload_size(),
    );
    assert_eq!(
        policy.validate(&wrong_adapter).unwrap_err().kind(),
        SerializationErrorKind::AdapterIdMismatch
    );

    let adapter_wildcard = PayloadMetadataPolicy::from_parts(
        config.format().clone(),
        config.content_type().clone(),
        config.version(),
        config.trust_profile(),
        None,
        config.max_payload_size(),
    )
    .unwrap();
    assert!(adapter_wildcard.validate(&wrong_adapter).is_ok());
}

#[test]
fn metadata_policy_enforces_version_and_size_boundaries() {
    let config = SerializationConfig::new(
        SerializationFormat::new("binary").unwrap(),
        AdapterId::new("binary.primary").unwrap(),
    )
    .unwrap()
    .with_max_payload_size(10)
    .unwrap();
    let policy = PayloadMetadataPolicy::from_config(&config);

    let exact_limit = config.metadata_for_size(10).unwrap();
    assert!(policy.validate(&exact_limit).is_ok());

    let oversized = PayloadMetadata::new(
        exact_limit.format().clone(),
        exact_limit.content_type().clone(),
        exact_limit.version(),
        exact_limit.trust_profile(),
        exact_limit.adapter_id().clone(),
        11,
    );
    assert_eq!(
        policy.validate(&oversized).unwrap_err().kind(),
        SerializationErrorKind::PayloadSizeLimitExceeded
    );

    let newer_version = PayloadMetadata::new(
        exact_limit.format().clone(),
        exact_limit.content_type().clone(),
        PayloadVersion::new(2).unwrap(),
        exact_limit.trust_profile(),
        exact_limit.adapter_id().clone(),
        exact_limit.payload_size(),
    );
    let error = policy.validate(&newer_version).unwrap_err();
    assert_eq!(error.kind(), SerializationErrorKind::UnsupportedVersion);
    assert_eq!(error.operation(), Some(SerializationOperation::Deserialize));
}
```

- [ ] **단계 2: RED 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

RED 예상 결과:

- `SerializationConfig`, `PayloadMetadataPolicy`, `SerializedPayload`가
  정의되지 않아 컴파일이 실패한다.

- [ ] **단계 3: config 및 metadata 구현**

다음 내용으로 `crates/serialization/src/metadata.rs`를 생성한다.

```rust
use crate::{
    AdapterId, ContentType, PayloadVersion, SerializationError, SerializationFormat,
    SerializationOperation, SerializationTrustProfile,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PayloadMetadata {
    format: SerializationFormat,
    content_type: ContentType,
    version: PayloadVersion,
    trust_profile: SerializationTrustProfile,
    adapter_id: AdapterId,
    payload_size: usize,
}

impl PayloadMetadata {
    #[must_use]
    pub fn new(
        format: SerializationFormat,
        content_type: ContentType,
        version: PayloadVersion,
        trust_profile: SerializationTrustProfile,
        adapter_id: AdapterId,
        payload_size: usize,
    ) -> Self {
        Self {
            format,
            content_type,
            version,
            trust_profile,
            adapter_id,
            payload_size,
        }
    }

    #[must_use]
    pub fn format(&self) -> &SerializationFormat {
        &self.format
    }

    #[must_use]
    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    #[must_use]
    pub fn version(&self) -> PayloadVersion {
        self.version
    }

    #[must_use]
    pub fn trust_profile(&self) -> SerializationTrustProfile {
        self.trust_profile
    }

    #[must_use]
    pub fn adapter_id(&self) -> &AdapterId {
        &self.adapter_id
    }

    #[must_use]
    pub fn payload_size(&self) -> usize {
        self.payload_size
    }
}

#[derive(PartialEq, Eq)]
pub struct SerializedPayload {
    bytes: Vec<u8>,
    metadata: PayloadMetadata,
}

impl SerializedPayload {
    pub fn new(bytes: Vec<u8>, metadata: PayloadMetadata) -> Result<Self, SerializationError> {
        if bytes.len() != metadata.payload_size() {
            return Err(SerializationError::invalid_metadata(
                "payload_size",
                "metadata payload size must match byte length",
            ));
        }
        Ok(Self { bytes, metadata })
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    #[must_use]
    pub fn metadata(&self) -> &PayloadMetadata {
        &self.metadata
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PayloadMetadataPolicy {
    format: SerializationFormat,
    content_type: ContentType,
    max_supported_version: PayloadVersion,
    trust_profile: SerializationTrustProfile,
    adapter_id: Option<AdapterId>,
    max_payload_size: usize,
}

impl PayloadMetadataPolicy {
    pub fn from_parts(
        format: SerializationFormat,
        content_type: ContentType,
        max_supported_version: PayloadVersion,
        trust_profile: SerializationTrustProfile,
        adapter_id: Option<AdapterId>,
        max_payload_size: usize,
    ) -> Result<Self, SerializationError> {
        if max_payload_size == 0 {
            return Err(SerializationError::invalid_config(
                "max_payload_size",
                "max payload size must be positive",
            ));
        }
        Ok(Self::new_unchecked(
            format,
            content_type,
            max_supported_version,
            trust_profile,
            adapter_id,
            max_payload_size,
        ))
    }

    pub(crate) fn new_unchecked(
        format: SerializationFormat,
        content_type: ContentType,
        max_supported_version: PayloadVersion,
        trust_profile: SerializationTrustProfile,
        adapter_id: Option<AdapterId>,
        max_payload_size: usize,
    ) -> Self {
        Self {
            format,
            content_type,
            max_supported_version,
            trust_profile,
            adapter_id,
            max_payload_size,
        }
    }

    pub fn validate(&self, metadata: &PayloadMetadata) -> Result<(), SerializationError> {
        if metadata.format != self.format {
            return Err(SerializationError::format_mismatch(
                SerializationOperation::Deserialize,
                self.format.clone(),
                metadata.format.clone(),
            ));
        }
        if metadata.content_type != self.content_type {
            return Err(SerializationError::content_type_mismatch(
                SerializationOperation::Deserialize,
                self.content_type.clone(),
                metadata.content_type.clone(),
            ));
        }
        if metadata.version > self.max_supported_version {
            return Err(SerializationError::unsupported_version(
                SerializationOperation::Deserialize,
                self.max_supported_version,
                metadata.version,
            ));
        }
        if metadata.trust_profile != self.trust_profile {
            return Err(SerializationError::trust_profile_mismatch(
                SerializationOperation::Deserialize,
                self.trust_profile,
                metadata.trust_profile,
            ));
        }
        if let Some(expected) = &self.adapter_id {
            if metadata.adapter_id != *expected {
                return Err(SerializationError::adapter_id_mismatch(
                    SerializationOperation::Deserialize,
                    expected.clone(),
                    metadata.adapter_id.clone(),
                ));
            }
        }
        if metadata.payload_size > self.max_payload_size {
            return Err(SerializationError::payload_size_limit_exceeded(
                SerializationOperation::Deserialize,
                self.max_payload_size,
                metadata.payload_size,
            ));
        }
        Ok(())
    }
}
```

다음 내용으로 `crates/serialization/src/config.rs`를 생성한다.

```rust
use crate::{
    AdapterId, ContentType, PayloadMetadata, PayloadMetadataPolicy, PayloadVersion,
    SerializationError, SerializationFormat, SerializationOperation, SerializationTrustProfile,
};

pub const DEFAULT_MAX_PAYLOAD_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializationConfig {
    format: SerializationFormat,
    content_type: ContentType,
    version: PayloadVersion,
    trust_profile: SerializationTrustProfile,
    adapter_id: AdapterId,
    max_payload_size: usize,
}

impl SerializationConfig {
    pub fn new(
        format: SerializationFormat,
        adapter_id: AdapterId,
    ) -> Result<Self, SerializationError> {
        Ok(Self {
            format,
            content_type: ContentType::octet_stream(),
            version: PayloadVersion::new(1)?,
            trust_profile: SerializationTrustProfile::StaticallyTyped,
            adapter_id,
            max_payload_size: DEFAULT_MAX_PAYLOAD_SIZE,
        })
    }

    pub fn with_max_payload_size(mut self, max_payload_size: usize) -> Result<Self, SerializationError> {
        if max_payload_size == 0 {
            return Err(SerializationError::invalid_config(
                "max_payload_size",
                "max payload size must be positive",
            ));
        }
        self.max_payload_size = max_payload_size;
        Ok(self)
    }

    pub fn with_trust_profile(
        mut self,
        trust_profile: SerializationTrustProfile,
    ) -> Result<Self, SerializationError> {
        if trust_profile == SerializationTrustProfile::UnsafeLegacyCompatibility {
            return Err(SerializationError::invalid_config(
                "trust_profile",
                "unsafe legacy compatibility cannot be a default config",
            ));
        }
        self.trust_profile = trust_profile;
        Ok(self)
    }

    /// Explicitly opts into the migration-only unsafe legacy trust profile.
    pub fn with_unsafe_legacy_compatibility_for_migration(mut self) -> Self {
        self.trust_profile = SerializationTrustProfile::UnsafeLegacyCompatibility;
        self
    }

    pub fn metadata_for_size(&self, payload_size: usize) -> Result<PayloadMetadata, SerializationError> {
        if payload_size > self.max_payload_size {
            return Err(SerializationError::payload_size_limit_exceeded(
                SerializationOperation::Serialize,
                self.max_payload_size,
                payload_size,
            ));
        }
        Ok(PayloadMetadata::new(
            self.format.clone(),
            self.content_type.clone(),
            self.version,
            self.trust_profile,
            self.adapter_id.clone(),
            payload_size,
        ))
    }

    #[must_use]
    pub fn metadata_policy(&self) -> PayloadMetadataPolicy {
        PayloadMetadataPolicy::from_config(self)
    }

    #[must_use]
    pub fn format(&self) -> &SerializationFormat {
        &self.format
    }

    #[must_use]
    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    #[must_use]
    pub fn version(&self) -> PayloadVersion {
        self.version
    }

    #[must_use]
    pub fn trust_profile(&self) -> SerializationTrustProfile {
        self.trust_profile
    }

    #[must_use]
    pub fn max_payload_size(&self) -> usize {
        self.max_payload_size
    }
}

impl PayloadMetadataPolicy {
    #[must_use]
    pub fn from_config(config: &SerializationConfig) -> Self {
        Self::new_unchecked(
            config.format.clone(),
            config.content_type.clone(),
            config.version,
            config.trust_profile,
            Some(config.adapter_id.clone()),
            config.max_payload_size,
        )
    }
}
```

`crates/serialization/src/lib.rs`를 수정한다.

```rust
mod config;
mod metadata;

pub use config::{DEFAULT_MAX_PAYLOAD_SIZE, SerializationConfig};
pub use metadata::{PayloadMetadata, PayloadMetadataPolicy, SerializedPayload};
```

- [ ] **단계 4: config 및 metadata test GREEN 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

예상 결과:

- config, metadata 및 policy test가 통과한다.

## 작업 5: Serde 호환 trait 및 Rustdoc 예제

**복잡도:** 중간
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 생성: `crates/serialization/src/traits.rs`
- 수정: `crates/serialization/src/lib.rs`
- 수정: `crates/serialization/src/config.rs`
- 수정: `crates/serialization/src/metadata.rs`
- 수정: `crates/serialization/tests/contracts.rs`

- [ ] **단계 1: trait 구현 형태의 실패 테스트 작성**

`crates/serialization/tests/contracts.rs`에 추가한다.

```rust
use bluetape_rs_serialization::{BinarySerializer, Deserializer, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Account {
    id: String,
}

struct EchoSerializer {
    config: SerializationConfig,
}

impl EchoSerializer {
    fn new() -> Self {
        Self {
            config: SerializationConfig::new(
                SerializationFormat::new("binary").unwrap(),
                AdapterId::new("echo").unwrap(),
            )
            .unwrap(),
        }
    }
}

impl Serializer<Account> for EchoSerializer {
    fn serialize(&self, value: &Account) -> Result<SerializedPayload, SerializationError> {
        let bytes = value.id.as_bytes().to_vec();
        let metadata = self.config.metadata_for_size(bytes.len())?;
        SerializedPayload::new(bytes, metadata)
    }
}

impl Deserializer<Account> for EchoSerializer {
    fn deserialize(&self, payload: &SerializedPayload) -> Result<Account, SerializationError> {
        self.expected_metadata().validate(payload.metadata())?;
        let id = std::str::from_utf8(payload.bytes())
            .map_err(|_| {
                SerializationError::malformed_input(
                    SerializationOperation::Deserialize,
                    AdapterId::new("echo").unwrap(),
                    "payload must be valid utf-8",
                )
            })?
            .to_owned();
        Ok(Account { id })
    }

    fn expected_metadata(&self) -> PayloadMetadataPolicy {
        self.config.metadata_policy()
    }
}

#[test]
fn traits_round_trip_with_caller_supplied_type() {
    fn assert_binary_serializer<T, S>(serializer: &S, value: &T) -> T
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
        S: BinarySerializer<T>,
    {
        let payload = serializer.serialize(value).unwrap();
        serializer.deserialize(&payload).unwrap()
    }

    let serializer = EchoSerializer::new();
    let value = Account {
        id: "acct-1".to_owned(),
    };

    assert_eq!(assert_binary_serializer(&serializer, &value), value);
}
```

- [ ] **단계 2: RED 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

RED 예상 결과:

- trait와 `serde` derive dev usage가 연결되지 않아 컴파일이 실패한다.

- [ ] **단계 3: trait 구현**

`crates/serialization/src/traits.rs`를 생성한다.

```rust
use crate::{PayloadMetadataPolicy, SerializedPayload, SerializationError};
use serde::{Serialize, de::DeserializeOwned};

pub trait Serializer<T>
where
    T: Serialize,
{
    fn serialize(&self, value: &T) -> Result<SerializedPayload, SerializationError>;
}

pub trait Deserializer<T>
where
    T: DeserializeOwned,
{
    fn deserialize(&self, payload: &SerializedPayload) -> Result<T, SerializationError>;

    fn expected_metadata(&self) -> PayloadMetadataPolicy;
}

pub trait BinarySerializer<T>: Serializer<T> + Deserializer<T>
where
    T: Serialize + DeserializeOwned,
{
}

impl<T, S> BinarySerializer<T> for S
where
    T: Serialize + DeserializeOwned,
    S: Serializer<T> + Deserializer<T>,
{
}
```

`crates/serialization/src/lib.rs`를 수정한다.

```rust
mod traits;

pub use traits::{BinarySerializer, Deserializer, Serializer};
```

derive macro dev dependency는 작업 1에서 추가했다. production dependency는
derive를 요구하지 않는 `serde.workspace = true`로 유지한다.

- [ ] **단계 4: trait test GREEN 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

예상 결과:

- trait round-trip test가 통과한다.

- [ ] **단계 5: 컴파일 검사 Rustdoc 예제 추가**

다음 공개 constructor와 method에 `# Examples` 및 `# Errors` Rustdoc을
추가한다.

- `SerializationFormat::new`
- `ContentType::new`
- `AdapterId::new`
- `PayloadVersion::new`
- `SerializationConfig::new`
- `SerializationConfig::metadata_for_size`
- `SerializedPayload::new`
- `PayloadMetadataPolicy::validate`
- `Serializer`
- `Deserializer`
- `BinarySerializer`

각 예제는 doctest에서 컴파일되어야 한다. 문서를 간결하게 유지하기 위해
예제에서 `#` hidden setup line을 사용할 수 있다. `DeserializeOwned`를
만족하지 못하는 borrowed decode target이 거부됨을 보여 주는 `compile_fail`
Rustdoc 예제를 하나 포함한다.

- [ ] **단계 6: 문서 컴파일 검증**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --doc --all-features --locked
RUSTDOCFLAGS="-D warnings" cargo doc -p bluetape-rs-serialization --all-features --no-deps --locked
```

예상 결과:

- doctest가 통과하고 크레이트 문서가 경고 없이 빌드된다.

## 작업 6: 문서 동등성 및 공개 범위

**복잡도:** 중간
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 수정: `crates/serialization/README.md`
- 수정: `crates/serialization/README.ko.md`
- 수정: `README.md`
- 수정: `README.ko.md`
- 검토: `WIP.md`

- [ ] **단계 1: 영어 크레이트 README 갱신**

`crates/serialization/README.md`에서 bootstrap-only 문구를 contract 문구로
교체한다. 다음 섹션을 포함한다.

```markdown
## Contracts

- `SerializationFormat`, `ContentType`, `AdapterId`, and `PayloadVersion` validate stable metadata tokens.
- `SerializationConfig` applies safe defaults: statically typed trust, `application/octet-stream`, payload version `1`, and a 16 MiB payload limit.
- `SerializedPayload` owns bytes and metadata together so `payload_size` matches `bytes.len()`.
- `PayloadMetadataPolicy` rejects format, content type, version, trust profile, adapter id, and size mismatches with typed errors.
- `Serializer<T>`, `Deserializer<T>`, and `BinarySerializer<T>` are `serde`-compatible contracts; callers still supply the Rust target type.

## Example

Show a minimal direct-crate flow that creates `SerializationConfig`, builds a `SerializedPayload`, validates `PayloadMetadataPolicy`, and matches `SerializationErrorKind`.

## Safety Boundary

There is no dynamic registry, hidden default serializer, environment-selected adapter, fallback adapter, or payload-selected Rust type.

`UnsafeLegacyCompatibility` is migration-only vocabulary for fully trusted deployments. It is not a default and must not be used for shared or untrusted payload boundaries without a separate adapter review.

## Cache Rollout Guidance

Version cache namespaces or key prefixes when changing format id, content type, trust profile, or incompatible payload versions. Mismatches are hard typed failures. Caller-owned actions are evict, rebuild from the source of truth, migrate namespace, or alert during unexpected mismatches.

Payload-free diagnostic fields are: error kind, operation, format id, content type, version relation, trust profile, adapter id, payload size bucket, and configured size limit.
```

- [ ] **단계 2: 동등한 내용으로 한국어 README 갱신**

`crates/serialization/README.ko.md`에서 같은 섹션을 한국어로 미러링한다.

```markdown
## Contracts

- `SerializationFormat`, `ContentType`, `AdapterId`, `PayloadVersion`는 stable metadata token을 검증합니다.
- `SerializationConfig` 기본값은 statically typed trust, `application/octet-stream`, payload version `1`, 16 MiB payload limit입니다.
- `SerializedPayload`는 bytes와 metadata를 함께 소유해 `payload_size`가 `bytes.len()`과 일치하도록 합니다.
- `PayloadMetadataPolicy`는 format, content type, version, trust profile, adapter id, size mismatch를 typed error로 거부합니다.
- `Serializer<T>`, `Deserializer<T>`, `BinarySerializer<T>`는 `serde` 호환 contract입니다. Rust target type은 caller가 제공합니다.

## Example

Direct crate dependency 사용 흐름으로 `SerializationConfig` 생성, `SerializedPayload` 구성, `PayloadMetadataPolicy` 검증, `SerializationErrorKind` matching을 보여줍니다.

## Safety Boundary

Dynamic registry, hidden default serializer, environment-selected adapter, fallback adapter, payload-selected Rust type은 없습니다.

`UnsafeLegacyCompatibility`는 완전히 trusted deployment의 migration-only vocabulary입니다. 기본값이 아니며 shared/untrusted payload boundary에서는 별도 adapter review 없이는 사용하지 않습니다.

## Cache Rollout Guidance

Format id, content type, trust profile, incompatible payload version이 바뀌면 cache namespace나 key prefix를 versioning합니다. Mismatch는 hard typed failure입니다. Caller-owned action은 evict, source of truth 기반 rebuild, namespace migration, 예상 밖 mismatch alert입니다.

Payload-free diagnostic field는 error kind, operation, format id, content type, version relation, trust profile, adapter id, payload size bucket, configured size limit입니다.
```

- [ ] **단계 3: root README 쌍 갱신**

`README.md` 및 `README.ko.md`에서 serialization package 설명을 crate
reservation/bootstrap에서 contract API로 갱신한다.

영어 문구:

```markdown
| `serialization` | active | Rust-native SerDe contracts: validated format metadata, trust profiles, typed errors, safe config defaults, and `serde`-compatible serializer/deserializer traits. Concrete adapters start in follow-up `0.5.0` issues. |
```

한국어 문구:

```markdown
| `serialization` | active | Rust-native SerDe contract: 검증된 format metadata, trust profile, typed error, safe config default, `serde` 호환 serializer/deserializer trait. Concrete adapter는 후속 `0.5.0` issue에서 시작합니다. |
```

표 문구가 다르면 기존 표 형태를 보존하고 serialization 행만 갱신한다.

- [ ] **단계 4: WIP 영향 검토**

실행한다.

```bash
rg -n "serialization|Serializer|Deserializer|trust profile|format id" WIP.md
```

예상 결과:

- WIP는 이미 #109를 typed contract로 설명하므로 범위 변경이 필요하지 않다.
- 구현 후에도 WIP가 #109를 pending으로 표시하면 0.5.0 task queue 아래에
  이슈 #109는 contract만 구현하고 adapter 작업은 #111에 남긴다는 짧은
  메모를 추가한다.

## 작업 7: 전체 검증 및 검토 준비 상태

**복잡도:** 중간
**필수 스킬:** `$bluetape-rs-patterns`, `verification-before-completion`

**파일:**
- 모든 변경 파일

- [ ] **단계 1: formatting 실행**

실행한다.

```bash
cargo fmt --all --check
```

예상 결과:

- exit code가 0이다.

- [ ] **단계 2: 대상 serialization test 실행**

실행한다.

```bash
cargo test -p bluetape-rs-serialization --all-features --locked
cargo test -p bluetape-rs-serialization --doc --all-features --locked
```

예상 결과:

- 모든 serialization unit, integration 및 doctest가 통과한다.

- [ ] **단계 3: root facade feature 검사 실행**

실행한다.

```bash
cargo check -p bluetape-rs --locked
cargo check -p bluetape-rs --no-default-features --locked
cargo check -p bluetape-rs --features serialization --locked
```

예상 결과:

- default 및 no-default build가 변경되지 않는다.
- `--features serialization`이 contract crate를 해석한다.

- [ ] **단계 4: workspace validation 실행**

실행한다.

```bash
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
git diff --check
```

예상 결과:

- 모든 명령이 exit 0으로 종료한다.

- [ ] **단계 5: dependency 경계 검증**

실행한다.

```bash
rg -n 'serde_json|bincode|prost|apache-avro|fory|redis|testcontainers|sqlx|bluetape-rs-compression' crates/serialization Cargo.toml Cargo.lock
```

예상 결과:

- 이슈 #109가 production adapter dependency를 도입하지 않는다.
- 문자열이 오래된 문서나 관련 없는 workspace package metadata에만 나타나면
  Step 6 보고서에 경로와 이유를 기록한다.

- [ ] **단계 6: Step 6-R 검토 입력 준비**

실행한다.

```bash
git status --short --branch
git diff --stat origin/develop...HEAD
git diff --name-only origin/develop...HEAD
```

예상 결과:

- 모든 변경이 의도한 이슈 #109 파일이다.
- 관련 없는 root checkout 변경이 없다.
- 구현 후 Step 6-R 코드 검토에 사용할 근거가 준비된다.

- [ ] **단계 7: PR 준비 전에 필수 workflow 검토 gate 완료**

구현과 검증 후:

- 명시적인 `P0=0 P1=0`과 함께 Step 6-R 로컬/네이티브 코드 검토 근거를
  기록한다.
- `SerializedPayload`, metadata construction 및 clone behavior의
  allocation/copy review 근거를 포함한다.
- 필요한 마지막 `## DoD Status` 섹션을 포함해 PR을 생성/갱신한다.
- CI/병합 준비 상태를 주장하기 전에 Step 7-R PR 후 검토를 실행한다.
- Step 7-R도 `P0=0 P1=0`이 될 때까지 병합 준비 상태를 주장하지 않는다.

## 검증 매트릭스

| 요구 사항 | 계획 범위 |
|---|---|
| Rust 네이티브 모듈 분할 | Tasks 2-5 |
| `serde` 호환 계약 | Task 5 |
| 안전한 기본값 및 타입 지정 config 검증 | Task 4 |
| Format/content/adapter/version 검증 | Task 2 |
| Payload-size 일관성 | Task 4 |
| 메타데이터 정책 불일치 매핑 | Task 4, format/content/version/trust/adapter id/size 포함 |
| Adapter-id 정책 와일드카드 및 엄격 일치 | Task 4 |
| 방향을 포함한 오류 컨텍스트 | Tasks 3-4 |
| 페이로드 바이트 없는 오류 컨텍스트 및 source-redaction 우회 저항성 | Task 3 |
| 캐시 롤아웃/운영자 지침 | Task 6 |
| 페이로드 없는 진단 필드 | Task 6 |
| 대형 페이로드 clone/allocation 검토 | Task 7 단계 7 |
| README/Rustdoc/README.ko 동등성 | Tasks 5-6 |
| 어댑터 의존성 없음 | Tasks 1 및 7 |

## 롤백 및 재실행 지점

- `serde` 또는 `thiserror` dependency 연결로 root feature 검사가 깨지면 Task 1만
  되돌리고 RED부터 Task 1을 다시 실행한다.
- validation newtype가 clippy/doc 마찰을 만들면 Task 2의 테스트는 유지하고,
  public constant나 허용 문법을 바꾸지 않은 채 구현을 리팩터링한다.
- Task 4 또는 Task 5에서 `SerializedPayload` 설계가 사용하기 어렵다면 구현을
  계속하기 전에 사양으로 돌아간다. caller-supplied payload size를 조용히
  다시 도입하지 않는다.
- README 동등성이 어긋나면 같은 commit에서 두 README 파일을 갱신하고 실제
  export name을 기준으로 `rg`로 확인한다.

## 단계 3 체크리스트 완료 보고

| 항목 | 상태 | 메모 |
|------|--------|-------|
| feature worktree 안의 plan path 확인 | 완료 | `docs/superpowers/plans/2026-06-13-serialization-contracts-plan.md` |
| 모든 작업에 복잡도 표기 | 완료 | Tasks 1-7 |
| 코드 작업에 `$bluetape-rs-patterns` 적용 | 완료 | 각 작업에 명시 |
| TDD red/green 단계 포함 | 완료 | Tasks 2-5 |
| 테스트 및 검증 작업 포함 | 완료 | Task 7 |
| README 로케일 집합 작업 포함 | 완료 | Task 6 |
| 위험한 순서/dependency 가정 명시 | 완료 | 롤백 및 재실행 지점 |
