use std::fmt::{self, Display};

/// Conventional trace field for a request identifier.
pub const REQUEST_ID_FIELD: &str = "request.id";

/// Conventional trace field for a task identifier.
pub const TASK_ID_FIELD: &str = "task.id";

/// Conventional trace field for a correlation identifier.
pub const CORRELATION_ID_FIELD: &str = "correlation.id";

/// Maximum accepted correlation identifier length in bytes.
pub const MAX_CORRELATION_ID_LEN: usize = 256;

/// A non-empty, single-line correlation identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CorrelationId(String);

/// Reason a [`CorrelationId`] value was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorrelationIdError {
    /// The identifier is empty or contains only whitespace.
    Blank,
    /// The identifier is longer than [`MAX_CORRELATION_ID_LEN`] bytes.
    TooLong {
        /// Actual identifier length in bytes.
        len: usize,
        /// Maximum accepted identifier length in bytes.
        max: usize,
    },
    /// The identifier contains a control, separator, or bidirectional control character.
    UnsafeCharacter {
        /// Rejected character.
        ch: char,
    },
}

impl Display for CorrelationIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => f.write_str("correlation id must contain visible text"),
            Self::TooLong { len, max } => write!(
                f,
                "correlation id is {len} bytes, exceeding the {max} byte limit"
            ),
            Self::UnsafeCharacter { ch } => {
                write!(
                    f,
                    "correlation id contains unsafe character U+{:04X}",
                    *ch as u32
                )
            }
        }
    }
}

impl std::error::Error for CorrelationIdError {}

impl CorrelationId {
    /// Creates a correlation identifier when `value` contains visible text, no
    /// control characters, and at most [`MAX_CORRELATION_ID_LEN`] bytes.
    pub fn new(value: impl Into<String>) -> Result<Self, CorrelationIdError> {
        let value = value.into();
        let len = value.len();
        if len > MAX_CORRELATION_ID_LEN {
            return Err(CorrelationIdError::TooLong {
                len,
                max: MAX_CORRELATION_ID_LEN,
            });
        }
        if !value.chars().any(|ch| !ch.is_whitespace()) {
            return Err(CorrelationIdError::Blank);
        }
        if let Some(ch) = value.chars().find(|ch| !is_safe_correlation_char(*ch)) {
            return Err(CorrelationIdError::UnsafeCharacter { ch });
        }
        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_safe_correlation_char(ch: char) -> bool {
    !ch.is_control()
        && !matches!(
            ch,
            '\u{061c}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{200e}'
                | '\u{200f}'
                | '\u{202a}'..='\u{202e}'
                | '\u{2066}'..='\u{2069}'
        )
}

impl Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
