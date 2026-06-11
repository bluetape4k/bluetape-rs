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
    /// The selected adapter does not support the requested operation.
    #[error("{algorithm} does not support {operation}: {reason}")]
    UnsupportedOperation {
        /// Adapter name.
        algorithm: &'static str,
        /// Operation name.
        operation: &'static str,
        /// Reason the operation is unsupported.
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
    /// The selected adapter could not read plain input for streaming
    /// compression.
    #[error("{algorithm} compression stream read failed")]
    CompressRead {
        /// Adapter name.
        algorithm: &'static str,
        /// Original IO cause.
        #[source]
        source: std::io::Error,
    },
    /// The selected adapter could not write compressed output.
    #[error("{algorithm} compression stream write failed")]
    CompressWrite {
        /// Adapter name.
        algorithm: &'static str,
        /// Original IO or codec cause.
        #[source]
        source: std::io::Error,
    },
    /// The selected adapter could not finish a streaming compression writer.
    #[error("{algorithm} compression stream finish failed")]
    CompressFinish {
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
    /// The selected adapter could not construct a streaming decoder.
    #[error("{algorithm} decompression stream initialization failed")]
    DecompressInit {
        /// Adapter name.
        algorithm: &'static str,
        /// Original IO or codec cause.
        #[source]
        source: std::io::Error,
    },
    /// The selected adapter could not read decoded output from a decoder.
    #[error("{algorithm} decompression stream read failed")]
    DecompressRead {
        /// Adapter name.
        algorithm: &'static str,
        /// Original IO or codec cause.
        #[source]
        source: std::io::Error,
    },
    /// The selected adapter could not write decompressed output.
    #[error("{algorithm} decompression stream write failed")]
    DecompressWrite {
        /// Adapter name.
        algorithm: &'static str,
        /// Original IO cause.
        #[source]
        source: std::io::Error,
    },
    /// The decompressed payload exceeded the configured safety limit.
    #[error("{algorithm} decompressed size {actual} exceeded limit {limit}")]
    DecompressedSizeLimitExceeded {
        /// Adapter name.
        algorithm: &'static str,
        /// Configured maximum decompressed bytes.
        limit: usize,
        /// Observed decompressed bytes before stopping.
        actual: usize,
    },
}
