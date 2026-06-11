use thiserror::Error;

/// Errors returned by compression adapters.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CompressionError {
    /// The selected adapter does not support the requested compression level.
    #[error("{algorithm} does not support compression level {level}: {reason}")]
    UnsupportedLevel {
        /// Adapter name.
        algorithm: &'static str,
        /// Requested level.
        level: u32,
        /// Reason the level is unsupported.
        reason: &'static str,
    },
    /// The selected adapter could not compress the input.
    #[error("{algorithm} compression failed")]
    Compress {
        /// Adapter name.
        algorithm: &'static str,
        /// Original IO or codec cause.
        #[source]
        source: std::io::Error,
    },
    /// The selected adapter could not decompress the input.
    #[error("{algorithm} decompression failed")]
    Decompress {
        /// Adapter name.
        algorithm: &'static str,
        /// Original IO or codec cause.
        #[source]
        source: std::io::Error,
    },
}
