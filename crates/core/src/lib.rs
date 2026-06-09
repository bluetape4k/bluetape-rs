//! Core validation, string, and numeric helpers.
//!
//! Prefer the Rust standard library when it already expresses the operation
//! clearly. This crate is for small repeated backend-service patterns.

use std::error::Error;
use std::fmt::{self, Display};

/// Error returned when caller-owned input violates a validation contract.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationError {
    /// A named value was empty.
    Empty {
        /// Caller-facing value name.
        name: String,
    },
    /// A named value was empty or contained only whitespace.
    Blank {
        /// Caller-facing value name.
        name: String,
    },
    /// A range definition was invalid.
    InvalidRange {
        /// Caller-facing range name.
        name: String,
        /// Rendered lower bound.
        lower: String,
        /// Rendered upper bound.
        upper: String,
        /// Boundary semantics used for the range.
        kind: RangeKind,
    },
    /// A named value was outside the accepted range.
    OutOfRange {
        /// Caller-facing value name.
        name: String,
        /// Rendered rejected value.
        value: String,
        /// Rendered lower bound.
        lower: String,
        /// Rendered upper bound.
        upper: String,
        /// Boundary semantics used for the range.
        kind: RangeKind,
    },
    /// A named value was expected to be positive.
    NotPositive {
        /// Caller-facing value name.
        name: String,
        /// Rendered rejected value.
        value: String,
    },
    /// A named value was expected to be non-negative.
    Negative {
        /// Caller-facing value name.
        name: String,
        /// Rendered rejected value.
        value: String,
    },
    /// A named floating-point value was expected to be finite.
    NonFinite {
        /// Caller-facing value name.
        name: String,
        /// Rendered rejected value.
        value: String,
    },
    /// A byte limit was invalid.
    NegativeLimit {
        /// Caller-facing limit name.
        name: String,
        /// Rejected byte limit.
        value: isize,
    },
}

/// Range boundary semantics for validation errors.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RangeKind {
    /// Inclusive range: `[lower, upper]`.
    Inclusive,
    /// Half-open range: `[lower, upper)`.
    HalfOpen,
}

impl Display for RangeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inclusive => f.write_str("inclusive"),
            Self::HalfOpen => f.write_str("half-open"),
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { name } => write!(f, "{name} must not be empty"),
            Self::Blank { name } => write!(f, "{name} must not be blank"),
            Self::InvalidRange {
                name,
                lower,
                upper,
                kind,
            } => write!(
                f,
                "{name} {kind} range is invalid: lower {lower} must be less than upper {upper}"
            ),
            Self::OutOfRange {
                name,
                value,
                lower,
                upper,
                kind,
            } => match kind {
                RangeKind::Inclusive => {
                    write!(f, "{name}[{value}] must be in range [{lower}, {upper}]")
                }
                RangeKind::HalfOpen => {
                    write!(f, "{name}[{value}] must be in range [{lower}, {upper})")
                }
            },
            Self::NotPositive { name, value } => write!(f, "{name}[{value}] must be positive"),
            Self::Negative { name, value } => write!(f, "{name}[{value}] must be non-negative"),
            Self::NonFinite { name, value } => write!(f, "{name}[{value}] must be finite"),
            Self::NegativeLimit { name, value } => {
                write!(f, "{name}[{value}] must be non-negative")
            }
        }
    }
}

impl Error for ValidationError {}

/// Number types supported by the small numeric helpers.
pub trait Number: Copy + PartialOrd + Display {
    /// Returns whether this value can be compared reliably by validation helpers.
    fn is_valid_number(self) -> bool;
}

macro_rules! impl_number {
    ($($type:ty),* $(,)?) => {
        $(
            impl Number for $type {
                fn is_valid_number(self) -> bool {
                    true
                }
            }
        )*
    };
}

impl_number!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl Number for f32 {
    fn is_valid_number(self) -> bool {
        self.is_finite()
    }
}

impl Number for f64 {
    fn is_valid_number(self) -> bool {
        self.is_finite()
    }
}

/// Returns `Ok(value)` when it is not empty.
pub fn require_not_empty(name: impl Into<String>, value: &str) -> Result<&str, ValidationError> {
    if value.is_empty() {
        Err(ValidationError::Empty { name: name.into() })
    } else {
        Ok(value)
    }
}

/// Returns `Ok(value)` when it contains at least one non-whitespace character.
pub fn require_not_blank(name: impl Into<String>, value: &str) -> Result<&str, ValidationError> {
    if has_text(value) {
        Ok(value)
    } else {
        Err(ValidationError::Blank { name: name.into() })
    }
}

/// Returns `Ok(value)` when it is inside the inclusive `[lower, upper]` range.
pub fn require_in_range<T>(
    name: impl Into<String>,
    value: T,
    lower: T,
    upper: T,
) -> Result<T, ValidationError>
where
    T: Number,
{
    let name = name.into();
    require_finite_number(&name, value)?;
    require_finite_number(&name, lower)?;
    require_finite_number(&name, upper)?;
    if lower > upper {
        return Err(invalid_range(name, lower, upper, RangeKind::Inclusive));
    }
    if value < lower || value > upper {
        return Err(out_of_range(
            name,
            value,
            lower,
            upper,
            RangeKind::Inclusive,
        ));
    }
    Ok(value)
}

