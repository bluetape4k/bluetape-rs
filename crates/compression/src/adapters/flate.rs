use crate::adapters::common::copy_decompressed;
use crate::{CompressionConfig, CompressionError, CompressionLevel};

pub(crate) fn flate_level(level: CompressionLevel) -> flate2::Compression {
    match level {
        CompressionLevel::Default => flate2::Compression::default(),
        CompressionLevel::Fast => flate2::Compression::fast(),
        CompressionLevel::Best => flate2::Compression::best(),
        CompressionLevel::Custom(level) => flate2::Compression::new(level),
    }
}

pub(crate) fn read_all(
    algorithm: &'static str,
    mut reader: impl std::io::Read,
    config: CompressionConfig,
) -> Result<Vec<u8>, CompressionError> {
    let mut out = Vec::new();
    copy_decompressed(algorithm, &mut reader, &mut out, config)?;
    Ok(out)
}
