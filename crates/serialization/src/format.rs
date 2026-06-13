use crate::SerializationError;
use std::fmt;

/// Maximum serialization format id length in bytes.
pub const MAX_FORMAT_ID_LEN: usize = 64;
/// Maximum content type length in bytes.
pub const MAX_CONTENT_TYPE_LEN: usize = 127;
/// Maximum adapter id length in bytes.
pub const MAX_ADAPTER_ID_LEN: usize = 64;

/// Stable serialization format identifier.
///
/// # Examples
///
/// ```
/// use bluetape_rs_serialization::SerializationFormat;
///
/// let format = SerializationFormat::new("binary")?;
/// assert_eq!(format.as_str(), "binary");
/// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
/// ```
///
/// # Errors
///
/// Returns [`SerializationError`] when the id is empty, too long, non-ASCII, or
/// contains unsupported bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SerializationFormat(String);

/// Validated media type for serialized payload bytes.
///
/// # Examples
///
/// ```
/// use bluetape_rs_serialization::ContentType;
///
/// let content_type = ContentType::new("application/octet-stream")?;
/// assert_eq!(content_type.as_str(), "application/octet-stream");
/// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
/// ```
///
/// # Errors
///
/// Returns [`SerializationError`] when the value is not a lowercase ASCII media
/// type with exactly one slash and non-empty type/subtype segments.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentType(String);

/// Stable id for a concrete serialization adapter.
///
/// # Examples
///
/// ```
/// use bluetape_rs_serialization::AdapterId;
///
/// let adapter = AdapterId::new("binary.primary")?;
/// assert_eq!(adapter.as_str(), "binary.primary");
/// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
/// ```
///
/// # Errors
///
/// Returns [`SerializationError`] when the id is empty, too long, non-ASCII, or
/// contains unsupported bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AdapterId(String);

/// Positive serialization payload version.
///
/// # Examples
///
/// ```
/// use bluetape_rs_serialization::PayloadVersion;
///
/// let version = PayloadVersion::new(1)?;
/// assert_eq!(version.get(), 1);
/// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
/// ```
///
/// # Errors
///
/// Returns [`SerializationError`] when `version` is zero.
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

impl AsRef<str> for SerializationFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl ContentType {
    pub fn new(value: impl Into<String>) -> Result<Self, SerializationError> {
        let value = value.into();
        validate_token(
            "content_type",
            &value,
            MAX_CONTENT_TYPE_LEN,
            is_content_type_byte,
        )?;
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

impl AsRef<str> for ContentType {
    fn as_ref(&self) -> &str {
        self.as_str()
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

impl AsRef<str> for AdapterId {
    fn as_ref(&self) -> &str {
        self.as_str()
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

impl fmt::Display for PayloadVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn validate_token(
    field: &'static str,
    value: &str,
    max_len: usize,
    allowed: fn(u8) -> bool,
) -> Result<(), SerializationError> {
    if value.is_empty() {
        return Err(SerializationError::invalid_metadata(
            field,
            "value must not be empty",
        ));
    }
    if value.len() > max_len {
        return Err(SerializationError::invalid_metadata(
            field,
            "value exceeds max length",
        ));
    }
    if !value.is_ascii() || !value.bytes().all(allowed) {
        return Err(SerializationError::invalid_metadata(
            field,
            "value contains unsupported bytes",
        ));
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
