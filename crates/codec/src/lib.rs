//! Codec and encoding helpers for bluetape-rs.
//!
//! This crate is the `0.3.0` codec boundary. It starts with strict hexadecimal
//! encoding and decoding plus focused Base64 helpers, then grows into
//! additional base-family helpers in follow-up issues.
//!
//! Planned APIs stay small and Rust-native:
//!
//! - strict hex encoding and decoding
//! - Base64 standard and URL-safe variants
//! - typed errors for caller-owned invalid encoded input
//! - small binary/text helpers only when they make codec call sites clearer
//!
//! Compression belongs to `0.4.0`, and serde-oriented serialization belongs to
//! `0.5.0`. This crate does not provide those APIs.
//!
//! ```text
//! // Enable the root facade when a single dependency is more convenient:
//! // bluetape-rs = { version = "0.1.1", features = ["codec"] }
//! ```

mod base64;
mod hex;

pub use base64::{
    Base64DecodeError, decode_base64, decode_base64_unpadded, decode_base64_url,
    decode_base64_url_unpadded, encode_base64, encode_base64_unpadded, encode_base64_url,
    encode_base64_url_unpadded,
};
pub use hex::{HexDecodeError, decode_hex, encode_hex_lower, encode_hex_upper};

#[cfg(test)]
mod tests {
    #[test]
    fn crate_metadata_matches_codec_boundary() {
        assert_eq!(env!("CARGO_PKG_NAME"), "bluetape-rs-codec");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.3.0");
    }
}
