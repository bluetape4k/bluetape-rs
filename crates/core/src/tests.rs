use crate::*;
use std::error::Error;

#[test]
fn validates_strings() {
    assert_eq!(
        require_not_empty("name", " ").expect("space is non-empty"),
        " "
    );
    assert!(matches!(
        require_not_empty("name", ""),
        Err(ValidationError::Empty { .. })
    ));
    assert_eq!(require_not_blank("name", "blue").expect("text"), "blue");
    assert!(matches!(
        require_not_blank("name", " \t"),
        Err(ValidationError::Blank { .. })
    ));
}

#[test]
fn validates_ranges_and_numbers() {
    assert_eq!(require_in_range("value", 5, 1, 5).expect("inclusive"), 5);
    assert!(matches!(
        require_in_range("value", 6, 1, 5),
        Err(ValidationError::OutOfRange {
            kind: RangeKind::Inclusive,
            ..
        })
    ));
    assert_eq!(
        require_in_half_open_range("value", 4, 1, 5).expect("half-open"),
        4
    );
    assert!(matches!(
        require_in_half_open_range("value", 5, 1, 5),
        Err(ValidationError::OutOfRange {
            kind: RangeKind::HalfOpen,
            ..
        })
    ));
    assert_eq!(require_positive("count", 1).expect("positive"), 1);
    assert!(matches!(
        require_positive("count", 0),
        Err(ValidationError::NotPositive { .. })
    ));
    assert_eq!(require_non_negative("count", 0).expect("zero"), 0);
    assert!(matches!(
        require_non_negative("count", -1),
        Err(ValidationError::Negative { .. })
    ));
}

#[test]
fn rejects_non_finite_float_numbers() {
    assert!(matches!(
        require_positive("count", f64::NAN),
        Err(ValidationError::NonFinite { .. })
    ));
    assert!(matches!(
        require_non_negative("count", f64::INFINITY),
        Err(ValidationError::NonFinite { .. })
    ));
    assert!(matches!(
        require_in_range("value", f32::NAN, 0.0, 1.0),
        Err(ValidationError::NonFinite { .. })
    ));
    assert!(matches!(
        clamp(f64::NAN, 0.0, 1.0),
        Err(ValidationError::NonFinite { .. })
    ));
}

#[test]
fn handles_string_defaults_and_unicode_truncation() {
    assert!(has_text(" blue "));
    assert!(!has_text(" \t "));
    assert_eq!(empty_to_default("", "fallback"), "fallback");
    assert_eq!(empty_to_default(" ", "fallback"), " ");
    assert_eq!(blank_to_default(" ", "fallback"), "fallback");
    assert_eq!(
        truncate_utf8_bytes("Hello, 세계", 9).expect("truncate"),
        "Hello, "
    );
    assert_eq!(
        truncate_utf8_bytes("안녕하세요", 7).expect("truncate"),
        "안녕"
    );
    assert!(matches!(
        truncate_utf8_bytes("abc", -1),
        Err(ValidationError::NegativeLimit { .. })
    ));
}

#[test]
fn handles_clamp_and_hex_helpers() {
    assert_eq!(clamp(120, 0, 100).expect("clamp"), 100);
    assert!(matches!(
        clamp(1, 10, 1),
        Err(ValidationError::InvalidRange { .. })
    ));
    assert!(is_hex_digit('f'));
    assert!(!is_hex_digit('g'));
    assert!(is_prefixed_hex("0x1234"));
    assert!(is_prefixed_hex("-#cafe"));
    assert!(!is_prefixed_hex("1234"));
    assert!(!is_prefixed_hex("0x"));
    assert!(!is_prefixed_hex("0xxyz"));
}

#[test]
fn validation_error_implements_std_error() {
    fn assert_error(_: &dyn Error) {}

    let error = require_not_blank("name", "").expect_err("blank");
    assert_error(&error);
    assert_eq!(error.to_string(), "name must not be blank");
}
