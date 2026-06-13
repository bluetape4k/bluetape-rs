# Serialization Contracts Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement issue #109 by adding Rust-native serialization contract APIs for format ids, trust profiles, payload metadata, typed errors, config defaults, and `serde`-compatible traits without adding any concrete adapter.

**Architecture:** Keep `crates/serialization/src/lib.rs` as the concise export surface and split public contracts into focused modules: `format`, `trust`, `metadata`, `error`, `config`, and `traits`. Encode produces a `SerializedPayload` that owns bytes plus metadata, so payload size cannot diverge from `bytes.len()`. Decode remains caller-typed through `serde::de::DeserializeOwned`, and adapter selection remains explicit with no registry or fallback.

**Tech Stack:** Rust 2024, `serde` public trait bounds, `thiserror` typed errors, Cargo workspace dependencies, compile-checked Rustdoc examples, integration tests under `crates/serialization/tests`.

---

## File Structure

- Modify `Cargo.toml`: add workspace `serde = "1.0"` and keep adapter dependencies out.
- Modify `crates/serialization/Cargo.toml`: add `serde.workspace = true` and `thiserror.workspace = true`; keep default features empty.
- Modify `crates/serialization/src/lib.rs`: concise module declarations and public re-exports only.
- Create `crates/serialization/src/format.rs`: `SerializationFormat`, `ContentType`, `AdapterId`, `PayloadVersion`, validation constants, constructors, `Display`, `AsRef<str>`.
- Create `crates/serialization/src/trust.rs`: `SerializationTrustProfile` and its safety-oriented Rustdoc.
- Create `crates/serialization/src/metadata.rs`: `PayloadMetadata`, `SerializedPayload`, `PayloadMetadataPolicy`, payload-size consistency and metadata-policy validation.
- Create `crates/serialization/src/error.rs`: `SerializationError`, `SerializationErrorKind`, `SerializationOperation`, non-bypassable adapter-source wrapper, typed mismatch/config/malformed/limit variants.
- Create `crates/serialization/src/config.rs`: `SerializationConfig`, `DEFAULT_MAX_PAYLOAD_SIZE`, safe defaults and validation.
- Create `crates/serialization/src/traits.rs`: `Serializer<T>`, `Deserializer<T>`, `BinarySerializer<T>`.
- Create `crates/serialization/tests/contracts.rs`: public API tests for validation, defaults, metadata policy, mismatch errors, payload-size consistency, and source redaction.
- Modify `crates/serialization/README.md`: document contract APIs, unsupported adapters, no dynamic registry, no payload-selected type, cache rollout guidance.
- Modify `crates/serialization/README.ko.md`: keep Korean README synchronized with the English README.
- Modify `README.md` and `README.ko.md`: update serialization package row/guidance from bootstrap-only to contracts-only.

## Task 1: Cargo Dependency Boundary

**Complexity:** low
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Modify: `Cargo.toml`
- Modify: `crates/serialization/Cargo.toml`

- [ ] **Step 1: Write the dependency boundary expectation**

Run before editing:

```bash
rg -n 'serde|thiserror|bincode|serde_json|prost|apache-avro|fory|redis|testcontainers|sqlx' Cargo.toml crates/serialization/Cargo.toml
```

Expected before change:

- root `Cargo.toml` has `thiserror` but no `serde`;
- `crates/serialization/Cargo.toml` has no dependencies;
- no adapter dependencies are present in `crates/serialization`.

- [ ] **Step 2: Add only allowed workspace dependencies**

In root `Cargo.toml`, add this under `[workspace.dependencies]` near the other third-party dependencies:

```toml
serde = "1.0"
```

In `crates/serialization/Cargo.toml`, replace the empty `[dependencies]` section with:

```toml
[dependencies]
serde.workspace = true
thiserror.workspace = true

[dev-dependencies]
serde = { workspace = true, features = ["derive"] }
```

Do not add `serde_json`, `bincode`, `prost`, `apache-avro`, Fory, Redis, Testcontainers, SQLx, compression, or resilience dependencies.

- [ ] **Step 3: Verify the dependency boundary**

Run:

```bash
cargo check -p bluetape-rs-serialization --all-features
rg -n 'serde_json|bincode|prost|apache-avro|fory|redis|testcontainers|sqlx|bluetape-rs-compression' crates/serialization Cargo.toml
```

Expected:

- `cargo check` succeeds and refreshes `Cargo.lock` if needed.
- `rg` returns no adapter dependency in `crates/serialization`.

## Task 2: Validation Newtypes And Trust Profiles

**Complexity:** medium
**Required skill:** `$bluetape-rs-patterns`

**Files:**
  - Create: `crates/serialization/src/error.rs`
