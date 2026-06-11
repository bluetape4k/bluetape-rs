use crate::{CompressionConfig, CompressionError, CompressionLevel, Compressor};

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
        zstd::stream::decode_all(compressed).map_err(|source| CompressionError::Decompress {
            algorithm: self.name(),
            source,
        })
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
