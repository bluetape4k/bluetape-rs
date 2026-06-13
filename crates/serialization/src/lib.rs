//! Rust-native serialization contracts for bluetape-rs.
//!
//! This crate defines the metadata, error, configuration, and trait contracts
//! that later concrete adapters implement. It intentionally does not provide a
//! global registry, hidden default adapter, fallback adapter, or payload-selected
//! Rust type.

mod config;
mod error;
mod format;
mod metadata;
mod traits;
mod trust;

pub use config::{DEFAULT_MAX_PAYLOAD_SIZE, SerializationConfig};
pub use error::{
    AdapterFailureSource, SerializationError, SerializationErrorKind, SerializationOperation,
};
pub use format::{
    AdapterId, ContentType, MAX_ADAPTER_ID_LEN, MAX_CONTENT_TYPE_LEN, MAX_FORMAT_ID_LEN,
    PayloadVersion, SerializationFormat,
};
pub use metadata::{PayloadMetadata, PayloadMetadataPolicy, SerializedPayload};
pub use traits::{BinarySerializer, Deserializer, Serializer};
pub use trust::SerializationTrustProfile;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_metadata_matches_serialization_boundary() {
        assert_eq!(env!("CARGO_PKG_NAME"), "bluetape-rs-serialization");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.4.0");
    }
}
