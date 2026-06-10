//! Focused collection and iterator helpers.
//!
//! Prefer standard library iterator, slice, and map APIs when they already
//! express the operation clearly. This crate is for repeated backend-service
//! patterns that benefit from a named helper and typed error behavior.
//!
//! ```
//! use bluetape_rs_collections::{iter, map, slice};
//!
//! // Helper APIs will be added under these focused namespaces.
//! ```

pub mod error;
pub mod iter;
pub mod map;
pub mod slice;

pub use error::CollectionError;