/// Returns `Ok(value)` when it is inside the half-open `[lower, upper)` range.
pub fn require_in_half_open_range<T>(
    name: impl Into<String>,
    value: T,
    lower: T,
    upper: T,
) -> Result<T, ValidationError>
where
    T: Number,
{
    let name = name.into();
    require_finite_number(&name, value)?;
    require_finite_number(&name, lower)?;
    require_finite_number(&name, upper)?;
    if lower >= upper {
        return Err(invalid_range(name, lower, upper, RangeKind::HalfOpen));
    }
    if value < lower || value >= upper {
        return Err(out_of_range(name, value, lower, upper, RangeKind::HalfOpen));
    }
    Ok(value)
}

/// Returns `Ok(value)` when it is greater than zero.
pub fn require_positive<T>(name: impl Into<String>, value: T) -> Result<T, ValidationError>
where
    T: Number + Default,
{
    let name = name.into();
    require_finite_number(&name, value)?;
    if value <= T::default() {
        Err(ValidationError::NotPositive {
            name,
            value: value.to_string(),
        })
    } else {
        Ok(value)
    }
}

/// Returns `Ok(value)` when it is greater than or equal to zero.
pub fn require_non_negative<T>(name: impl Into<String>, value: T) -> Result<T, ValidationError>
where
    T: Number + Default,
{
    let name = name.into();
    require_finite_number(&name, value)?;
    if value < T::default() {
        Err(ValidationError::Negative {
            name,
            value: value.to_string(),
        })
    } else {
        Ok(value)
    }
}

/// Returns whether the value contains at least one non-whitespace character.
pub fn has_text(value: &str) -> bool {
    value.chars().any(|ch| !ch.is_whitespace())
}

/// Returns `fallback` when `value` is empty.
pub fn empty_to_default<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if value.is_empty() { fallback } else { value }
}

/// Returns `fallback` when `value` is empty or only whitespace.
pub fn blank_to_default<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if has_text(value) { value } else { fallback }
}

/// Truncates `value` to at most `max_bytes` without splitting a UTF-8 scalar.
pub fn truncate_utf8_bytes(value: &str, max_bytes: isize) -> Result<&str, ValidationError> {
    if max_bytes < 0 {
        return Err(ValidationError::NegativeLimit {
            name: "max_bytes".to_string(),
            value: max_bytes,
        });
    }

    let mut max_bytes = max_bytes as usize;
    if value.len() <= max_bytes {
        return Ok(value);
    }

    while max_bytes > 0 && !value.is_char_boundary(max_bytes) {
        max_bytes -= 1;
    }
    Ok(&value[..max_bytes])
}

/// Returns `value` constrained to the inclusive `[lower, upper]` range.
pub fn clamp<T>(value: T, lower: T, upper: T) -> Result<T, ValidationError>
where
    T: Number,
{
    require_finite_number("value", value)?;
    require_finite_number("lower", lower)?;
    require_finite_number("upper", upper)?;
    if lower > upper {
        return Err(invalid_range(
            "range".to_string(),
            lower,
            upper,
            RangeKind::Inclusive,
        ));
    }
    if value < lower {
        Ok(lower)
    } else if value > upper {
        Ok(upper)
    } else {
        Ok(value)
    }
}

fn require_finite_number<T>(name: &str, value: T) -> Result<(), ValidationError>
where
    T: Number,
{
    if value.is_valid_number() {
        Ok(())
    } else {
        Err(ValidationError::NonFinite {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

/// Returns whether `ch` is an ASCII hexadecimal digit.
pub fn is_hex_digit(ch: char) -> bool {
    ch.is_ascii_hexdigit()
}

/// Returns whether `value` uses `0x`, `0X`, `#`, `-0x`, `-0X`, or `-#` hex notation.
pub fn is_prefixed_hex(value: &str) -> bool {
    let value = value.trim();
    let value = value.strip_prefix('-').unwrap_or(value);
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .or_else(|| value.strip_prefix('#'));

    matches!(digits, Some(digits) if !digits.is_empty() && digits.chars().all(is_hex_digit))
}

fn invalid_range<T>(name: String, lower: T, upper: T, kind: RangeKind) -> ValidationError
where
    T: Display,
{
    ValidationError::InvalidRange {
        name,
        lower: lower.to_string(),
        upper: upper.to_string(),
        kind,
    }
}

fn out_of_range<T>(name: String, value: T, lower: T, upper: T, kind: RangeKind) -> ValidationError
where
    T: Display,
{
    ValidationError::OutOfRange {
        name,
        value: value.to_string(),
        lower: lower.to_string(),
        upper: upper.to_string(),
        kind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
