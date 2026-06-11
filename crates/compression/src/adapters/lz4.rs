use crate::adapters::common::{copy_decompressed, copy_plain_to_encoder, enforce_size_limit};
use crate::{
    CompressionConfig, CompressionError, CompressionLevel, CompressionWriter, Compressor,
    DecompressionReader,
};
use std::io::{Read, Write};

/// lz4 compressor.
///
/// One-shot byte helpers use lz4 block payloads with prepended uncompressed
/// size. Streaming helpers use lz4 framed payloads; the two payload formats are
/// intentionally not interchangeable.
#[derive(Debug, Clone, Copy, Default)]
pub struct Lz4;

impl Compressor for Lz4 {
    fn name(&self) -> &'static str {
        "lz4"
    }

    fn compress_with_config(
        &self,
        plain: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        reject_non_default_level(self.name(), config.level)?;
        Ok(lz4_flex::block::compress_prepend_size(plain))
    }

    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.decompress_with_config(compressed, CompressionConfig::default())
    }

    fn decompress_with_config(
        &self,
        compressed: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        let (declared_size, _) =
            lz4_flex::block::uncompressed_size(compressed).map_err(|source| {
                CompressionError::Decompress {
                    algorithm: self.name(),
                    source: std::io::Error::new(std::io::ErrorKind::InvalidData, source),
                }
            })?;
        enforce_size_limit(self.name(), declared_size, config)?;
        lz4_flex::block::decompress_size_prepended(compressed).map_err(|source| {
            CompressionError::Decompress {
                algorithm: self.name(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, source),
            }
        })
    }

    fn compression_writer<W>(
        &self,
        writer: W,
        config: CompressionConfig,
    ) -> Result<CompressionWriter<W>, CompressionError>
    where
        W: Write,
    {
        reject_non_default_level(self.name(), config.level)?;
        Ok(CompressionWriter::lz4(lz4_flex::frame::FrameEncoder::new(
            writer,
        )))
    }

    fn decompression_reader<R>(
        &self,
        reader: R,
        config: CompressionConfig,
    ) -> Result<DecompressionReader<R>, CompressionError>
    where
        R: Read,
    {
        Ok(DecompressionReader::lz4(crate::stream::LimitedReader::new(
            self.name(),
            lz4_flex::frame::FrameDecoder::new(reader),
            config,
        )))
    }

    fn compress_stream<R, W>(
        &self,
        mut reader: R,
        writer: W,
        config: CompressionConfig,
    ) -> Result<u64, CompressionError>
    where
        R: Read,
        W: Write,
    {
        let mut encoder = self.compression_writer(writer, config)?;
        let copied = copy_plain_to_encoder(self.name(), &mut reader, &mut encoder)?;
        encoder
            .finish()?
            .flush()
            .map_err(|source| CompressionError::CompressFinish {
                algorithm: self.name(),
                source,
            })?;
        Ok(copied)
    }

    fn decompress_stream<R, W>(
        &self,
        reader: R,
        mut writer: W,
        config: CompressionConfig,
    ) -> Result<u64, CompressionError>
    where
        R: Read,
        W: Write,
    {
        let mut decoder = lz4_flex::frame::FrameDecoder::new(reader);
        copy_decompressed(self.name(), &mut decoder, &mut writer, config)
    }
}

fn reject_non_default_level(
    algorithm: &'static str,
    level: CompressionLevel,
) -> Result<(), CompressionError> {
    match level {
        CompressionLevel::Default => Ok(()),
        CompressionLevel::Fast => Err(CompressionError::UnsupportedLevel {
            algorithm,
            level: 1,
            reason: "lz4 block compression does not expose configurable levels",
        }),
        CompressionLevel::Best => Err(CompressionError::UnsupportedLevel {
            algorithm,
            level: 9,
            reason: "lz4 block compression does not expose configurable levels",
        }),
        CompressionLevel::Custom(level) => Err(CompressionError::UnsupportedLevel {
            algorithm,
            level,
            reason: "lz4 block compression does not expose configurable levels",
        }),
    }
}
