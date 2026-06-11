use crate::{CompressionConfig, CompressionError};

/// A one-shot byte compressor.
pub trait Compressor: Copy + Send + Sync + 'static {
    /// Stable algorithm name used in reports and registry lookup.
    fn name(&self) -> &'static str;

    /// Compress bytes using this adapter's default configuration.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the underlying encoder rejects or fails
    /// to finish the payload.
    fn compress(&self, plain: &[u8]) -> Result<Vec<u8>, CompressionError> {
        self.compress_with_config(plain, CompressionConfig::default())
    }

    /// Compress bytes using an explicit configuration.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the underlying encoder rejects or fails
    /// to finish the payload.
    fn compress_with_config(
        &self,
        plain: &[u8],
        config: CompressionConfig,
    ) -> Result<Vec<u8>, CompressionError>;

    /// Decompress bytes.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the compressed payload is invalid for
    /// this algorithm or the underlying decoder fails.
    fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressionError>;
}
