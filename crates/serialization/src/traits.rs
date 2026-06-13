use crate::{PayloadMetadataPolicy, SerializationError, SerializedPayload};
use serde::{Serialize, de::DeserializeOwned};

/// Serializes a caller-supplied Rust value to a typed payload.
///
/// # Examples
///
/// ```
/// # use bluetape_rs_serialization::{SerializationError, SerializedPayload};
/// # use serde::Serialize;
/// # struct Example;
/// # impl bluetape_rs_serialization::Serializer<String> for Example {
/// #     fn serialize(&self, _value: &String) -> Result<SerializedPayload, SerializationError> {
/// #         unimplemented!()
/// #     }
/// # }
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
