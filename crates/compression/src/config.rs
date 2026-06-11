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

/// One-shot compression configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct CompressionConfig {
    /// Requested compression level.
    pub level: CompressionLevel,
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
}
