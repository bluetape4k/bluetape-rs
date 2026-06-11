use crate::adapters::common::{copy_decompressed, copy_plain_to_encoder};
use crate::{
    CompressionConfig, CompressionError, CompressionLevel, CompressionWriter, Compressor,
    DecompressionReader,
};
use std::io::{Read, Write};

/// zstd compressor.
#[derive(Debug, Clone, Copy, Default)]
pub struct Zstd;

impl Compressor for Zstd {
    fn name(&self) -> &'static str {
        "zstd"
    }

    fn compress_with_config(
        &self,
        plain: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        zstd::bulk::compress(plain, zstd_level(self.name(), config.level)?).map_err(|source| {
            CompressionError::Compress {
                algorithm: self.name(),
                source,
            }
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
        let mut out = Vec::new();
        self.decompress_stream(compressed, &mut out, config)?;
        Ok(out)
    }

    fn compression_writer<W>(
        &self,
        writer: W,
        config: CompressionConfig,
    ) -> Result<CompressionWriter<W>, CompressionError>
    where
        W: Write,
    {
        zstd::stream::write::Encoder::new(writer, zstd_level(self.name(), config.level)?)
            .map(CompressionWriter::zstd)
            .map_err(|source| CompressionError::CompressWrite {
                algorithm: self.name(),
                source,
            })
    }

    fn decompression_reader<R>(
        &self,
        reader: R,
        config: CompressionConfig,
    ) -> Result<DecompressionReader<R>, CompressionError>
    where
        R: Read,
    {
        zstd::stream::read::Decoder::new(reader)
            .map(|reader| {
                DecompressionReader::zstd(crate::stream::LimitedReader::new(
                    self.name(),
                    reader,
                    config,
                ))
            })
            .map_err(|source| CompressionError::DecompressInit {
                algorithm: self.name(),
                source,
            })
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
        let mut decoder = zstd::stream::read::Decoder::new(reader).map_err(|source| {
            CompressionError::DecompressInit {
                algorithm: self.name(),
                source,
            }
        })?;
        copy_decompressed(self.name(), &mut decoder, &mut writer, config)
    }
}

fn zstd_level(algorithm: &'static str, level: CompressionLevel) -> Result<i32, CompressionError> {
    match level {
        CompressionLevel::Default => Ok(0),
        CompressionLevel::Fast => Ok(1),
        CompressionLevel::Best => Ok(21),
        CompressionLevel::Custom(level) => {
            i32::try_from(level).map_err(|_| CompressionError::UnsupportedLevel {
                algorithm,
                level,
                reason: "zstd levels must fit in i32",
            })
        }
    }
}
