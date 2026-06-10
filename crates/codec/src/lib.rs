//! Codec and encoding helpers for bluetape-rs.
//!
//! This crate is the `0.3.0` codec boundary. It starts with workspace and
//! facade wiring only so the first codec APIs can land in focused follow-up
//! issues without mixing bootstrap work with encoder behavior.
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

#[cfg(test)]
mod tests {
    #[test]
    fn crate_metadata_matches_codec_boundary() {
        assert_eq!(env!("CARGO_PKG_NAME"), "bluetape-rs-codec");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.3.0");
    }
}
