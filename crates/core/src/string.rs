use crate::error::ValidationError;

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
