/// Default decompressed-size safety limit for decode helpers.
pub const DEFAULT_MAX_DECOMPRESSED_SIZE: usize = 64 * 1024 * 1024;

/// Compression level requested by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionLevel {
    /// Library default for the selected algorithm.
    #[default]
    Default,
    /// Prefer speed over ratio.
    Fast,
    /// Prefer ratio over speed.
    Best,
    /// Algorithm-specific custom level.
    Custom(u32),
}

/// Compression and decompression configuration.
///
/// `CompressionConfig::default()` applies a 64 MiB decompressed-size safety
/// limit. Use [`CompressionConfig::without_decompressed_size_limit`] only when
/// another trusted layer bounds decoded output.
///
/// # Examples
///
/// ```
/// use bluetape_rs_compression::{CompressionConfig, DEFAULT_MAX_DECOMPRESSED_SIZE};
///
/// let default = CompressionConfig::new();
/// assert_eq!(
///     default.max_decompressed_size,
///     Some(DEFAULT_MAX_DECOMPRESSED_SIZE)
/// );
///
/// let trusted_input = default.without_decompressed_size_limit();
/// assert_eq!(trusted_input.max_decompressed_size, None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct CompressionConfig {
    /// Requested compression level.
    pub level: CompressionLevel,
    /// Maximum decompressed bytes accepted by decode helpers.
    pub max_decompressed_size: Option<usize>,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            level: CompressionLevel::Default,
            max_decompressed_size: Some(DEFAULT_MAX_DECOMPRESSED_SIZE),
        }
    }
}

impl CompressionConfig {
    /// Creates a configuration with library-default compression level.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a copy of this configuration with the requested compression level.
    #[must_use]
    pub fn with_level(mut self, level: CompressionLevel) -> Self {
        self.level = level;
        self
    }

    /// Returns a copy of this configuration with a decompressed-size safety
    /// limit.
    #[must_use]
    pub fn with_max_decompressed_size(mut self, max_decompressed_size: usize) -> Self {
        self.max_decompressed_size = Some(max_decompressed_size);
        self
    }

    /// Returns a copy of this configuration without a decompressed-size safety
    /// limit.
    ///
    /// This is intended for trusted inputs whose decompressed size is bounded by
    /// another layer.
    #[must_use]
    pub fn without_decompressed_size_limit(mut self) -> Self {
        self.max_decompressed_size = None;
        self
    }
}
