//! Core validation, string, and numeric helpers.
//!
//! Prefer the Rust standard library when it already expresses the operation
//! clearly. This crate is for small repeated backend-service patterns.
//!
//! ```
//! use bluetape_rs_core::{require_in_range, require_not_blank};
//!
//! let name = require_not_blank("name", "bluetape").expect("name");
//! let port = require_in_range("port", 8080, 1, 65_535).expect("port");
//!
//! assert_eq!(name, "bluetape");
//! assert_eq!(port, 8080);
//! ```

mod error;
mod hex;
mod number;
mod string;

pub use error::{RangeKind, ValidationError};
pub use hex::{is_hex_digit, is_prefixed_hex};
pub use number::{
    Number, clamp, require_in_half_open_range, require_in_range, require_non_negative,
    require_positive,
};
pub use string::{
    blank_to_default, empty_to_default, has_text, require_not_blank, require_not_empty,
    truncate_utf8_bytes,
};

#[cfg(test)]
mod tests;
