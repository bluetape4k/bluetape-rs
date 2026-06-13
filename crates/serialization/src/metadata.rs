use crate::{
    AdapterId, ContentType, PayloadVersion, SerializationError, SerializationFormat,
    SerializationOperation, SerializationTrustProfile,
};

/// Metadata describing serialized payload bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadMetadata {
    /// Payload format id.
    pub format: SerializationFormat,
    /// Payload content type.
    pub content_type: ContentType,
    /// Payload version.
    pub version: PayloadVersion,
    /// Trust profile expected for this payload.
    pub trust_profile: SerializationTrustProfile,
    /// Adapter that produced the payload.
    pub adapter_id: AdapterId,
    /// Payload byte length.
    pub payload_size: usize,
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
/// assert_eq!(payload.metadata().payload_size, payload.bytes().len());
/// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
/// ```
///
/// # Errors
///
/// Returns [`SerializationError`] when metadata `payload_size` differs from
/// `bytes.len()`.
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

/// Expected metadata policy for deserialization.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PayloadMetadataPolicy {
    /// Expected format.
    pub format: SerializationFormat,
    /// Expected content type.
    pub content_type: ContentType,
    /// Inclusive maximum supported payload version.
    pub max_supported_version: PayloadVersion,
    /// Expected trust profile.
    pub trust_profile: SerializationTrustProfile,
    /// Optional strict adapter id.
    pub adapter_id: Option<AdapterId>,
    /// Maximum payload size.
    pub max_payload_size: usize,
}

impl PayloadMetadataPolicy {
    /// Creates a policy from explicit parts.
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

    /// Validates payload metadata before deserialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_serialization::{
    ///     AdapterId, SerializationConfig, SerializationFormat,
    /// };
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
