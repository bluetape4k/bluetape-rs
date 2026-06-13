/// Describes how much serialized data can influence deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SerializationTrustProfile {
    /// Private caches or queues controlled by one trusted deployment boundary.
    TrustedInternal,
    /// Payloads may carry type metadata, but only through explicit allowlists.
    AllowListedTypes,
    /// Payloads do not select runtime types; callers provide the Rust target
    /// type.
    #[default]
    StaticallyTyped,
    /// Migration-only boundary for fully trusted legacy data.
    UnsafeLegacyCompatibility,
}
