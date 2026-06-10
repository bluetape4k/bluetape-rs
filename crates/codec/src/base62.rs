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

#[cfg(test)]
mod tests {
    use super::*;

    const STRESS_WORKERS: usize = 8;
    const STRESS_ROUNDS: usize = 512;

    #[test]
    fn encodes_empty_input() {
        assert_eq!(encode_base62([]), "");
    }

    #[test]
    fn decodes_empty_input() {
        assert_eq!(
            decode_base62("").expect("empty base62 is valid"),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn encodes_known_byte_oriented_vector() {
        assert_eq!(encode_base62(b"Hello, World!"), "1wJfrzvdbtXUOlUjUf");
    }

    #[test]
    fn round_trips_binary_data() {
        let bytes = [0x00, 0x00, 0x01, 0x7f, 0x80, 0xab, 0xcd, 0xef, 0xff];

        assert_eq!(
            decode_base62(encode_base62(bytes)).expect("valid base62"),
            bytes
        );
    }

    #[test]
    fn preserves_leading_zero_bytes_as_zeroes() {
        assert_eq!(encode_base62([0, 0, 1]), "001");
        assert_eq!(decode_base62("001").expect("valid base62"), [0, 0, 1]);
    }

    #[test]
    fn rejects_non_alphabet_bytes() {
        assert_eq!(
            decode_base62("Foo Bar").expect_err("space should fail"),
            Base62DecodeError::InvalidCharacter {
                index: 3,
                byte: b' '
            }
        );
        assert_eq!(
            decode_base62("abc_def").expect_err("underscore should fail"),
            Base62DecodeError::InvalidCharacter {
                index: 3,
                byte: b'_'
            }
        );
    }

    #[test]
    fn reports_utf8_bytes_by_byte_position() {
        assert_eq!(
            decode_base62("12é").expect_err("non-ascii byte should fail"),
            Base62DecodeError::InvalidCharacter {
                index: 2,
                byte: 0xc3
            }
        );
    }

    #[test]
    fn formats_decode_errors_for_service_diagnostics() {
        assert_eq!(
            Base62DecodeError::InvalidCharacter {
                index: 4,
                byte: b'_'
            }
            .to_string(),
            "base62 input contains invalid byte 0x5f at position 4"
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
                        let encoded = encode_base62(case);
                        let decoded = decode_base62(&encoded).expect("valid Base62 round trip");
                        assert_eq!(decoded, case, "worker={worker} round={round}");
                    }
                });
            }
        });
    }
}
