//! Strict hexadecimal encoding and decoding.

use std::error::Error;
use std::fmt;

const LOWER_HEX: &[u8; 16] = b"0123456789abcdef";
const UPPER_HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Error returned when strict hexadecimal decoding rejects caller-owned input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HexDecodeError {
    /// The encoded text has an odd byte length.
    OddLength {
        /// Number of bytes in the encoded text.
        len: usize,
    },
    /// The encoded text contains a non-hexadecimal byte.
    InvalidCharacter {
        /// Zero-based byte position of the invalid character.
        index: usize,
        /// Invalid byte value.
        byte: u8,
    },
}

impl fmt::Display for HexDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OddLength { len } => {
                write!(f, "hex input must have even byte length, got {len}")
            }
            Self::InvalidCharacter { index, byte } => {
                write!(
                    f,
                    "hex input contains invalid byte 0x{byte:02x} at position {index}"
                )
            }
        }
    }
}

impl Error for HexDecodeError {}

/// Encodes bytes as lowercase hexadecimal text.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::encode_hex_lower;
///
/// assert_eq!(encode_hex_lower([0x00, 0xab, 0xff]), "00abff");
/// ```
#[must_use]
pub fn encode_hex_lower(bytes: impl AsRef<[u8]>) -> String {
    encode_hex_with_alphabet(bytes.as_ref(), LOWER_HEX)
}

/// Encodes bytes as uppercase hexadecimal text.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::encode_hex_upper;
///
/// assert_eq!(encode_hex_upper([0x00, 0xab, 0xff]), "00ABFF");
/// ```
#[must_use]
pub fn encode_hex_upper(bytes: impl AsRef<[u8]>) -> String {
    encode_hex_with_alphabet(bytes.as_ref(), UPPER_HEX)
}

/// Decodes strict hexadecimal text into bytes.
///
/// The decoder accepts uppercase and lowercase ASCII hexadecimal digits. It
/// rejects odd-length input, prefixes such as `0x`, whitespace, separators, and
/// any non-ASCII digit.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::decode_hex;
///
/// assert_eq!(decode_hex("00abFF")?, vec![0x00, 0xab, 0xff]);
/// # Ok::<(), bluetape_rs_codec::HexDecodeError>(())
/// ```
///
/// # Errors
///
/// Returns [`HexDecodeError::OddLength`] when the input has an odd byte length,
/// or [`HexDecodeError::InvalidCharacter`] with the invalid byte position when
/// any character is not an ASCII hexadecimal digit.
pub fn decode_hex(encoded: impl AsRef<str>) -> Result<Vec<u8>, HexDecodeError> {
    let encoded = encoded.as_ref();
    let bytes = encoded.as_bytes();
    if bytes.len() % 2 != 0 {
        return Err(HexDecodeError::OddLength { len: bytes.len() });
    }

    let mut decoded = Vec::with_capacity(bytes.len() / 2);
    for (pair_index, chunk) in bytes.chunks_exact(2).enumerate() {
        let high_index = pair_index * 2;
        let high = decode_nibble(chunk[0], high_index)?;
        let low = decode_nibble(chunk[1], high_index + 1)?;
        decoded.push((high << 4) | low);
    }

    Ok(decoded)
}

fn encode_hex_with_alphabet(bytes: &[u8], alphabet: &[u8; 16]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(alphabet[(byte >> 4) as usize] as char);
        encoded.push(alphabet[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn decode_nibble(byte: u8, index: usize) -> Result<u8, HexDecodeError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(HexDecodeError::InvalidCharacter { index, byte }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_empty_input() {
        assert_eq!(encode_hex_lower([]), "");
        assert_eq!(encode_hex_upper([]), "");
    }

    #[test]
    fn encodes_binary_data_with_lowercase_and_uppercase_variants() {
        let bytes = [0x00, 0x01, 0x7f, 0x80, 0xab, 0xcd, 0xef, 0xff];

        assert_eq!(encode_hex_lower(bytes), "00017f80abcdefff");
        assert_eq!(encode_hex_upper(bytes), "00017F80ABCDEFFF");
    }

    #[test]
    fn decodes_empty_input() {
        assert_eq!(
            decode_hex("").expect("empty hex is valid"),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn decodes_mixed_case_hex() {
        assert_eq!(
            decode_hex("00017F80abcdefff").expect("valid mixed-case hex"),
            vec![0x00, 0x01, 0x7f, 0x80, 0xab, 0xcd, 0xef, 0xff]
        );
    }

    #[test]
    fn rejects_odd_length_with_byte_length() {
        assert_eq!(
            decode_hex("abc").expect_err("odd-length hex should fail"),
            HexDecodeError::OddLength { len: 3 }
        );
    }

    #[test]
    fn rejects_invalid_character_with_byte_position() {
        assert_eq!(
            decode_hex("00xz").expect_err("invalid hex should fail"),
            HexDecodeError::InvalidCharacter {
                index: 2,
                byte: b'x'
            }
        );
    }

    #[test]
    fn rejects_invalid_second_nibble_with_position() {
        assert_eq!(
            decode_hex("0g").expect_err("invalid low nibble should fail"),
            HexDecodeError::InvalidCharacter {
                index: 1,
                byte: b'g'
            }
        );
    }

    #[test]
    fn rejects_non_ascii_input_by_byte_position() {
        assert_eq!(
            decode_hex("00é").expect_err("non-ascii hex should fail"),
            HexDecodeError::InvalidCharacter {
                index: 2,
                byte: 0xc3
            }
        );
    }

    #[test]
    fn formats_decode_errors_for_service_diagnostics() {
        assert_eq!(
            HexDecodeError::OddLength { len: 7 }.to_string(),
            "hex input must have even byte length, got 7"
        );
        assert_eq!(
            HexDecodeError::InvalidCharacter {
                index: 4,
                byte: b'_'
            }
            .to_string(),
            "hex input contains invalid byte 0x5f at position 4"
        );
    }
}
