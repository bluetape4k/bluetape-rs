use bluetape_rs_serialization::{
    AdapterId, BinarySerializer, ContentType, DEFAULT_MAX_PAYLOAD_SIZE, Deserializer,
    MAX_ADAPTER_ID_LEN, MAX_CONTENT_TYPE_LEN, MAX_FORMAT_ID_LEN, PayloadMetadata,
    PayloadMetadataPolicy, PayloadVersion, SerializationConfig, SerializationError,
    SerializationErrorKind, SerializationFormat, SerializationOperation, SerializationTrustProfile,
    SerializedPayload, Serializer,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[test]
fn accepts_valid_metadata_tokens() {
    assert_eq!(
        SerializationFormat::new("binary").unwrap().as_str(),
        "binary"
    );
    assert_eq!(
        SerializationFormat::new("custom.binary/v1")
            .unwrap()
            .as_str(),
        "custom.binary/v1"
    );
    assert_eq!(
        ContentType::new("application/octet-stream")
            .unwrap()
            .as_str(),
        "application/octet-stream"
    );
    assert_eq!(
        AdapterId::new("binary.primary").unwrap().as_str(),
        "binary.primary"
    );
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
    assert_eq!(
        config.trust_profile(),
        SerializationTrustProfile::StaticallyTyped
    );
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