- Create: `crates/serialization/src/format.rs`
- Create: `crates/serialization/src/trust.rs`
- Create: `crates/serialization/tests/contracts.rs`
- Modify: `crates/serialization/src/lib.rs`

- [ ] **Step 1: Write failing tests for valid and invalid metadata tokens**

Create `crates/serialization/tests/contracts.rs` with this initial test set:

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

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

Expected RED:

- test compile fails because `AdapterId`, `ContentType`, `PayloadVersion`, `SerializationFormat`, and `SerializationTrustProfile` are not defined.

- [ ] **Step 3: Implement minimal error scaffold, validation newtypes, and trust enum**

Create the minimal `crates/serialization/src/error.rs` scaffold first so token constructors can return the crate error type during Task 2:

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

Create `crates/serialization/src/format.rs`:

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

Create `crates/serialization/src/trust.rs`:

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

Modify `crates/serialization/src/lib.rs` to add modules and exports:

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

- [ ] **Step 4: Run GREEN for token tests**

Run:

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

Expected:

- metadata token tests pass with the minimal error scaffold.

## Task 3: Typed Errors And Redacted Adapter Sources

**Complexity:** medium
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Create: `crates/serialization/src/error.rs`
- Modify: `crates/serialization/tests/contracts.rs`
- Modify: `crates/serialization/src/lib.rs`

- [ ] **Step 1: Extend tests for typed errors and redaction**

Append to `crates/serialization/tests/contracts.rs`:

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

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

Expected RED:

- compile fails because error constructors and variants are not implemented.

- [ ] **Step 3: Implement `SerializationError`**

Create `crates/serialization/src/error.rs` with:

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

Do not expose the raw `AdapterSource` alias or a raw `Box<dyn Error>` source field in the public API. Public adapter failures must be created through explicit safe-source or redacted-source constructors, and Step 6-R must verify that no public API accepts an unwrapped raw adapter source.

Modify the error export in `crates/serialization/src/lib.rs` after the full error implementation:

```rust
pub use error::{
    AdapterFailureSource, SerializationError, SerializationErrorKind, SerializationOperation,
};
```

- [ ] **Step 4: Run GREEN for error tests**

Run:

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

Expected:

- token and error tests pass.

## Task 4: Config, Metadata, And Policy Contracts

**Complexity:** high
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Create: `crates/serialization/src/config.rs`
- Create: `crates/serialization/src/metadata.rs`
- Modify: `crates/serialization/src/lib.rs`
- Modify: `crates/serialization/tests/contracts.rs`

- [ ] **Step 1: Write failing tests for config defaults and payload consistency**

Append to `crates/serialization/tests/contracts.rs`:

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

    let wrong_format = PayloadMetadata {
        format: SerializationFormat::new("json").unwrap(),
        ..metadata.clone()
    };
    assert_eq!(
        policy.validate(&wrong_format).unwrap_err().kind(),
        SerializationErrorKind::FormatMismatch
    );

    let wrong_content_type = PayloadMetadata {
        content_type: ContentType::new("application/json").unwrap(),
        ..metadata.clone()
    };
    assert_eq!(
        policy.validate(&wrong_content_type).unwrap_err().kind(),
        SerializationErrorKind::ContentTypeMismatch
    );

    let wrong_trust_profile = PayloadMetadata {
        trust_profile: SerializationTrustProfile::TrustedInternal,
        ..metadata.clone()
    };
    assert_eq!(
        policy.validate(&wrong_trust_profile).unwrap_err().kind(),
        SerializationErrorKind::TrustProfileMismatch
    );

    let wrong_adapter = PayloadMetadata {
        adapter_id: AdapterId::new("binary.secondary").unwrap(),
        ..metadata.clone()
    };
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
    );
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

    let oversized = PayloadMetadata {
        payload_size: 11,
        ..exact_limit.clone()
    };
    assert_eq!(
        policy.validate(&oversized).unwrap_err().kind(),
        SerializationErrorKind::PayloadSizeLimitExceeded
    );

    let newer_version = PayloadMetadata {
        version: PayloadVersion::new(2).unwrap(),
        ..exact_limit
    };
    let error = policy.validate(&newer_version).unwrap_err();
    assert_eq!(error.kind(), SerializationErrorKind::UnsupportedVersion);
    assert_eq!(error.operation(), Some(SerializationOperation::Deserialize));
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

Expected RED:

- compile fails because `SerializationConfig`, `PayloadMetadataPolicy`, and `SerializedPayload` are not defined.

- [ ] **Step 3: Implement config and metadata**

Create `crates/serialization/src/metadata.rs` with:

