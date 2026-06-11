use crate::{CompressionConfig, CompressionError, CompressionLevel, Compressor};

/// snappy raw block compressor.
#[derive(Debug, Clone, Copy, Default)]
pub struct Snappy;

impl Compressor for Snappy {
    fn name(&self) -> &'static str {
        "snappy"
    }

    fn compress_with_config(
        &self,
        plain: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError> {
        reject_non_default_level(self.name(), config.level)?;
        snap::raw::Encoder::new()
            .compress_vec(plain)
            .map_err(|source| CompressionError::Compress {
                algorithm: self.name(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, source),
            })
    }

    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressionError> {
        snap::raw::Decoder::new()
            .decompress_vec(compressed)
            .map_err(|source| CompressionError::Decompress {
                algorithm: self.name(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, source),
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
            reason: "snappy raw block compression does not expose configurable levels",
        }),
        CompressionLevel::Best => Err(CompressionError::UnsupportedLevel {
            algorithm,
            level: 9,
            reason: "snappy raw block compression does not expose configurable levels",
        }),
        CompressionLevel::Custom(level) => Err(CompressionError::UnsupportedLevel {
            algorithm,
            level,
            reason: "snappy raw block compression does not expose configurable levels",
        }),
    }
}
