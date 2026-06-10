//! Strict Base64 encoding and decoding.

use std::error::Error;
use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD};

/// Error returned when Base64 decoding rejects caller-owned input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Base64DecodeError {
    /// The encoded text contains a byte outside the selected Base64 alphabet.
    InvalidByte {
        /// Zero-based byte position of the invalid byte.
        index: usize,
        /// Invalid byte value.
        byte: u8,
    },
    /// The encoded text length is invalid for Base64 decoding.
    InvalidLength {
        /// Number of bytes in the encoded text.
        len: usize,
    },
    /// The final non-padding symbol has impossible trailing bits.
    InvalidLastSymbol {
        /// Zero-based byte position of the invalid symbol.
        index: usize,
        /// Invalid byte value.
        byte: u8,
    },
    /// The encoded text violates the selected padding policy.
    InvalidPadding,
}

impl fmt::Display for Base64DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidByte { index, byte } => {
                write!(
                    f,
                    "base64 input contains invalid byte 0x{byte:02x} at position {index}"
                )
            }
            Self::InvalidLength { len } => {
                write!(f, "base64 input has invalid byte length {len}")
            }
            Self::InvalidLastSymbol { index, byte } => {
                write!(
                    f,
                    "base64 input contains invalid last symbol 0x{byte:02x} at position {index}"
                )
            }
            Self::InvalidPadding => write!(f, "base64 input violates padding policy"),
        }
    }
}

impl Error for Base64DecodeError {}

impl From<base64::DecodeError> for Base64DecodeError {
    fn from(error: base64::DecodeError) -> Self {
        match error {
            base64::DecodeError::InvalidByte(index, byte) => Self::InvalidByte { index, byte },
            base64::DecodeError::InvalidLength(len) => Self::InvalidLength { len },
            base64::DecodeError::InvalidLastSymbol(index, byte) => {
                Self::InvalidLastSymbol { index, byte }
            }
            base64::DecodeError::InvalidPadding => Self::InvalidPadding,
        }
    }
}

/// Encodes bytes with the standard Base64 alphabet and `=` padding.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::encode_base64;
///
/// assert_eq!(encode_base64(b"fo"), "Zm8=");
/// ```
#[must_use]
pub fn encode_base64(bytes: impl AsRef<[u8]>) -> String {
    STANDARD.encode(bytes)
}

/// Decodes standard Base64 text that uses the standard alphabet and padding.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::decode_base64;
///
/// assert_eq!(decode_base64("Zm8=")?, b"fo");
/// # Ok::<(), bluetape_rs_codec::Base64DecodeError>(())
/// ```
///
/// # Errors
///
/// Returns [`Base64DecodeError`] when input contains invalid bytes, invalid
/// length, invalid trailing bits, or padding that does not match the standard
/// padded policy.
pub fn decode_base64(encoded: impl AsRef<str>) -> Result<Vec<u8>, Base64DecodeError> {
    STANDARD.decode(encoded.as_ref()).map_err(Into::into)
}

/// Encodes bytes with the standard Base64 alphabet and no padding.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::encode_base64_unpadded;
///
/// assert_eq!(encode_base64_unpadded(b"fo"), "Zm8");
/// ```
#[must_use]
pub fn encode_base64_unpadded(bytes: impl AsRef<[u8]>) -> String {
    STANDARD_NO_PAD.encode(bytes)
}

/// Decodes standard Base64 text that must not contain padding.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::decode_base64_unpadded;
///
/// assert_eq!(decode_base64_unpadded("Zm8")?, b"fo");
/// # Ok::<(), bluetape_rs_codec::Base64DecodeError>(())
/// ```
///
/// # Errors
///
/// Returns [`Base64DecodeError`] when input contains invalid bytes, invalid
/// length, invalid trailing bits, or padding in a no-padding variant.
pub fn decode_base64_unpadded(encoded: impl AsRef<str>) -> Result<Vec<u8>, Base64DecodeError> {
    STANDARD_NO_PAD.decode(encoded.as_ref()).map_err(Into::into)
}

/// Encodes bytes with the URL-safe Base64 alphabet and `=` padding.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::encode_base64_url;
///
/// assert_eq!(encode_base64_url([0xfb, 0xff]), "-_8=");
/// ```
#[must_use]
pub fn encode_base64_url(bytes: impl AsRef<[u8]>) -> String {
    URL_SAFE.encode(bytes)
}

/// Decodes URL-safe Base64 text that uses `=` padding.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::decode_base64_url;
///
/// assert_eq!(decode_base64_url("-_8=")?, vec![0xfb, 0xff]);
/// # Ok::<(), bluetape_rs_codec::Base64DecodeError>(())
/// ```
///
/// # Errors
///
/// Returns [`Base64DecodeError`] when input contains invalid bytes, invalid
/// length, invalid trailing bits, or padding that does not match the URL-safe
/// padded policy.
pub fn decode_base64_url(encoded: impl AsRef<str>) -> Result<Vec<u8>, Base64DecodeError> {
    URL_SAFE.decode(encoded.as_ref()).map_err(Into::into)
}

/// Encodes bytes with the URL-safe Base64 alphabet and no padding.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::encode_base64_url_unpadded;
///
/// assert_eq!(encode_base64_url_unpadded([0xfb, 0xff]), "-_8");
/// ```
#[must_use]
pub fn encode_base64_url_unpadded(bytes: impl AsRef<[u8]>) -> String {
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Decodes URL-safe Base64 text that must not contain padding.
///
/// # Examples
///
/// ```
/// use bluetape_rs_codec::decode_base64_url_unpadded;
///
/// assert_eq!(decode_base64_url_unpadded("-_8")?, vec![0xfb, 0xff]);
/// # Ok::<(), bluetape_rs_codec::Base64DecodeError>(())
/// ```
///
/// # Errors
///
/// Returns [`Base64DecodeError`] when input contains invalid bytes, invalid
/// length, invalid trailing bits, or padding in a no-padding variant.
pub fn decode_base64_url_unpadded(encoded: impl AsRef<str>) -> Result<Vec<u8>, Base64DecodeError> {
    URL_SAFE_NO_PAD.decode(encoded.as_ref()).map_err(Into::into)
}
