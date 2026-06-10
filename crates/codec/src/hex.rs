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
