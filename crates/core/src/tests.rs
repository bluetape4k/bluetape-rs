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

#[test]
fn validation_error_formats_every_public_variant() {
    let cases = [
        (
            ValidationError::Empty {
                name: "name".to_owned(),
            },
            "name must not be empty",
        ),
        (
            ValidationError::Blank {
                name: "name".to_owned(),
            },
            "name must not be blank",
        ),
        (
            ValidationError::InvalidRange {
                name: "port".to_owned(),
                lower: "10".to_owned(),
                upper: "1".to_owned(),
                kind: RangeKind::Inclusive,
            },
            "port inclusive range is invalid: lower 10 must be less than upper 1",
        ),
        (
            ValidationError::OutOfRange {
                name: "port".to_owned(),
                value: "11".to_owned(),
                lower: "1".to_owned(),
                upper: "10".to_owned(),
                kind: RangeKind::Inclusive,
            },
            "port[11] must be in range [1, 10]",
        ),
        (
            ValidationError::OutOfRange {
                name: "index".to_owned(),
                value: "10".to_owned(),
                lower: "0".to_owned(),
                upper: "10".to_owned(),
                kind: RangeKind::HalfOpen,
            },
            "index[10] must be in range [0, 10)",
        ),
        (
            ValidationError::NotPositive {
                name: "workers".to_owned(),
                value: "0".to_owned(),
            },
            "workers[0] must be positive",
        ),
        (
            ValidationError::Negative {
                name: "count".to_owned(),
                value: "-1".to_owned(),
            },
            "count[-1] must be non-negative",
        ),
        (
            ValidationError::NonFinite {
                name: "ratio".to_owned(),
                value: "NaN".to_owned(),
            },
            "ratio[NaN] must be finite",
        ),
        (
            ValidationError::NegativeLimit {
                name: "max_bytes".to_owned(),
                value: -1,
            },
            "max_bytes[-1] must be non-negative",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
        assert!(error.source().is_none());
    }
    assert_eq!(RangeKind::Inclusive.to_string(), "inclusive");
    assert_eq!(RangeKind::HalfOpen.to_string(), "half-open");
}