```rust
use crate::{
    AdapterId, ContentType, PayloadVersion, SerializationError, SerializationFormat,
    SerializationOperation, SerializationTrustProfile,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadMetadata {
    pub format: SerializationFormat,
    pub content_type: ContentType,
    pub version: PayloadVersion,
    pub trust_profile: SerializationTrustProfile,
    pub adapter_id: AdapterId,
    pub payload_size: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SerializedPayload {
    bytes: Vec<u8>,
    metadata: PayloadMetadata,
}

impl SerializedPayload {
    pub fn new(bytes: Vec<u8>, metadata: PayloadMetadata) -> Result<Self, SerializationError> {
        if bytes.len() != metadata.payload_size {
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
    pub format: SerializationFormat,
    pub content_type: ContentType,
    pub max_supported_version: PayloadVersion,
    pub trust_profile: SerializationTrustProfile,
    pub adapter_id: Option<AdapterId>,
    pub max_payload_size: usize,
}

impl PayloadMetadataPolicy {
    #[must_use]
    pub fn from_parts(
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

Create `crates/serialization/src/config.rs` with:

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
        Ok(PayloadMetadata {
            format: self.format.clone(),
            content_type: self.content_type.clone(),
            version: self.version,
            trust_profile: self.trust_profile,
            adapter_id: self.adapter_id.clone(),
            payload_size,
        })
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
        Self::from_parts(
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

Modify `crates/serialization/src/lib.rs`:

```rust
mod config;
mod metadata;

pub use config::{DEFAULT_MAX_PAYLOAD_SIZE, SerializationConfig};
pub use metadata::{PayloadMetadata, PayloadMetadataPolicy, SerializedPayload};
```

- [ ] **Step 4: Run GREEN for config and metadata tests**

Run:

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

Expected:

- config, metadata, and policy tests pass.

## Task 5: Serde-Compatible Traits And Rustdoc Examples

**Complexity:** medium
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Create: `crates/serialization/src/traits.rs`
- Modify: `crates/serialization/src/lib.rs`
- Modify: `crates/serialization/src/config.rs`
- Modify: `crates/serialization/src/metadata.rs`
- Modify: `crates/serialization/tests/contracts.rs`

- [ ] **Step 1: Write failing tests for trait implementation shape**

Append to `crates/serialization/tests/contracts.rs`:

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

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

Expected RED:

- compile fails because traits and `serde` derive dev usage are not wired.

- [ ] **Step 3: Implement traits**

Create `crates/serialization/src/traits.rs`:

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

Modify `crates/serialization/src/lib.rs`:

```rust
mod traits;

pub use traits::{BinarySerializer, Deserializer, Serializer};
```

The derive macro dev dependency was added in Task 1. The production dependency remains `serde.workspace = true` without requiring derive.

- [ ] **Step 4: Run GREEN for trait tests**

Run:

```bash
cargo test -p bluetape-rs-serialization --test contracts --all-features --locked
```

Expected:

- trait round-trip test passes.

- [ ] **Step 5: Add compile-checked Rustdoc examples**

Add `# Examples` and `# Errors` Rustdoc to public constructors and methods:

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

Each example must compile under doctest; examples may use `#` hidden setup lines to keep docs concise. Include one `compile_fail` Rustdoc example showing that borrowed decode targets that cannot satisfy `DeserializeOwned` are rejected.

- [ ] **Step 6: Verify docs compile**

Run:

```bash
cargo test -p bluetape-rs-serialization --doc --all-features --locked
RUSTDOCFLAGS="-D warnings" cargo doc -p bluetape-rs-serialization --all-features --no-deps --locked
```

Expected:

- doctests pass and crate docs build without warnings.

## Task 6: Documentation Parity And Public Scope

**Complexity:** medium
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Modify: `crates/serialization/README.md`
- Modify: `crates/serialization/README.ko.md`
- Modify: `README.md`
- Modify: `README.ko.md`
- Review: `WIP.md`

- [ ] **Step 1: Update crate README in English**

In `crates/serialization/README.md`, replace bootstrap-only wording with contract wording. Include these sections:

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

- [ ] **Step 2: Update Korean README with equivalent content**

In `crates/serialization/README.ko.md`, mirror the same sections in Korean:

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

- [ ] **Step 3: Update root README pair**

In `README.md` and `README.ko.md`, update the serialization package description from crate reservation/bootstrap to contract APIs:

English wording:

```markdown
| `serialization` | active | Rust-native SerDe contracts: validated format metadata, trust profiles, typed errors, safe config defaults, and `serde`-compatible serializer/deserializer traits. Concrete adapters start in follow-up `0.5.0` issues. |
```

Korean wording:

```markdown
| `serialization` | active | Rust-native SerDe contract: 검증된 format metadata, trust profile, typed error, safe config default, `serde` 호환 serializer/deserializer trait. Concrete adapter는 후속 `0.5.0` issue에서 시작합니다. |
```

If the table wording differs, preserve the existing table shape and update only the serialization row.

