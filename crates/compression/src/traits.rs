use crate::{CompressionConfig, CompressionError, CompressionWriter, DecompressionReader};
use std::io::{Read, Write};

/// A compressor with one-shot byte helpers and optional streaming helpers.
pub trait Compressor: Copy + Send + Sync + 'static {
    /// Stable algorithm name used in reports and registry lookup.
    fn name(&self) -> &'static str;

    /// Compress bytes using this adapter's default configuration.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the underlying encoder rejects or fails
    /// to finish the payload.
    fn compress(&self, plain: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.compress_with_config(plain, CompressionConfig::default())
    }

    /// Compress bytes using an explicit configuration.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the underlying encoder rejects or fails
    /// to finish the payload.
    fn compress_with_config(
        &self,
        plain: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError>;

    /// Decompress bytes.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the compressed payload is invalid for
    /// this algorithm or the underlying decoder fails.
    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressionError>;

    /// Decompress bytes using an explicit configuration.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the compressed payload is invalid for
    /// this algorithm, the underlying decoder fails, or decompressed output
    /// exceeds the configured safety limit.
    fn decompress_with_config(
        &self,
        compressed: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        let decompressed = self.decompress(compressed)?;
        if let Some(limit) = config.max_decompressed_size
            && decompressed.len() > limit
        {
            return Err(CompressionError::DecompressedSizeLimitExceeded {
                algorithm: self.name(),
                limit,
                actual: decompressed.len(),
            });
        }
        Ok(decompressed)
    }

    /// Constructs a streaming compression writer.
    ///
    /// Call [`CompressionWriter::finish`] after the final write to flush codec
    /// trailers and recover the wrapped writer.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected adapter rejects the
    /// configuration or cannot construct its writer.
    fn compression_writer<W>(
        &self,
        _writer: W,
        _config: CompressionConfig,
    ) -> Result<CompressionWriter<W>, CompressionError>
    where
        W: Write,
    {
        Err(CompressionError::UnsupportedOperation {
            algorithm: self.name(),
            operation: "compression_writer",
            reason: "direct streaming writer construction is not implemented by this compressor",
        })
    }

    /// Constructs a streaming decompression reader.
    ///
    /// The returned reader enforces [`CompressionConfig`] decompressed-size
    /// limits while bytes are read.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the selected adapter cannot construct
    /// its reader.
    fn decompression_reader<R>(
        &self,
        _reader: R,
        _config: CompressionConfig,
    ) -> Result<DecompressionReader<R>, CompressionError>
    where
        R: Read,
    {
        Err(CompressionError::UnsupportedOperation {
            algorithm: self.name(),
            operation: "decompression_reader",
            reason: "direct streaming reader construction is not implemented by this compressor",
        })
    }

    /// Compress bytes from `reader` into `writer`.
    ///
    /// The returned `u64` is the number of plain bytes read from `reader`.
    /// Implementations finish/flush their encoder before returning. For lz4
    /// and snappy this method uses the framed stream format; their one-shot
    /// byte helpers intentionally keep the existing block/raw payload format.
    /// The default implementation preserves source compatibility for custom
    /// implementors by buffering the full input and calling
    /// [`Compressor::compress_with_config`]; production streaming adapters
    /// should override it.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the underlying reader, writer, or
    /// encoder fails.
    fn compress_stream<R, W>(
        &self,
        mut reader: R,
        mut writer: W,
        config: CompressionConfig,
    ) -> Result<u64, CompressionError>
    where
        R: Read,
        W: Write,
    {
        let mut plain = Vec::new();
        reader
            .read_to_end(&mut plain)
            .map_err(|source| CompressionError::CompressRead {
                algorithm: self.name(),
                source,
            })?;

        let compressed = self.compress_with_config(&plain, config)?;
        writer
            .write_all(&compressed)
            .map_err(|source| CompressionError::CompressWrite {
                algorithm: self.name(),
                source,
            })?;
        writer
            .flush()
            .map_err(|source| CompressionError::CompressFinish {
                algorithm: self.name(),
                source,
            })?;
        Ok(plain.len() as u64)
    }

    /// Decompress bytes from `reader` into `writer`.
    ///
    /// The returned `u64` is the number of decompressed bytes written to
    /// `writer`. For lz4 and snappy this method expects the framed stream
    /// format emitted by [`Compressor::compress_stream`].
    /// The default implementation preserves source compatibility for custom
    /// implementors by buffering the full compressed input and calling
    /// [`Compressor::decompress_with_config`]; production streaming adapters
    /// should override it.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the underlying reader, writer, decoder
    /// fails, or decompressed output exceeds the configured safety limit.
    fn decompress_stream<R, W>(
        &self,
        mut reader: R,
        mut writer: W,
        config: CompressionConfig,
    ) -> Result<u64, CompressionError>
    where
        R: Read,
        W: Write,
    {
        let mut compressed = Vec::new();
        reader
            .read_to_end(&mut compressed)
            .map_err(|source| CompressionError::DecompressRead {
                algorithm: self.name(),
                source,
            })?;

        let decompressed = self.decompress_with_config(&compressed, config)?;
        writer
            .write_all(&decompressed)
            .map_err(|source| CompressionError::DecompressWrite {
                algorithm: self.name(),
                source,
            })?;
        writer
            .flush()
            .map_err(|source| CompressionError::DecompressWrite {
                algorithm: self.name(),
                source,
            })?;
        Ok(decompressed.len() as u64)
    }
}
