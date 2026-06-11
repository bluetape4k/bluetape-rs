use crate::{CompressionConfig, CompressionError, CompressionWriter, DecompressionReader};
#[cfg(any(
    feature = "gzip",
    feature = "zlib",
    feature = "deflate",
    feature = "zstd",
    feature = "lz4",
    feature = "snappy"
))]
use crate::{Compressor, adapters};
use std::io::{Read, Write};

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
    /// lz4 one-shot block format with prepended size.
    ///
    /// Stream helpers for this variant use the lz4 framed format. Do not mix
    /// one-shot block payloads with framed stream payloads.
    #[cfg(feature = "lz4")]
    Lz4,
    /// snappy one-shot raw block format.
    ///
    /// Stream helpers for this variant use the snappy framed format. Do not
    /// mix one-shot raw payloads with framed stream payloads.
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
    pub fn decompress(self, compressed: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.decompress_with_config(compressed, CompressionConfig::default())
    }

    /// Decompress bytes with this registry entry and explicit configuration.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected decoder fails or output
    /// exceeds the configured decompressed-size limit.
    pub fn decompress_with_config(
        self,
        _compressed: &[u8],
        _config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip => adapters::Gzip.decompress_with_config(_compressed, _config),
            #[cfg(feature = "zlib")]
            Self::Zlib => adapters::Zlib.decompress_with_config(_compressed, _config),
            #[cfg(feature = "deflate")]
            Self::Deflate => adapters::Deflate.decompress_with_config(_compressed, _config),
            #[cfg(feature = "zstd")]
            Self::Zstd => adapters::Zstd.decompress_with_config(_compressed, _config),
            #[cfg(feature = "lz4")]
            Self::Lz4 => adapters::Lz4.decompress_with_config(_compressed, _config),
            #[cfg(feature = "snappy")]
            Self::Snappy => adapters::Snappy.decompress_with_config(_compressed, _config),
        }
    }

    /// Constructs a streaming compression writer for this registry entry.
    ///
    /// Call [`CompressionWriter::finish`] after the final write.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected adapter rejects the
    /// configuration or cannot construct its writer.
    pub fn compression_writer<W>(
        self,
        _writer: W,
        _config: CompressionConfig,
    ) -> Result<CompressionWriter<W>, CompressionError>
    where
        W: Write,
    {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip => adapters::Gzip.compression_writer(_writer, _config),
            #[cfg(feature = "zlib")]
            Self::Zlib => adapters::Zlib.compression_writer(_writer, _config),
            #[cfg(feature = "deflate")]
            Self::Deflate => adapters::Deflate.compression_writer(_writer, _config),
            #[cfg(feature = "zstd")]
            Self::Zstd => adapters::Zstd.compression_writer(_writer, _config),
            #[cfg(feature = "lz4")]
            Self::Lz4 => adapters::Lz4.compression_writer(_writer, _config),
            #[cfg(feature = "snappy")]
            Self::Snappy => adapters::Snappy.compression_writer(_writer, _config),
        }
    }

    /// Constructs a streaming decompression reader for this registry entry.
    ///
    /// The returned reader enforces [`CompressionConfig`] decompressed-size
    /// limits while bytes are read. `lz4` and `snappy` use framed stream
    /// formats here; their one-shot helpers use block/raw payload formats.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected adapter cannot construct
    /// its reader.
    pub fn decompression_reader<R>(
        self,
        _reader: R,
        _config: CompressionConfig,
    ) -> Result<DecompressionReader<R>, CompressionError>
    where
        R: Read,
    {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip => adapters::Gzip.decompression_reader(_reader, _config),
            #[cfg(feature = "zlib")]
            Self::Zlib => adapters::Zlib.decompression_reader(_reader, _config),
            #[cfg(feature = "deflate")]
            Self::Deflate => adapters::Deflate.decompression_reader(_reader, _config),
            #[cfg(feature = "zstd")]
            Self::Zstd => adapters::Zstd.decompression_reader(_reader, _config),
            #[cfg(feature = "lz4")]
            Self::Lz4 => adapters::Lz4.decompression_reader(_reader, _config),
            #[cfg(feature = "snappy")]
            Self::Snappy => adapters::Snappy.decompression_reader(_reader, _config),
        }
    }

    /// Compress bytes from `reader` into `writer` with this registry entry.
    ///
    /// For `lz4` and `snappy`, this method emits framed stream payloads. Their
    /// one-shot helpers use block/raw payload formats, so the two payload
    /// families must not be mixed.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected streaming encoder fails.
    pub fn compress_stream<R, W>(
        self,
        _reader: R,
        _writer: W,
        _config: CompressionConfig,
    ) -> Result<u64, CompressionError>
    where
        R: Read,
        W: Write,
    {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip => adapters::Gzip.compress_stream(_reader, _writer, _config),
            #[cfg(feature = "zlib")]
            Self::Zlib => adapters::Zlib.compress_stream(_reader, _writer, _config),
            #[cfg(feature = "deflate")]
            Self::Deflate => adapters::Deflate.compress_stream(_reader, _writer, _config),
            #[cfg(feature = "zstd")]
            Self::Zstd => adapters::Zstd.compress_stream(_reader, _writer, _config),
            #[cfg(feature = "lz4")]
            Self::Lz4 => adapters::Lz4.compress_stream(_reader, _writer, _config),
            #[cfg(feature = "snappy")]
            Self::Snappy => adapters::Snappy.compress_stream(_reader, _writer, _config),
        }
    }

    /// Decompress bytes from `reader` into `writer` with this registry entry.
    ///
    /// For `lz4` and `snappy`, this method expects framed stream payloads from
    /// [`CompressionAlgorithm::compress_stream`]. Their one-shot helpers use
    /// block/raw payload formats, so the two payload families must not be
    /// mixed.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected streaming decoder fails
    /// or output exceeds the configured decompressed-size limit.
    pub fn decompress_stream<R, W>(
        self,
        _reader: R,
        _writer: W,
        _config: CompressionConfig,
    ) -> Result<u64, CompressionError>
    where
        R: Read,
        W: Write,
    {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip => adapters::Gzip.decompress_stream(_reader, _writer, _config),
            #[cfg(feature = "zlib")]
            Self::Zlib => adapters::Zlib.decompress_stream(_reader, _writer, _config),
            #[cfg(feature = "deflate")]
            Self::Deflate => adapters::Deflate.decompress_stream(_reader, _writer, _config),
            #[cfg(feature = "zstd")]
            Self::Zstd => adapters::Zstd.decompress_stream(_reader, _writer, _config),
            #[cfg(feature = "lz4")]
            Self::Lz4 => adapters::Lz4.decompress_stream(_reader, _writer, _config),
            #[cfg(feature = "snappy")]
            Self::Snappy => adapters::Snappy.decompress_stream(_reader, _writer, _config),
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