- [ ] **Step 4: Review WIP impact**

Run:

```bash
rg -n "serialization|Serializer|Deserializer|trust profile|format id" WIP.md
```

Expected:

- WIP already describes #109 as typed contracts and does not need a scope change.
- If WIP still says #109 is pending after implementation, add one short note under the 0.5.0 task queue that issue #109 implements contracts only and adapter work remains #111.

## Task 7: Full Validation And Review-Ready State

**Complexity:** medium
**Required skill:** `$bluetape-rs-patterns`, `verification-before-completion`

**Files:**
- All changed files

- [ ] **Step 1: Run formatting**

Run:

```bash
cargo fmt --all --check
```

Expected:

- exit code 0.

- [ ] **Step 2: Run targeted serialization tests**

Run:

```bash
cargo test -p bluetape-rs-serialization --all-features --locked
cargo test -p bluetape-rs-serialization --doc --all-features --locked
```

Expected:

- all serialization unit, integration, and doctests pass.

- [ ] **Step 3: Run root facade feature checks**

Run:

```bash
cargo check -p bluetape-rs --locked
cargo check -p bluetape-rs --no-default-features --locked
cargo check -p bluetape-rs --features serialization --locked
```

Expected:

- default and no-default builds remain unchanged;
- `--features serialization` resolves the contract crate.

- [ ] **Step 4: Run workspace validation**

Run:

```bash
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
git diff --check
```

Expected:

- all commands exit 0.

- [ ] **Step 5: Verify dependency boundary**

Run:

```bash
rg -n 'serde_json|bincode|prost|apache-avro|fory|redis|testcontainers|sqlx|bluetape-rs-compression' crates/serialization Cargo.toml Cargo.lock
```

Expected:

- no production adapter dependency is introduced by issue #109.
- If a string appears only in old docs or unrelated workspace package metadata, record the path and reason in the Step 6 report.

- [ ] **Step 6: Prepare Step 6-R review input**

Run:

```bash
git status --short --branch
git diff --stat origin/develop...HEAD
git diff --name-only origin/develop...HEAD
```

Expected:

- all changes are intentional issue #109 files;
- no unrelated root checkout changes;
- evidence is ready for Step 6-R code review after implementation.

- [ ] **Step 7: Complete required workflow review gates before PR readiness**

After implementation and validation:

- record Step 6-R local/native code review evidence with explicit `P0=0 P1=0`;
- include allocation/copy review evidence for `SerializedPayload`, metadata construction, and clone behavior;
- create/update the PR with the required final `## DoD Status` section;
- run Step 7-R post-PR review before any CI/merge-ready claim;
- do not claim merge readiness until Step 7-R also has `P0=0 P1=0`.

## Verification Matrix

| Requirement | Plan coverage |
|---|---|
| Rust-native module split | Tasks 2-5 |
| `serde`-compatible contracts | Task 5 |
| Safe defaults and typed config validation | Task 4 |
| Format/content/adapter/version validation | Task 2 |
| Payload-size consistency | Task 4 |
| Metadata policy mismatch mapping | Task 4, including format/content/version/trust/adapter id/size |
| Adapter-id policy wildcard and strict matching | Task 4 |
| Direction-bearing error context | Tasks 3-4 |
| Error context without payload bytes and source-redaction bypass resistance | Task 3 |
| Cache rollout/operator guidance | Task 6 |
| Payload-free diagnostic fields | Task 6 |
| Large payload clone/allocation review | Task 7 Step 7 |
| README/Rustdoc/README.ko parity | Tasks 5-6 |
| No adapter dependencies | Tasks 1 and 7 |

## Rollback And Re-run Points

- If `serde` or `thiserror` dependency wiring breaks root feature checks, revert Task 1 only and re-run Task 1 from RED.
- If validation newtypes create clippy/doc friction, keep tests from Task 2 and refactor implementation without changing public constants or accepted grammar.
- If `SerializedPayload` design proves awkward in Task 4 or Task 5, return to the spec before implementation continues; do not silently reintroduce caller-supplied payload size.
- If README parity drifts, update both README files in the same commit and verify with `rg` against actual exported names.

## Step 3 Checklist Completion Report

| Item | Status | Notes |
|------|--------|-------|
| Plan path confirmed inside feature worktree | Done | `docs/superpowers/plans/2026-06-13-serialization-contracts-plan.md` |
| All tasks have complexity labels | Done | Tasks 1-7 |
| `$bluetape-rs-patterns` applied to code-bearing tasks | Done | Explicit in each task |
| TDD red/green steps included | Done | Tasks 2-5 |
| Tests and verification tasks included | Done | Task 7 |
| README locale set tasks included | Done | Task 6 |
| Risky ordering/dependency assumptions explicit | Done | Rollback and re-run points |
