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

#[cfg(test)]
mod tests {
    use super::*;

    const STRESS_WORKERS: usize = 8;
    const STRESS_ROUNDS: usize = 512;

    #[test]
    fn encodes_empty_input() {
        assert_eq!(encode_base58([]), "");
    }

    #[test]
    fn decodes_empty_input() {
        assert_eq!(
            decode_base58("").expect("empty base58 is valid"),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn encodes_known_bitcoin_base58_vector() {
        assert_eq!(encode_base58(b"Hello World!"), "2NEpo7TZRRrLZSi2U");
        assert_eq!(encode_base58(b"Hello, World!"), "72k1xXWG59fYdzSNoA");
    }

    #[test]
    fn round_trips_binary_data() {
        let bytes = [0x00, 0x00, 0x01, 0x7f, 0x80, 0xab, 0xcd, 0xef, 0xff];

        assert_eq!(
            decode_base58(encode_base58(bytes)).expect("valid base58"),
            bytes
        );
    }

    #[test]
    fn preserves_leading_zero_bytes_as_ones() {
        assert_eq!(encode_base58([0, 0, 1]), "112");
        assert_eq!(decode_base58("112").expect("valid base58"), [0, 0, 1]);
    }

    #[test]
    fn rejects_ambiguous_and_non_alphabet_bytes() {
        assert_eq!(
            decode_base58("0OIl").expect_err("ambiguous characters should fail"),
            Base58DecodeError::InvalidCharacter {
                index: 0,
                byte: b'0'
            }
        );
        assert_eq!(
            decode_base58("12 3").expect_err("space should fail"),
            Base58DecodeError::InvalidCharacter {
                index: 2,
                byte: b' '
            }
        );
    }

    #[test]
    fn reports_utf8_bytes_by_byte_position() {
        assert_eq!(
            decode_base58("12é").expect_err("non-ascii byte should fail"),
            Base58DecodeError::InvalidCharacter {
                index: 2,
                byte: 0xc3
            }
        );
    }

    #[test]
    fn formats_decode_errors_for_service_diagnostics() {
        assert_eq!(
            Base58DecodeError::InvalidCharacter {
                index: 3,
                byte: b'0'
            }
            .to_string(),
            "base58 input contains invalid byte 0x30 at position 3"
        );
    }

    #[test]
    fn stress_round_trips_are_stable_across_threads() {
        let cases: &[&[u8]] = &[
            b"",
            b"Hello World!",
            b"Hello, World!",
            &[0, 0, 0, 1],
            &[0, 1, 2, 3, 254, 255],
            &[255; 32],
        ];

        std::thread::scope(|scope| {
            for worker in 0..STRESS_WORKERS {
                scope.spawn(move || {
                    for round in 0..STRESS_ROUNDS {
                        let case = cases[(worker + round) % cases.len()];
                        let encoded = encode_base58(case);
                        let decoded = decode_base58(&encoded).expect("valid Base58 round trip");
                        assert_eq!(decoded, case, "worker={worker} round={round}");
                    }
                });
            }
        });
    }
}
