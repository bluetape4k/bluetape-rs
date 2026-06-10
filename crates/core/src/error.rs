use std::error::Error;
use std::fmt::{self, Display};

/// Error returned when caller-owned input violates a validation contract.
///
/// The variants preserve the caller-facing field name and string-rendered
/// values so failures can be reported without losing the original validation
/// context.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::{ValidationError, require_positive};
///
/// let error = require_positive("workers", 0).unwrap_err();
/// assert!(matches!(error, ValidationError::NotPositive { .. }));
/// assert_eq!(error.to_string(), "workers[0] must be positive");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationError {
    /// A named value was empty.
    Empty {
        /// Caller-facing value name.
        name: String,
    },
    /// A named value was empty or contained only whitespace.
    Blank {
        /// Caller-facing value name.
        name: String,
    },
    /// A range definition was invalid.
    InvalidRange {
        /// Caller-facing range name.
        name: String,
        /// Rendered lower bound.
        lower: String,
        /// Rendered upper bound.
        upper: String,
        /// Boundary semantics used for the range.
        kind: RangeKind,
    },
    /// A named value was outside the accepted range.
    OutOfRange {
        /// Caller-facing value name.
        name: String,
        /// Rendered rejected value.
        value: String,
        /// Rendered lower bound.
        lower: String,
        /// Rendered upper bound.
        upper: String,
        /// Boundary semantics used for the range.
        kind: RangeKind,
    },
    /// A named value was expected to be positive.
    NotPositive {
        /// Caller-facing value name.
        name: String,
        /// Rendered rejected value.
        value: String,
    },
    /// A named value was expected to be non-negative.
    Negative {
        /// Caller-facing value name.
        name: String,
        /// Rendered rejected value.
        value: String,
    },
    /// A named floating-point value was expected to be finite.
    NonFinite {
        /// Caller-facing value name.
        name: String,
        /// Rendered rejected value.
        value: String,
    },
    /// A byte limit was invalid.
    NegativeLimit {
        /// Caller-facing limit name.
        name: String,
        /// Rejected byte limit.
        value: isize,
    },
}

/// Range boundary semantics for validation errors.
///
/// # Examples
///
/// ```
/// use bluetape_rs_core::RangeKind;
///
/// assert_eq!(RangeKind::Inclusive.to_string(), "inclusive");
/// assert_eq!(RangeKind::HalfOpen.to_string(), "half-open");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RangeKind {
    /// Inclusive range: `[lower, upper]`.
    Inclusive,
    /// Half-open range: `[lower, upper)`.
    HalfOpen,
}

impl Display for RangeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inclusive => f.write_str("inclusive"),
            Self::HalfOpen => f.write_str("half-open"),
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { name } => write!(f, "{name} must not be empty"),
            Self::Blank { name } => write!(f, "{name} must not be blank"),
            Self::InvalidRange {
                name,
                lower,
                upper,
                kind,
            } => write!(
                f,
                "{name} {kind} range is invalid: lower {lower} must be less than upper {upper}"
            ),
            Self::OutOfRange {
                name,
                value,
                lower,
                upper,
                kind,
            } => match kind {
                RangeKind::Inclusive => {
                    write!(f, "{name}[{value}] must be in range [{lower}, {upper}]")
                }
                RangeKind::HalfOpen => {
                    write!(f, "{name}[{value}] must be in range [{lower}, {upper})")
                }
            },
            Self::NotPositive { name, value } => write!(f, "{name}[{value}] must be positive"),
            Self::Negative { name, value } => write!(f, "{name}[{value}] must be non-negative"),
            Self::NonFinite { name, value } => write!(f, "{name}[{value}] must be finite"),
            Self::NegativeLimit { name, value } => {
                write!(f, "{name}[{value}] must be non-negative")
            }
        }
    }
}

impl Error for ValidationError {}
