//! Error types returned by collection helpers.

use std::error::Error;
use std::fmt;

/// Errors produced by collection helper constructors.
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CollectionError {
    /// A size parameter was zero, but the helper requires a positive value.
    InvalidSize {
        /// Name of the invalid parameter.
        parameter: &'static str,
        /// Invalid value supplied by the caller.
        value: usize,
    },
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSize { parameter, value } => {
                write!(f, "{parameter} must be positive, got {value}")
            }
        }
    }
}

impl Error for CollectionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_size_formats_public_error_message() {
        let error = CollectionError::InvalidSize {
            parameter: "window_size",
            value: 0,
        };

        assert_eq!(error.to_string(), "window_size must be positive, got 0");
        assert!(error.source().is_none());
    }
}
