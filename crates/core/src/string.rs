use crate::error::ValidationError;

/// Returns `Ok(value)` when it is not empty.
///
/// Whitespace-only input is accepted by this helper. Use
/// [`require_not_blank`] when whitespace-only input must be rejected.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::require_not_empty;
///
/// assert_eq!(require_not_empty("name", "alice")?, "alice");
/// assert!(require_not_empty("name", "").is_err());
/// # Ok::<(), bluetape_rs_core::ValidationError>(())
/// ```
///
/// # Errors
///
/// Returns [`ValidationError::Empty`] when `value` has zero bytes.
pub fn require_not_empty(name: impl Into<String>, value: &str) -> Result<&str, ValidationError> {
    if value.is_empty() {
        Err(ValidationError::Empty { name: name.into() })
    } else {
        Ok(value)
    }
}

/// Returns `Ok(value)` when it contains at least one non-whitespace character.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::require_not_blank;
///
/// assert_eq!(require_not_blank("name", " alice " )?, " alice ");
/// assert!(require_not_blank("name", " \t\n").is_err());
/// # Ok::<(), bluetape_rs_core::ValidationError>(())
/// ```
///
/// # Errors
///
/// Returns [`ValidationError::Blank`] when `value` is empty or contains only
/// whitespace characters.
pub fn require_not_blank(name: impl Into<String>, value: &str) -> Result<&str, ValidationError> {
    if has_text(value) {
        Ok(value)
    } else {
        Err(ValidationError::Blank { name: name.into() })
    }
}

/// Returns whether the value contains at least one non-whitespace character.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::has_text;
///
/// assert!(has_text(" bluetape "));
/// assert!(!has_text(" \n\t"));
/// ```
pub fn has_text(value: &str) -> bool {
    value.chars().any(|ch| !ch.is_whitespace())
}

/// Returns `fallback` when `value` is empty.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::empty_to_default;
///
/// assert_eq!(empty_to_default("", "fallback"), "fallback");
/// assert_eq!(empty_to_default(" ", "fallback"), " ");
/// ```
pub fn empty_to_default<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if value.is_empty() { fallback } else { value }
}

/// Returns `fallback` when `value` is empty or only whitespace.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::blank_to_default;
///
/// assert_eq!(blank_to_default(" \n", "fallback"), "fallback");
/// assert_eq!(blank_to_default("value", "fallback"), "value");
/// ```
pub fn blank_to_default<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if has_text(value) { value } else { fallback }
}

/// Truncates `value` to at most `max_bytes` without splitting a UTF-8 scalar.
///
/// The returned slice may be shorter than `max_bytes` when the byte limit falls
/// in the middle of a multi-byte UTF-8 scalar.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::truncate_utf8_bytes;
///
/// assert_eq!(truncate_utf8_bytes("abcdef", 3)?, "abc");
/// assert_eq!(truncate_utf8_bytes("가나다", 4)?, "가");
/// # Ok::<(), bluetape_rs_core::ValidationError>(())
/// ```
///
/// # Errors
///
/// Returns [`ValidationError::NegativeLimit`] when `max_bytes` is negative.
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
