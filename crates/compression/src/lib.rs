//! Opt-in compression helpers for bluetape-rs.
//!
//! This crate keeps compression separate from the core codec crate so algorithm
//! dependencies remain explicit. Enable the algorithms you need through additive
//! features, or use `all` when comparing every adapter.

mod adapters;
mod config;
mod error;
mod registry;
mod stream;
mod traits;

pub use config::{CompressionConfig, CompressionLevel, DEFAULT_MAX_DECOMPRESSED_SIZE};
pub use error::CompressionError;
pub use registry::{CompressionAlgorithm, algorithms};
pub use stream::{CompressionWriter, DecompressionReader};
pub use traits::Compressor;

#[cfg(any(
    feature = "gzip",
    feature = "zlib",
    feature = "deflate",
    feature = "zstd",
    feature = "lz4",
    feature = "snappy"
))]
pub use adapters::*;

#[cfg(any(
    feature = "gzip",
    feature = "zlib",
    feature = "deflate",
    feature = "zstd",
    feature = "lz4",
    feature = "snappy"
))]
pub use registry::default_algorithm;
