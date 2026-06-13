use crate::{PayloadMetadataPolicy, SerializationError, SerializedPayload};
use serde::{Serialize, de::DeserializeOwned};

/// Serializes a caller-supplied Rust value to a typed payload.
///
/// # Examples
///
/// ```
/// use bluetape_rs_serialization::{
///     AdapterId, SerializationConfig, SerializationError, SerializationFormat,
///     SerializedPayload, Serializer,
/// };
/// # use serde::Serialize;
///
/// struct Utf8Serializer {
///     config: SerializationConfig,
/// }
///
/// impl Serializer<String> for Utf8Serializer {
///     fn serialize(&self, value: &String) -> Result<SerializedPayload, SerializationError> {
///         let bytes = value.as_bytes().to_vec();
///         let metadata = self.config.metadata_for_size(bytes.len())?;
///         SerializedPayload::new(bytes, metadata)
///     }
/// }
///
/// let serializer = Utf8Serializer {
///     config: SerializationConfig::new(
///         SerializationFormat::new("binary")?,
///         AdapterId::new("utf8.example")?,
///     )?,
/// };
/// let payload = serializer.serialize(&"hello".to_owned())?;
/// assert_eq!(payload.bytes(), b"hello");
/// # Ok::<(), bluetape_rs_serialization::SerializationError>(())
/// ```
pub trait Serializer<T>
where
    T: Serialize,
{
    /// Serializes `value`.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when the adapter cannot encode the value.
    fn serialize(&self, value: &T) -> Result<SerializedPayload, SerializationError>;
}

/// Deserializes typed payload bytes to a caller-supplied Rust type.
///
/// ```compile_fail
/// use bluetape_rs_serialization::Deserializer;
///
/// fn borrowed_target_is_not_owned<D>(decoder: &D)
/// where
///     D: Deserializer<&'static str>,
/// {
///     let _ = decoder;
/// }
/// ```
pub trait Deserializer<T>
where
    T: DeserializeOwned,
{
    /// Deserializes `payload`.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] when metadata validation or adapter decode
    /// fails.
    fn deserialize(&self, payload: &SerializedPayload) -> Result<T, SerializationError>;

    /// Returns metadata expected by this deserializer.
    fn expected_metadata(&self) -> PayloadMetadataPolicy;
}

/// Convenience trait for serializers that can both encode and decode binary
/// payloads.
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
