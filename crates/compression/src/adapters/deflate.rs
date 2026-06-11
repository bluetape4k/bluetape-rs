use crate::adapters::common::{copy_decompressed, copy_plain_to_encoder};
use crate::adapters::flate::{flate_level, read_all};
use crate::{
    CompressionConfig, CompressionError, CompressionWriter, Compressor, DecompressionReader,
};
use std::io::{Read, Write};

/// raw deflate compressor.
#[derive(Debug, Clone, Copy, Default)]
pub struct Deflate;

impl Compressor for Deflate {
    fn name(&self) -> &'static str {
        "deflate"
    }

    fn compress_with_config(
        &self,
        plain: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        use flate2::write::DeflateEncoder;
        use std::io::Write;

        let mut encoder = DeflateEncoder::new(Vec::new(), flate_level(config.level));
        encoder
            .write_all(plain)
            .map_err(|source| CompressionError::Compress {
                algorithm: self.name(),
                source,
            })?;
        encoder
            .finish()
            .map_err(|source| CompressionError::Compress {
                algorithm: self.name(),
                source,
            })
    }

    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.decompress_with_config(compressed, CompressionConfig::default())
    }

    fn decompress_with_config(
        &self,
        compressed: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        use flate2::read::DeflateDecoder;

        read_all(self.name(), DeflateDecoder::new(compressed), config)
    }

    fn compression_writer<W>(
        &self,
        writer: W,
        config: CompressionConfig,
    ) -> Result<CompressionWriter<W>, CompressionError>
    where
        W: Write,
    {
        Ok(CompressionWriter::deflate(
            flate2::write::DeflateEncoder::new(writer, flate_level(config.level)),
        ))
    }

    fn decompression_reader<R>(
        &self,
        reader: R,
        config: CompressionConfig,
    ) -> Result<DecompressionReader<R>, CompressionError>
    where
        R: Read,
    {
        Ok(DecompressionReader::deflate(
            crate::stream::LimitedReader::new(
                self.name(),
                flate2::read::DeflateDecoder::new(reader),
                config,
            ),
        ))
    }

    fn compress_stream<R, W>(
        &self,
        reader: R,
        mut writer: W,
        config: CompressionConfig,
    ) -> Result<u64, CompressionError>
    where
        R: Read,
        W: Write,
    {
        let mut reader = reader;
        let mut encoder = self.compression_writer(&mut writer, config)?;
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
        use flate2::read::DeflateDecoder;

        let mut decoder = DeflateDecoder::new(reader);
        copy_decompressed(self.name(), &mut decoder, &mut writer, config)
    }
}
