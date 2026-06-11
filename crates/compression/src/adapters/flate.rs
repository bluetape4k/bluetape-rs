use crate::{CompressionError, CompressionLevel};

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
) -> Result<Vec<u8>, CompressionError> {
    let mut out = Vec::new();
    reader
        .read_to_end(&mut out)
        .map_err(|source| CompressionError::Decompress { algorithm, source })?;
    Ok(out)
}
