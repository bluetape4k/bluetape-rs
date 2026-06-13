use crate::{
    AdapterId, ContentType, PayloadMetadata, PayloadMetadataPolicy, PayloadVersion,
    SerializationError, SerializationFormat, SerializationOperation, SerializationTrustProfile,
};

/// Default serialized payload safety limit: 16 MiB.
pub const DEFAULT_MAX_PAYLOAD_SIZE: usize = 16 * 1024 * 1024;

/// Safe default configuration for concrete serialization adapters.
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
    /// Creates a serialization config with safe defaults.
    ///
    /// Defaults are `application/octet-stream`, payload version `1`,
    /// [`SerializationTrustProfile::StaticallyTyped`], and a 16 MiB max payload.
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
    /// assert_eq!(config.version().get(), 1);
    /// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] only if constructing the built-in payload
    /// version fails.
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

    /// Returns a config with the given payload size limit.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when `max_payload_size` is zero.
    pub fn with_max_payload_size(
        mut self,
        max_payload_size: usize,
    ) -> Result<Self, SerializationError> {
        if max_payload_size == 0 {
            return Err(SerializationError::invalid_config(
                "max_payload_size",
                "max payload size must be positive",
            ));
        }
        self.max_payload_size = max_payload_size;
        Ok(self)
    }

    /// Returns a config with the given content type.
    #[must_use]
    pub fn with_content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = content_type;
        self
    }

    /// Returns a config with the given positive payload version.
    #[must_use]
    pub fn with_version(mut self, version: PayloadVersion) -> Self {
        self.version = version;
        self
    }

    /// Returns a config with a non-legacy trust profile.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] for
    /// [`SerializationTrustProfile::UnsafeLegacyCompatibility`]. Use
    /// [`SerializationConfig::with_unsafe_legacy_compatibility_for_migration`]
    /// for explicit migration-only opt-in.
    pub fn with_trust_profile(
        mut self,
        trust_profile: SerializationTrustProfile,
    ) -> Result<Self, SerializationError> {
        if trust_profile == SerializationTrustProfile::UnsafeLegacyCompatibility {
            return Err(SerializationError::invalid_config(
                "trust_profile",
                "unsafe legacy compatibility requires explicit migration opt-in",
            ));
        }
        self.trust_profile = trust_profile;
        Ok(self)
    }

    /// Explicitly opts into the migration-only unsafe legacy trust profile.
    #[must_use]
    pub fn with_unsafe_legacy_compatibility_for_migration(mut self) -> Self {
        self.trust_profile = SerializationTrustProfile::UnsafeLegacyCompatibility;
        self
    }

    /// Creates metadata for an encoded payload size.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when `payload_size` exceeds the configured
    /// maximum.
    pub fn metadata_for_size(
        &self,
        payload_size: usize,
    ) -> Result<PayloadMetadata, SerializationError> {
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

    /// Creates a metadata policy from this config.
    #[must_use]
    pub fn metadata_policy(&self) -> PayloadMetadataPolicy {
        PayloadMetadataPolicy::from_config(self)
    }

    /// Returns the format id.
    #[must_use]
    pub fn format(&self) -> &SerializationFormat {
        &self.format
    }

    /// Returns the content type.
    #[must_use]
    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    /// Returns the payload version.
    #[must_use]
    pub fn version(&self) -> PayloadVersion {
        self.version
    }

    /// Returns the trust profile.
    #[must_use]
    pub fn trust_profile(&self) -> SerializationTrustProfile {
        self.trust_profile
    }

    /// Returns the max payload size.
    #[must_use]
    pub fn max_payload_size(&self) -> usize {
        self.max_payload_size
    }
}

impl PayloadMetadataPolicy {
    /// Creates a policy from a config.
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
