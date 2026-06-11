use crate::{CompressionConfig, CompressionError};
#[cfg(any(
    feature = "gzip",
    feature = "zlib",
    feature = "deflate",
    feature = "zstd",
    feature = "lz4",
    feature = "snappy"
))]
use crate::{Compressor, adapters};

/// A registry entry for the algorithms compiled into this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompressionAlgorithm {
    /// gzip stream format.
    #[cfg(feature = "gzip")]
    Gzip,
    /// zlib stream format.
    #[cfg(feature = "zlib")]
    Zlib,
    /// raw deflate stream format.
    #[cfg(feature = "deflate")]
    Deflate,
    /// zstd stream format.
    #[cfg(feature = "zstd")]
    Zstd,
    /// lz4 block format with prepended size.
    #[cfg(feature = "lz4")]
    Lz4,
    /// snappy raw block format.
    #[cfg(feature = "snappy")]
    Snappy,
}

impl CompressionAlgorithm {
    /// Stable algorithm name.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip => "gzip",
            #[cfg(feature = "zlib")]
            Self::Zlib => "zlib",
            #[cfg(feature = "deflate")]
            Self::Deflate => "deflate",
            #[cfg(feature = "zstd")]
            Self::Zstd => "zstd",
            #[cfg(feature = "lz4")]
            Self::Lz4 => "lz4",
            #[cfg(feature = "snappy")]
            Self::Snappy => "snappy",
        }
    }

    /// Compress bytes with this registry entry.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected encoder fails.
    pub fn compress(self, plain: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.compress_with_config(plain, CompressionConfig::default())
    }

    /// Compress bytes with this registry entry and explicit configuration.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected encoder fails.
    pub fn compress_with_config(
        self,
        _plain: &[u8],
        _config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip => adapters::Gzip.compress_with_config(_plain, _config),
            #[cfg(feature = "zlib")]
            Self::Zlib => adapters::Zlib.compress_with_config(_plain, _config),
            #[cfg(feature = "deflate")]
            Self::Deflate => adapters::Deflate.compress_with_config(_plain, _config),
            #[cfg(feature = "zstd")]
            Self::Zstd => adapters::Zstd.compress_with_config(_plain, _config),
            #[cfg(feature = "lz4")]
            Self::Lz4 => adapters::Lz4.compress_with_config(_plain, _config),
            #[cfg(feature = "snappy")]
            Self::Snappy => adapters::Snappy.compress_with_config(_plain, _config),
        }
    }

    /// Decompress bytes with this registry entry.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected decoder fails.
    pub fn decompress(self, _compressed: &[u8]) -> Result<Vec<u8>, CompressionError> {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip => adapters::Gzip.decompress(_compressed),
            #[cfg(feature = "zlib")]
            Self::Zlib => adapters::Zlib.decompress(_compressed),
            #[cfg(feature = "deflate")]
            Self::Deflate => adapters::Deflate.decompress(_compressed),
            #[cfg(feature = "zstd")]
            Self::Zstd => adapters::Zstd.decompress(_compressed),
            #[cfg(feature = "lz4")]
            Self::Lz4 => adapters::Lz4.decompress(_compressed),
            #[cfg(feature = "snappy")]
            Self::Snappy => adapters::Snappy.decompress(_compressed),
        }
    }
}

/// Algorithms compiled into this crate, in registry order.
#[must_use]
pub fn algorithms() -> &'static [CompressionAlgorithm] {
    &[
        #[cfg(feature = "gzip")]
        CompressionAlgorithm::Gzip,
        #[cfg(feature = "zlib")]
        CompressionAlgorithm::Zlib,
        #[cfg(feature = "deflate")]
        CompressionAlgorithm::Deflate,
        #[cfg(feature = "zstd")]
        CompressionAlgorithm::Zstd,
        #[cfg(feature = "lz4")]
        CompressionAlgorithm::Lz4,
        #[cfg(feature = "snappy")]
        CompressionAlgorithm::Snappy,
    ]
}

/// Default algorithm for general-purpose compression when available.
#[must_use]
#[cfg(feature = "zstd")]
pub fn default_algorithm() -> CompressionAlgorithm {
    CompressionAlgorithm::Zstd
}

/// Default algorithm when zstd is not enabled.
#[must_use]
#[cfg(all(not(feature = "zstd"), feature = "gzip"))]
pub fn default_algorithm() -> CompressionAlgorithm {
    CompressionAlgorithm::Gzip
}

/// Default algorithm when zstd and gzip are not enabled.
#[must_use]
#[cfg(all(not(any(feature = "zstd", feature = "gzip")), feature = "zlib"))]
pub fn default_algorithm() -> CompressionAlgorithm {
    CompressionAlgorithm::Zlib
}

/// Default algorithm when zstd, gzip, and zlib are not enabled.
#[must_use]
#[cfg(all(
    not(any(feature = "zstd", feature = "gzip", feature = "zlib")),
    feature = "deflate"
))]
pub fn default_algorithm() -> CompressionAlgorithm {
    CompressionAlgorithm::Deflate
}

/// Default algorithm when only block-oriented faster codecs are available.
#[must_use]
#[cfg(all(
    not(any(
        feature = "zstd",
        feature = "gzip",
        feature = "zlib",
        feature = "deflate"
    )),
    feature = "lz4"
))]
pub fn default_algorithm() -> CompressionAlgorithm {
    CompressionAlgorithm::Lz4
}

/// Default algorithm when snappy is the only available algorithm family.
#[must_use]
#[cfg(all(
    not(any(
        feature = "zstd",
        feature = "gzip",
        feature = "zlib",
        feature = "deflate",
        feature = "lz4"
    )),
    feature = "snappy"
))]
pub fn default_algorithm() -> CompressionAlgorithm {
    CompressionAlgorithm::Snappy
}
