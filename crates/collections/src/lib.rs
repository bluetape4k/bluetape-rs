//! Focused collection and iterator helpers.
//!
//! Prefer standard library iterator, slice, and map APIs when they already
//! express the operation clearly. This crate is for repeated backend-service
//! patterns that benefit from a named helper and typed error behavior.
//!
//! ```
//! use bluetape_rs_collections::{iter, map, slice, Page};
//!
//! let page = Page::new(vec![1, 2, 3], 3).unwrap();
//! assert_eq!(page.total_pages(), 1);
//! ```

pub mod error;
pub mod iter;
pub mod map;
pub mod page;
pub mod slice;

pub use error::CollectionError;
pub use page::{DEFAULT_PAGE_SIZE, Page, PageError};
