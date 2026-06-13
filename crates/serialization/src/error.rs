use crate::{
    AdapterId, ContentType, PayloadVersion, SerializationFormat, SerializationTrustProfile,
};
use std::error::Error as StdError;
use std::fmt;
use thiserror::Error;

type AdapterSource = Box<dyn StdError + Send + Sync + 'static>;

/// Serialization operation direction for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SerializationOperation {
    /// Encoding a Rust value to bytes.
    Serialize,
    /// Decoding bytes to a Rust value.
    Deserialize,
}

impl fmt::Display for SerializationOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize => f.write_str("serialize"),
            Self::Deserialize => f.write_str("deserialize"),
        }
    }
}

/// Stable category for a serialization failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SerializationErrorKind {
    /// Invalid metadata token or metadata consistency problem.
    InvalidMetadata,
    /// Invalid serialization configuration.
    InvalidConfig,
    /// Payload format id did not match the expected format.
    FormatMismatch,
    /// Payload content type did not match the expected content type.
    ContentTypeMismatch,
    /// Payload version is not supported by this reader.
    UnsupportedVersion,
    /// Payload trust profile did not match the expected profile.
    TrustProfileMismatch,
    /// Payload adapter id did not match a strict policy.
    AdapterIdMismatch,
    /// Payload size exceeded a configured limit.
    PayloadSizeLimitExceeded,
    /// Payload bytes are malformed for the selected adapter.
    MalformedInput,
    /// Adapter failed while serializing or deserializing.
    AdapterFailure,
}

/// Source wrapper for adapter failures.
///
/// Use [`AdapterFailureSource::safe`] only when the source error is known not to
/// include raw payload bytes or payload snippets. Use
/// [`AdapterFailureSource::redacted`] for all other adapter sources.
pub struct AdapterFailureSource {
    source: AdapterSource,
}

impl AdapterFailureSource {
    /// Stores a safe, payload-free adapter source.
    #[must_use]
    pub fn safe<E>(source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            source: Box::new(source),
        }
    }

    /// Replaces a potentially payload-bearing source with a redacted source.
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

impl fmt::Debug for AdapterFailureSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AdapterFailureSource")
    }
}

impl fmt::Display for AdapterFailureSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.source.fmt(f)
    }
}

impl StdError for AdapterFailureSource {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.as_ref())
    }
}

/// Errors returned by serialization contracts and adapters.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SerializationError {
    /// Invalid metadata.
    #[error("invalid {field}: {reason}")]
    InvalidMetadata {
        /// Metadata field name.
        field: &'static str,
        /// Payload-free reason.
        reason: &'static str,
    },
    /// Invalid config.
    #[error("invalid serialization config {field}: {reason}")]
    InvalidConfig {
        /// Config field name.
        field: &'static str,
        /// Payload-free reason.
        reason: &'static str,
    },
    /// Format mismatch.
    #[error("serialization {operation} format mismatch: expected {expected}, observed {observed}")]
    FormatMismatch {
        /// Operation direction.
        operation: SerializationOperation,
        /// Expected format.
        expected: SerializationFormat,
        /// Observed format.
        observed: SerializationFormat,
    },
    /// Content type mismatch.
    #[error(
        "serialization {operation} content type mismatch: expected {expected}, observed {observed}"
    )]
    ContentTypeMismatch {
        /// Operation direction.
        operation: SerializationOperation,
        /// Expected content type.
        expected: ContentType,
        /// Observed content type.
        observed: ContentType,
    },
    /// Unsupported payload version.
    #[error(
        "unsupported serialization {operation} payload version: observed {observed}, max supported {max_supported}"
    )]
    UnsupportedVersion {
        /// Operation direction.
        operation: SerializationOperation,
        /// Inclusive maximum supported version.
        max_supported: PayloadVersion,
        /// Observed version.
        observed: PayloadVersion,
    },
    /// Trust profile mismatch.
    #[error(
        "serialization {operation} trust profile mismatch: expected {expected:?}, observed {observed:?}"
    )]
    TrustProfileMismatch {
        /// Operation direction.
        operation: SerializationOperation,
        /// Expected trust profile.
        expected: SerializationTrustProfile,
        /// Observed trust profile.
        observed: SerializationTrustProfile,
    },
    /// Adapter id mismatch.
    #[error(
        "serialization {operation} adapter id mismatch: expected {expected}, observed {observed}"
    )]
    AdapterIdMismatch {
        /// Operation direction.
        operation: SerializationOperation,
        /// Expected adapter id.
        expected: AdapterId,
        /// Observed adapter id.
        observed: AdapterId,
    },
    /// Payload size exceeded a configured limit.
    #[error("serialization {operation} payload size {actual} exceeded limit {limit}")]
    PayloadSizeLimitExceeded {
        /// Operation direction.
        operation: SerializationOperation,
        /// Configured limit.
        limit: usize,
        /// Observed size.
        actual: usize,
    },
    /// Malformed payload input.
    #[error("malformed serialization {operation} input from {adapter_id}: {reason}")]
    MalformedInput {
        /// Operation direction.
        operation: SerializationOperation,
        /// Adapter id.
        adapter_id: AdapterId,
        /// Payload-free reason.
        reason: &'static str,
    },
    /// Adapter failure with a safe or redacted source.
    #[error("serialization {operation} failed in adapter {adapter_id}")]
    AdapterFailure {
        /// Adapter id.
        adapter_id: AdapterId,
        /// Operation direction.
        operation: SerializationOperation,
        /// Safe or redacted source.
        #[source]
        source: AdapterFailureSource,
    },
}

impl SerializationError {
    /// Returns the stable error category.
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
            Self::PayloadSizeLimitExceeded { .. } => {
                SerializationErrorKind::PayloadSizeLimitExceeded
            }
            Self::MalformedInput { .. } => SerializationErrorKind::MalformedInput,
            Self::AdapterFailure { .. } => SerializationErrorKind::AdapterFailure,
        }
    }

    /// Returns the operation direction when the error is operation-specific.
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

    /// Creates an invalid metadata error.
    #[must_use]
    pub fn invalid_metadata(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidMetadata { field, reason }
    }

    /// Creates an invalid config error.
    #[must_use]
    pub fn invalid_config(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidConfig { field, reason }
    }

    /// Creates a format mismatch error.
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

    /// Creates a content type mismatch error.
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

    /// Creates an unsupported version error.
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

    /// Creates a trust profile mismatch error.
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

    /// Creates an adapter id mismatch error.
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

    /// Creates a payload size limit error.
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

    /// Creates an adapter failure with a safe, payload-free source.
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

    /// Creates an adapter failure with a redacted source.
    #[must_use]
    pub fn redacted_adapter_failure<E>(
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
            source: AdapterFailureSource::redacted(source),
        }
    }

    /// Creates a malformed input error.
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

impl fmt::Display for RedactedAdapterSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("adapter source redacted")
    }
}

impl StdError for RedactedAdapterSource {}
