use crate::*;
use std::error::Error;
use tracing::info;

#[test]
fn accepts_visible_correlation_ids() {
    let id = CorrelationId::new(" request-1 ").expect("visible id");
    assert_eq!(id.as_str(), " request-1 ");
    assert_eq!(id.to_string(), " request-1 ");
    assert_eq!(CorrelationId::new(" \t"), Err(CorrelationIdError::Blank));
}

#[test]
fn rejects_control_or_oversized_correlation_ids_with_typed_errors() {
    assert_eq!(
        CorrelationId::new("request\nforged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\n' })
    );
    assert_eq!(
        CorrelationId::new("request\rforged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\r' })
    );
    assert_eq!(
        CorrelationId::new("request\tforged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\t' })
    );
    assert_eq!(
        CorrelationId::new("request\u{2028}forged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\u{2028}' })
    );
    assert_eq!(
        CorrelationId::new("request\u{2029}forged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\u{2029}' })
    );
    assert_eq!(
        CorrelationId::new("request\u{202e}forged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\u{202e}' })
    );
    assert_eq!(
        CorrelationId::new("request\u{061c}forged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\u{061c}' })
    );
    assert_eq!(
        CorrelationId::new("request\u{200e}forged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\u{200e}' })
    );
    assert_eq!(
        CorrelationId::new("request\u{200f}forged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\u{200f}' })
    );
    assert_eq!(
        CorrelationId::new("request\u{202a}forged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\u{202a}' })
    );
    assert_eq!(
        CorrelationId::new("request\u{2066}forged"),
        Err(CorrelationIdError::UnsafeCharacter { ch: '\u{2066}' })
    );
    assert_eq!(
        CorrelationId::new("x".repeat(MAX_CORRELATION_ID_LEN + 1)),
        Err(CorrelationIdError::TooLong {
            len: MAX_CORRELATION_ID_LEN + 1,
            max: MAX_CORRELATION_ID_LEN,
        })
    );
    assert!(CorrelationId::new("x".repeat(MAX_CORRELATION_ID_LEN)).is_ok());
}

#[test]
fn correlation_id_error_formats_public_messages() {
    let blank = CorrelationIdError::Blank;
    let too_long = CorrelationIdError::TooLong { len: 257, max: 256 };
    let unsafe_character = CorrelationIdError::UnsafeCharacter { ch: '\u{202e}' };

    assert_eq!(
        blank.to_string(),
        "correlation id must contain visible text"
    );
    assert_eq!(
        too_long.to_string(),
        "correlation id is 257 bytes, exceeding the 256 byte limit"
    );
    assert_eq!(
        unsafe_character.to_string(),
        "correlation id contains unsafe character U+202E"
    );
    assert!(blank.source().is_none());
    assert!(too_long.source().is_none());
    assert!(unsafe_character.source().is_none());
}

#[test]
fn exposes_field_names() {
    assert_eq!(REQUEST_ID_FIELD, "request.id");
    assert_eq!(TASK_ID_FIELD, "task.id");
    assert_eq!(CORRELATION_ID_FIELD, "correlation.id");
}

#[test]
fn scoped_subscriber_does_not_require_global_installation() {
    let subscriber = text_subscriber_with_filter("info").expect("valid filter");
    let value = with_default(subscriber, || {
        info!(request.id = "r1", "captured by scoped subscriber");
        42
    });
    assert_eq!(value, 42);
}

#[test]
fn capture_subscriber_records_logs_for_tests() {
    let captured = CapturedLogs::new();
    let subscriber = capture_subscriber(captured.clone(), "info").expect("valid filter");

    with_default(subscriber, || {
        info!(correlation.id = "c1", "captured event");
    });

    let logs = captured.to_lossy_string();
    assert!(logs.contains("captured event"));
    assert!(logs.contains("correlation.id=\"c1\""));
    captured.clear();
    assert!(captured.to_lossy_string().is_empty());
}

#[test]
fn explicit_filter_builders_reject_invalid_directives() {
    let invalid_filter = "crate1::mod1=warn=info";
    assert!(text_subscriber_with_filter(invalid_filter).is_err());
    assert!(capture_subscriber(CapturedLogs::new(), invalid_filter).is_err());
}

#[test]
fn explicit_filter_builders_disable_regex_value_matching() {
    assert!(text_subscriber_with_filter("target[{field=hello.*}]=info").is_ok());
}
