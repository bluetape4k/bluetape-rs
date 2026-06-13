//! Rust-native serialization boundary for bluetape-rs.
//!
//! This bootstrap crate reserves the focused `0.5.0` serialization package and
//! keeps the root facade opt-in. Serializer traits, payload envelopes, and
//! adapters are added in later reviewed issues.
//!
//! ```text
//! // Enable the root facade when a single dependency is more convenient:
//! // bluetape-rs = { version = "...", features = ["serialization"] }
//! ```

#[cfg(test)]
mod tests {
    #[test]
    fn crate_metadata_matches_serialization_boundary() {
        assert_eq!(env!("CARGO_PKG_NAME"), "bluetape-rs-serialization");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.4.0");
    }
}
