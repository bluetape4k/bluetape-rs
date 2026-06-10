//! UTF-8 text boundary helpers for codec call sites.

use std::error::Error;
use std::fmt;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

/// Error returned when bytes cannot be decoded as UTF-8 text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TextDecodeError {
    /// The byte sequence is not valid UTF-8.
    InvalidUtf8 {
        /// Byte offset up to which the input was valid UTF-8.
        valid_up_to: usize,
        /// Length in bytes of the invalid sequence when known.
        error_len: Option<usize>,
    },
}

impl fmt::Display for TextDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUtf8 {
                valid_up_to,
                error_len: Some(error_len),
            } => write!(
                f,
                "text input contains invalid UTF-8 sequence of length {error_len} after byte {valid_up_to}"
            ),
            Self::InvalidUtf8 {
                valid_up_to,
                error_len: None,
            } => write!(
                f,
                "text input contains incomplete UTF-8 sequence after byte {valid_up_to}"
            ),
        }
    }
}

impl Error for TextDecodeError {}

impl From<Utf8Error> for TextDecodeError {
    fn from(error: Utf8Error) -> Self {
        Self::InvalidUtf8 {
            valid_up_to: error.valid_up_to(),
            error_len: error.error_len(),
        }
    }
}

impl From<FromUtf8Error> for TextDecodeError {
    fn from(error: FromUtf8Error) -> Self {
        error.utf8_error().into()
    }
}

/// Encodes UTF-8 text into owned bytes.
///
/// This helper belongs to `bluetape-rs-codec` only as the explicit text-to-byte
/// boundary used before binary encoders. It is not a general string utility.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::{encode_base64_url_unpadded, encode_utf8_text};
///
/// assert_eq!(encode_base64_url_unpadded(encode_utf8_text("Hello")), "SGVsbG8");
/// ```
#[must_use]
pub fn encode_utf8_text(text: impl AsRef<str>) -> Vec<u8> {
    text.as_ref().as_bytes().to_vec()
}

/// Decodes owned bytes into UTF-8 text without lossy replacement.
///
/// Use this after binary decoders when the service contract requires valid
/// UTF-8 text. Invalid UTF-8 returns a typed error with byte-position details.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::{decode_base64_url_unpadded, decode_utf8_text};
///
/// let bytes = decode_base64_url_unpadded("SGVsbG8")?;
///
/// assert_eq!(decode_utf8_text(bytes)?, "Hello");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// Returns [`TextDecodeError::InvalidUtf8`] when the input bytes are not valid
/// UTF-8.
pub fn decode_utf8_text(bytes: impl Into<Vec<u8>>) -> Result<String, TextDecodeError> {
    String::from_utf8(bytes.into()).map_err(Into::into)
}

/// Decodes bytes into UTF-8 text using replacement characters for invalid data.
///
/// This helper is intentionally named `lossy` so callers must opt in to data
/// replacement instead of silently accepting corrupted text.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::decode_utf8_text_lossy;
///
/// assert_eq!(decode_utf8_text_lossy([b'a', 0xff, b'z']), "a\u{fffd}z");
/// ```
#[must_use]
pub fn decode_utf8_text_lossy(bytes: impl AsRef<[u8]>) -> String {
    String::from_utf8_lossy(bytes.as_ref()).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decode_base64_url_unpadded, encode_base64_url_unpadded};

    #[test]
    fn encodes_utf8_text_for_binary_codecs() {
        assert_eq!(encode_utf8_text(""), Vec::<u8>::new());
        assert_eq!(encode_utf8_text("blue테이프"), "blue테이프".as_bytes());
        assert_eq!(
            encode_base64_url_unpadded(encode_utf8_text("blue테이프")),
            "Ymx1Ze2FjOydtO2UhA"
        );
    }

    #[test]
    fn decodes_utf8_text_after_binary_decoding() {
        let bytes =
            decode_base64_url_unpadded("Ymx1Ze2FjOydtO2UhA").expect("valid encoded text bytes");

        assert_eq!(
            decode_utf8_text(bytes).expect("valid UTF-8 text"),
            "blue테이프"
        );
    }

    #[test]
    fn rejects_invalid_utf8_without_lossy_replacement() {
        assert_eq!(
            decode_utf8_text([b'a', 0xff, b'z']).expect_err("invalid UTF-8 should fail"),
            TextDecodeError::InvalidUtf8 {
                valid_up_to: 1,
                error_len: Some(1)
            }
        );
    }

    #[test]
    fn reports_incomplete_utf8_sequence() {
        assert_eq!(
            decode_utf8_text([0xe2, 0x82]).expect_err("incomplete UTF-8 should fail"),
            TextDecodeError::InvalidUtf8 {
                valid_up_to: 0,
                error_len: None
            }
        );
    }

    #[test]
    fn decodes_utf8_text_lossy_only_when_callers_opt_in() {
        assert_eq!(decode_utf8_text_lossy([b'a', 0xff, b'z']), "a\u{fffd}z");
    }

    #[test]
    fn formats_decode_errors_for_service_diagnostics() {
        assert_eq!(
            TextDecodeError::InvalidUtf8 {
                valid_up_to: 2,
                error_len: Some(1)
            }
            .to_string(),
            "text input contains invalid UTF-8 sequence of length 1 after byte 2"
        );
        assert_eq!(
            TextDecodeError::InvalidUtf8 {
                valid_up_to: 3,
                error_len: None
            }
            .to_string(),
            "text input contains incomplete UTF-8 sequence after byte 3"
        );
    }
}
