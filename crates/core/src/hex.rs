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
