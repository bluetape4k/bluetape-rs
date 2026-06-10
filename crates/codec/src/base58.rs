//! Strict Base58 encoding and decoding.

use std::error::Error;
use std::fmt;

use crate::base_n::{BaseNDecodeError, decode_base_n, encode_base_n};

const BITCOIN_BASE58_ALPHABET: &[u8] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Error returned when Base58 decoding rejects caller-owned input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Base58DecodeError {
    /// The encoded text contains a byte outside the Bitcoin Base58 alphabet.
    InvalidCharacter {
        /// Zero-based byte position of the invalid byte.
        index: usize,
        /// Invalid byte value.
        byte: u8,
    },
}

impl fmt::Display for Base58DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter { index, byte } => {
                write!(
                    f,
                    "base58 input contains invalid byte 0x{byte:02x} at position {index}"
                )
            }
        }
    }
}

impl Error for Base58DecodeError {}

impl From<BaseNDecodeError> for Base58DecodeError {
    fn from(error: BaseNDecodeError) -> Self {
        match error {
            BaseNDecodeError::InvalidCharacter { index, byte } => {
                Self::InvalidCharacter { index, byte }
            }
        }
    }
}

/// Encodes bytes with the Bitcoin Base58 alphabet.
///
/// Leading zero bytes are preserved as leading `1` characters.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::encode_base58;
///
/// assert_eq!(encode_base58(b"Hello, World!"), "72k1xXWG59fYdzSNoA");
/// ```
#[must_use]
pub fn encode_base58(bytes: impl AsRef<[u8]>) -> String {
    encode_base_n(bytes.as_ref(), BITCOIN_BASE58_ALPHABET)
}

/// Decodes Bitcoin Base58 text into bytes.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::decode_base58;
///
/// assert_eq!(decode_base58("72k1xXWG59fYdzSNoA")?, b"Hello, World!");
/// # Ok::<(), bluetape_rs_codec::Base58DecodeError>(())
/// ```
///
/// # Errors
///
/// Returns [`Base58DecodeError`] when input contains bytes outside the Bitcoin
/// Base58 alphabet.
pub fn decode_base58(encoded: impl AsRef<str>) -> Result<Vec<u8>, Base58DecodeError> {
    decode_base_n(encoded.as_ref(), BITCOIN_BASE58_ALPHABET).map_err(Into::into)
}
