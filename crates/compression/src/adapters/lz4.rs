use crate::{CompressionConfig, CompressionError, CompressionLevel, Compressor};

/// lz4 compressor with prepended uncompressed size.
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
        Ok(lz4_flex::compress_prepend_size(plain))
    }

    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressionError> {
        lz4_flex::decompress_size_prepended(compressed).map_err(|source| {
            CompressionError::Decompress {
                algorithm: self.name(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, source),
            }
        })
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
