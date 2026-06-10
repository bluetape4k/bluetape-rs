/// Returns whether `ch` is an ASCII hexadecimal digit.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::is_hex_digit;
///
/// assert!(is_hex_digit('a'));
/// assert!(is_hex_digit('F'));
/// assert!(is_hex_digit('9'));
/// assert!(!is_hex_digit('g'));
/// ```
pub fn is_hex_digit(ch: char) -> bool {
    ch.is_ascii_hexdigit()
}

/// Returns whether `value` uses `0x`, `0X`, `#`, `-0x`, `-0X`, or `-#` hex notation.
///
/// The sign and prefix are recognized after trimming leading and trailing
/// whitespace. At least one hexadecimal digit must follow the prefix.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::is_prefixed_hex;
///
/// assert!(is_prefixed_hex("0xCAFE"));
/// assert!(is_prefixed_hex("-#ff"));
/// assert!(!is_prefixed_hex("CAFE"));
/// assert!(!is_prefixed_hex("0x"));
/// ```
pub fn is_prefixed_hex(value: &str) -> bool {
    let value = value.trim();
    let value = value.strip_prefix('-').unwrap_or(value);
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .or_else(|| value.strip_prefix('#'));

    matches!(digits, Some(digits) if !digits.is_empty() && digits.chars().all(is_hex_digit))
}
