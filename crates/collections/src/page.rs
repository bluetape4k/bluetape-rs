//! Page value types for page-number based collection results.
//!
//! This module models an already materialized page of items. It deliberately
//! does not define database pagination, cursor pagination, repository APIs, or
//! SQL query builders.

use std::error::Error;
use std::fmt;

/// Default page size used by [`Page::new`].
pub const DEFAULT_PAGE_SIZE: u64 = 10;

/// Error returned by [`Page`] constructors.
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PageError {
    /// Page size must be greater than zero.
    InvalidPageSize {
        /// Invalid page size supplied by the caller.
        value: u64,
    },
}

impl fmt::Display for PageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPageSize { value } => {
                write!(f, "page_size must be positive, got {value}")
            }
        }
    }
}

impl Error for PageError {}

/// A materialized page of items with page-number metadata.
///
/// Page numbers are zero-based. `total_items` is the total number of matching
/// items across all pages, not the number of items in this page.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::Page;
///
/// let page = Page::with_meta(vec!["a", "b"], 0, 2, 5).unwrap();
/// assert_eq!(page.items(), &["a", "b"]);
/// assert_eq!(page.total_pages(), 3);
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub struct Page<T> {
    items: Vec<T>,
    page_number: u64,
    page_size: u64,
    total_items: u64,
}

impl<T> Page<T> {
    /// Creates the first page using [`DEFAULT_PAGE_SIZE`].
    ///
    /// # Errors
    ///
    /// This constructor currently cannot fail because [`DEFAULT_PAGE_SIZE`] is
    /// positive. It returns `Result` to keep constructor ergonomics aligned with
    /// [`Page::with_meta`].
    pub fn new(items: Vec<T>, total_items: u64) -> Result<Self, PageError> {
        Self::with_meta(items, 0, DEFAULT_PAGE_SIZE, total_items)
    }

    /// Creates a page with explicit page-number metadata.
    ///
    /// # Errors
    ///
    /// Returns [`PageError::InvalidPageSize`] when `page_size` is zero.
    pub fn with_meta(
        items: Vec<T>,
        page_number: u64,
        page_size: u64,
        total_items: u64,
    ) -> Result<Self, PageError> {
        if page_size == 0 {
            return Err(PageError::InvalidPageSize { value: page_size });
        }

        Ok(Self {
            items,
            page_number,
            page_size,
            total_items,
        })
    }

    /// Returns the items in this page.
    #[must_use]
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Consumes the page and returns its items.
    #[must_use]
    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    /// Returns this page's zero-based page number.
    #[must_use]
    pub fn page_number(&self) -> u64 {
        self.page_number
    }

    /// Returns the requested page size.
    #[must_use]
    pub fn page_size(&self) -> u64 {
        self.page_size
    }

    /// Returns the total matching item count across all pages.
    #[must_use]
    pub fn total_items(&self) -> u64 {
        self.total_items
    }

    /// Returns the total page count implied by `total_items` and `page_size`.
    ///
    /// Empty result sets have zero pages. Non-empty result sets round up to
    /// include a final partial page.
    #[must_use]
    pub fn total_pages(&self) -> u64 {
        if self.total_items == 0 {
            0
        } else {
            ((self.total_items - 1) / self.page_size) + 1
        }
    }

    /// Returns the number of items in this page.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` when this page contains no items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uses_first_page_and_default_size() {
        let page = Page::new(vec![1, 2, 3], 100).unwrap();

        assert_eq!(page.page_number(), 0);
        assert_eq!(page.page_size(), DEFAULT_PAGE_SIZE);
        assert_eq!(page.total_items(), 100);
        assert_eq!(page.total_pages(), 10);
        assert_eq!(page.items(), &[1, 2, 3]);
    }

    #[test]
    fn with_meta_rounds_total_pages_up_for_remainder() {
        let page = Page::with_meta(vec![1, 2], 1, 10, 25).unwrap();

        assert_eq!(page.page_number(), 1);
        assert_eq!(page.total_pages(), 3);
    }

    #[test]
    fn with_meta_keeps_exact_total_page_count() {
        let page = Page::with_meta(vec![1, 2], 0, 10, 20).unwrap();

        assert_eq!(page.total_pages(), 2);
    }

    #[test]
    fn total_pages_is_zero_for_empty_result_sets() {
        let page = Page::<i32>::with_meta(Vec::new(), 0, 10, 0).unwrap();

        assert_eq!(page.total_pages(), 0);
        assert!(page.is_empty());
    }

    #[test]
    fn with_meta_rejects_zero_page_size() {
        let actual = Page::with_meta(vec![1], 0, 0, 1);

        assert_eq!(actual, Err(PageError::InvalidPageSize { value: 0 }));
    }

    #[test]
    fn into_items_returns_owned_items() {
        let page = Page::with_meta(vec!["a", "b"], 0, 2, 2).unwrap();

        assert_eq!(page.len(), 2);
        assert_eq!(page.into_items(), vec!["a", "b"]);
    }
}
