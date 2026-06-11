use crate::{CompressionConfig, CompressionError};
use std::io::{Read, Write};

pub(crate) fn copy_plain_to_encoder<R, W>(
    algorithm: &'static str,
    reader: &mut R,
    encoder: &mut W,
) -> Result<u64, CompressionError>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; 8192];
    let mut read_total = 0_u64;

    loop {
        let read = reader
            .read(&mut buf)
            .map_err(|source| CompressionError::CompressRead { algorithm, source })?;
        if read == 0 {
            return Ok(read_total);
        }

        encoder
            .write_all(&buf[..read])
            .map_err(|source| CompressionError::CompressWrite { algorithm, source })?;
        read_total = read_total.saturating_add(read as u64);
    }
}

pub(crate) fn copy_decompressed<R, W>(
    algorithm: &'static str,
    reader: &mut R,
    writer: &mut W,
    config: CompressionConfig,
) -> Result<u64, CompressionError>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; 8192];
    let mut written = 0_u64;

    loop {
        let read = reader
            .read(&mut buf)
            .map_err(|source| CompressionError::DecompressRead { algorithm, source })?;
        if read == 0 {
            return Ok(written);
        }

        let next = written.saturating_add(read as u64);
        if let Some(limit) = config.max_decompressed_size {
            if next > limit as u64 {
                return Err(CompressionError::DecompressedSizeLimitExceeded {
                    algorithm,
                    limit,
                    actual: usize::try_from(next).unwrap_or(usize::MAX),
                });
            }
        }

        writer
            .write_all(&buf[..read])
            .map_err(|source| CompressionError::DecompressWrite { algorithm, source })?;
        written = next;
    }
}

#[cfg(any(feature = "lz4", feature = "snappy"))]
pub(crate) fn enforce_size_limit(
    algorithm: &'static str,
    actual: usize,
    config: CompressionConfig,
) -> Result<(), CompressionError> {
    if let Some(limit) = config.max_decompressed_size {
        if actual > limit {
            return Err(CompressionError::DecompressedSizeLimitExceeded {
                algorithm,
                limit,
                actual,
            });
        }
    }
    Ok(())
}
