use crate::{
    AdapterId, ContentType, PayloadVersion, SerializationError, SerializationFormat,
    SerializationOperation, SerializationTrustProfile,
};
use std::fmt;

/// Metadata describing serialized payload bytes.
#[derive(Clone, PartialEq, Eq)]
pub struct PayloadMetadata {
    /// Payload format id.
    format: SerializationFormat,
    /// Payload content type.
    content_type: ContentType,
    /// Payload version.
    version: PayloadVersion,
    /// Trust profile expected for this payload.
    trust_profile: SerializationTrustProfile,
    /// Adapter that produced the payload.
    adapter_id: AdapterId,
    /// Payload byte length.
    payload_size: usize,
}

impl PayloadMetadata {
    /// Creates payload metadata from validated parts.
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

    /// Returns the payload format id.
    #[must_use]
    pub fn format(&self) -> &SerializationFormat {
        &self.format
    }

    /// Returns the payload content type.
    #[must_use]
    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    /// Returns the payload version.
    #[must_use]
    pub fn version(&self) -> PayloadVersion {
        self.version
    }

    /// Returns the payload trust profile.
    #[must_use]
    pub fn trust_profile(&self) -> SerializationTrustProfile {
        self.trust_profile
    }

    /// Returns the adapter id that produced the payload.
    #[must_use]
    pub fn adapter_id(&self) -> &AdapterId {
        &self.adapter_id
    }

    /// Returns the payload byte length recorded in metadata.
    #[must_use]
    pub fn payload_size(&self) -> usize {
        self.payload_size
    }
}

/// Serialized bytes and metadata kept together.
///
/// # Examples
///
/// ```
/// use bluetape_rs_serialization::{
///     AdapterId, SerializationConfig, SerializationFormat, SerializedPayload,
/// };
///
/// let config = SerializationConfig::new(
///     SerializationFormat::new("binary")?,
///     AdapterId::new("binary.primary")?,
/// )?;
/// let metadata = config.metadata_for_size(3)?;
/// let payload = SerializedPayload::new(vec![1, 2, 3], metadata)?;
/// assert_eq!(payload.metadata().payload_size(), payload.bytes().len());
/// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
/// ```
///
/// # Errors
///
/// Returns [`SerializationError`] when metadata `payload_size` differs from
/// `bytes.len()`.
#[derive(PartialEq, Eq)]
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

    /// Returns the serialized bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes this payload and returns the serialized bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns payload metadata.
    #[must_use]
    pub fn metadata(&self) -> &PayloadMetadata {
        &self.metadata
    }
}

impl fmt::Debug for PayloadMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PayloadMetadata")
            .field("format", &self.format)
            .field("content_type", &self.content_type)
            .field("version", &self.version)
            .field("trust_profile", &self.trust_profile)
            .field("adapter_id", &self.adapter_id)
            .field("payload_size", &self.payload_size)
            .finish()
    }
}

impl fmt::Debug for SerializedPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SerializedPayload")
            .field("metadata", &self.metadata)
            .field("bytes_len", &self.bytes.len())
            .finish()
    }
}

/// Expected metadata policy for deserialization.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PayloadMetadataPolicy {
    /// Expected format.
    format: SerializationFormat,
    /// Expected content type.
    content_type: ContentType,
    /// Inclusive maximum supported payload version.
    max_supported_version: PayloadVersion,
    /// Expected trust profile.
    trust_profile: SerializationTrustProfile,
    /// Optional strict adapter id.
    adapter_id: Option<AdapterId>,
    /// Maximum payload size.
    max_payload_size: usize,
}

impl PayloadMetadataPolicy {
    /// Creates a policy from explicit parts.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when `max_payload_size` is zero.
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

    /// Validates payload metadata before deserialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_serialization::{AdapterId, SerializationConfig, SerializationFormat};
    ///
    /// let config = SerializationConfig::new(
    ///     SerializationFormat::new("binary")?,
    ///     AdapterId::new("binary.primary")?,
    /// )?;
    /// let metadata = config.metadata_for_size(8)?;
    /// config.metadata_policy().validate(&metadata)?;
    /// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns typed mismatch errors for format, content type, version, trust
    /// profile, adapter id, or payload size mismatches.
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
