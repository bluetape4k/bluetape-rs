use crate::adapters::flate::{flate_level, read_all};
use crate::{CompressionConfig, CompressionError, Compressor};

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
        use flate2::read::DeflateDecoder;

        read_all(self.name(), DeflateDecoder::new(compressed))
    }
}
