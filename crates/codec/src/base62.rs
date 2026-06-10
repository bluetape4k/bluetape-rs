//! Strict byte-oriented Base62 encoding and decoding.

use std::error::Error;
use std::fmt;

use crate::base_n::{BaseNDecodeError, decode_base_n, encode_base_n};

const BASE62_ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Error returned when Base62 decoding rejects caller-owned input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Base62DecodeError {
    /// The encoded text contains a byte outside the selected Base62 alphabet.
    InvalidCharacter {
        /// Zero-based byte position of the invalid byte.
        index: usize,
        /// Invalid byte value.
        byte: u8,
    },
}

impl fmt::Display for Base62DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter { index, byte } => {
                write!(
                    f,
                    "base62 input contains invalid byte 0x{byte:02x} at position {index}"
                )
            }
        }
    }
}

impl Error for Base62DecodeError {}

impl From<BaseNDecodeError> for Base62DecodeError {
    fn from(error: BaseNDecodeError) -> Self {
        match error {
            BaseNDecodeError::InvalidCharacter { index, byte } => {
                Self::InvalidCharacter { index, byte }
            }
        }
    }
}

/// Encodes bytes with the bluetape Base62 alphabet.
///
/// The alphabet is `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`.
/// This API is byte-oriented; UUID and integer rendering should build on top of
/// it in a separate ID-focused crate. Leading zero bytes are preserved as
/// leading `0` characters.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::encode_base62;
///
/// assert_eq!(encode_base62(b"Hello, World!"), "1wJfrzvdbtXUOlUjUf");
/// ```
#[must_use]
pub fn encode_base62(bytes: impl AsRef<[u8]>) -> String {
    encode_base_n(bytes.as_ref(), BASE62_ALPHABET)
}

/// Decodes byte-oriented Base62 text into bytes.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::decode_base62;
///
/// assert_eq!(decode_base62("1wJfrzvdbtXUOlUjUf")?, b"Hello, World!");
/// # Ok::<(), bluetape_rs_codec::Base62DecodeError>(())
/// ```
///
/// # Errors
///
/// Returns [`Base62DecodeError`] when input contains bytes outside the Base62
/// alphabet.
pub fn decode_base62(encoded: impl AsRef<str>) -> Result<Vec<u8>, Base62DecodeError> {
    decode_base_n(encoded.as_ref(), BASE62_ALPHABET).map_err(Into::into)
}
