//! Iterator helper namespace.
//!
//! Helper APIs added here should preserve standard iterator ergonomics and avoid
//! hiding allocation, ordering, or error propagation behavior.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use crate::CollectionError;

/// Creates a lazy iterator over non-overlapping chunks of `size`.
///
/// This fills the stable `Iterator` gap for owned, generic iterators. Prefer
/// slice methods such as [`slice::chunks`](slice::chunks()) when the input is
/// already a slice.
///
/// # Errors
///
/// Returns [`CollectionError::InvalidSize`] when `size` is zero.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::iter;
///
/// let chunks: Vec<_> = iter::chunks(1..=5, 2).unwrap().collect();
/// assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
/// ```
pub fn chunks<I>(iterable: I, size: usize) -> Result<Chunks<I::IntoIter>, CollectionError>
where
    I: IntoIterator,
{
    if size == 0 {
        return Err(CollectionError::InvalidSize {
            parameter: "size",
            value: size,
        });
    }

    Ok(Chunks {
        iter: iterable.into_iter(),
        size,
    })
}

/// Lazy iterator returned by [`chunks`].
#[derive(Debug, Clone)]
pub struct Chunks<I> {
    iter: I,
    size: usize,
}

impl<I> Iterator for Chunks<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            match self.iter.next() {
                Some(item) => chunk.push(item),
                None => break,
            }
        }

        if chunk.is_empty() { None } else { Some(chunk) }
    }
}

/// Creates exact overlapping windows of `size`.
///
/// This is shorthand for [`sliding_windows`] with `partial_windows = false`.
/// Items are cloned into each yielded window because generic iterators cannot
/// lend overlapping borrowed slices safely.
///
/// # Errors
///
/// Returns [`CollectionError::InvalidSize`] when `size` is zero.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::iter;
///
/// let windows: Vec<_> = iter::windows([1, 2, 3, 4], 3).unwrap().collect();
/// assert_eq!(windows, vec![vec![1, 2, 3], vec![2, 3, 4]]);
/// ```
pub fn windows<I>(iterable: I, size: usize) -> Result<SlidingWindows<I::IntoIter>, CollectionError>
where
    I: IntoIterator,
    I::Item: Clone,
{
    sliding_windows(iterable, size, false)
}

/// Creates overlapping windows of `size`, optionally including shrinking tails.
///
/// This mirrors the useful part of bluetape4k's `sliding(size,
/// partialWindows)` helper while keeping Rust's allocation behavior explicit.
/// Prefer [`slice::windows`](slice::windows()) for borrowed exact windows
/// over a slice.
///
/// # Errors
///
/// Returns [`CollectionError::InvalidSize`] when `size` is zero.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::iter;
///
/// let windows: Vec<_> = iter::sliding_windows([1, 2, 3, 4], 3, true)
///     .unwrap()
///     .collect();
/// assert_eq!(windows, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4], vec![4]]);
/// ```
pub fn sliding_windows<I>(
    iterable: I,
    size: usize,
    partial_windows: bool,
) -> Result<SlidingWindows<I::IntoIter>, CollectionError>
where
    I: IntoIterator,
    I::Item: Clone,
{
    if size == 0 {
        return Err(CollectionError::InvalidSize {
            parameter: "size",
            value: size,
        });
    }

    Ok(SlidingWindows {
        iter: iterable.into_iter(),
        size,
        partial_windows,
        buffer: VecDeque::with_capacity(size),
        initialized: false,
        source_exhausted: false,
    })
}

/// Lazy iterator returned by [`sliding_windows`] and [`windows`].
#[derive(Debug, Clone)]
pub struct SlidingWindows<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
    size: usize,
    partial_windows: bool,
    buffer: VecDeque<I::Item>,
    initialized: bool,
    source_exhausted: bool,
}

impl<I> Iterator for SlidingWindows<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.initialized {
            self.initialized = true;
            while self.buffer.len() < self.size {
                match self.iter.next() {
                    Some(item) => self.buffer.push_back(item),
                    None => {
                        self.source_exhausted = true;
                        break;
                    }
                }
            }

            return self.current_window();
        }

        self.buffer.pop_front();
        if !self.source_exhausted {
            match self.iter.next() {
                Some(item) => self.buffer.push_back(item),
                None => self.source_exhausted = true,
            }
        }

        self.current_window()
    }
}

impl<I> SlidingWindows<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn current_window(&self) -> Option<Vec<I::Item>> {
        if self.buffer.len() == self.size || (self.partial_windows && !self.buffer.is_empty()) {
            Some(self.buffer.iter().cloned().collect())
        } else {
            None
        }
    }
}

/// Splits an iterable whenever `predicate` marks the next chunk's first item.
///
/// The matching item is retained as the first item in the new chunk, matching
/// bluetape4k-core's `chunkedBy` behavior. Empty input returns no chunks. If
/// the first item matches the predicate, it starts the first chunk; no empty
/// leading chunk is emitted.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::iter;
///
/// let chunks = iter::chunked_by([1, 2, 3, 4, 5], |value| *value == 3);
/// assert_eq!(chunks, vec![vec![1, 2], vec![3, 4, 5]]);
/// ```
#[must_use]
pub fn chunked_by<I, F>(iterable: I, mut predicate: F) -> Vec<Vec<I::Item>>
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> bool,
{
    let mut result = Vec::new();
    let mut current = Vec::new();

    for item in iterable {
        if predicate(&item) && !current.is_empty() {
            result.push(current);
            current = Vec::new();
        }
        current.push(item);
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

/// Counts occurrences of each item.
///
/// Rust's standard library exposes the building blocks for this via
/// [`Iterator::fold`], but not a named frequency helper. The returned
/// [`HashMap`] has unspecified iteration order.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::iter;
///
/// let counts = iter::frequencies(["a", "b", "a"]);
/// assert_eq!(counts.get("a"), Some(&2));
/// assert_eq!(counts.get("b"), Some(&1));
/// ```
#[must_use]
pub fn frequencies<I>(iterable: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + Hash,
{
    let mut counts = HashMap::new();
    for item in iterable {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

/// Groups items by a key derived from each item.
///
/// Rust's standard library has the primitives for this through `entry`, but no
/// built-in `group_by` collector for owned iterators. Items within each group
/// retain input order, while the returned [`HashMap`] has unspecified key
/// iteration order.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::iter;
///
/// let grouped = iter::group_by(["ape", "ant", "bear"], |value| value.chars().next().unwrap());
/// assert_eq!(grouped.get(&'a'), Some(&vec!["ape", "ant"]));
/// assert_eq!(grouped.get(&'b'), Some(&vec!["bear"]));
/// ```
#[must_use]
pub fn group_by<I, K, F>(iterable: I, mut key_selector: F) -> HashMap<K, Vec<I::Item>>
where
    I: IntoIterator,
    K: Eq + Hash,
    F: FnMut(&I::Item) -> K,
{
    let mut groups = HashMap::new();
    for item in iterable {
        groups
            .entry(key_selector(&item))
            .or_insert_with(Vec::new)
            .push(item);
    }
    groups
}

/// Partitions `Result` items into successes and errors.
///
/// Standard [`Iterator::partition`] can split by `Result::is_ok`, but it keeps
/// the wrappers. This helper returns the unwrapped values directly.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::iter;
///
/// let (values, errors) = iter::partition_results([Ok(1), Err("bad"), Ok(2)]);
/// assert_eq!(values, vec![1, 2]);
/// assert_eq!(errors, vec!["bad"]);
/// ```
#[must_use]
pub fn partition_results<I, T, E>(iterable: I) -> (Vec<T>, Vec<E>)
where
    I: IntoIterator<Item = Result<T, E>>,
{
    let mut values = Vec::new();
    let mut errors = Vec::new();

    for item in iterable {
        match item {
            Ok(value) => values.push(value),
            Err(error) => errors.push(error),
        }
    }

    (values, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_yields_non_overlapping_vectors() {
        let actual: Vec<_> = chunks(1..=5, 2).unwrap().collect();
        assert_eq!(actual, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn chunks_rejects_zero_size() {
        let error = chunks([1, 2, 3], 0).unwrap_err();
        assert_eq!(
            error,
            CollectionError::InvalidSize {
                parameter: "size",
                value: 0
            }
        );
    }

    #[test]
    fn windows_yields_exact_overlapping_windows() {
        let actual: Vec<_> = windows([1, 2, 3, 4], 3).unwrap().collect();
        assert_eq!(actual, vec![vec![1, 2, 3], vec![2, 3, 4]]);
    }

    #[test]
    fn windows_returns_empty_when_input_is_shorter_than_size() {
        let actual: Vec<_> = windows([1, 2], 3).unwrap().collect();
        assert!(actual.is_empty());
    }

    #[test]
    fn sliding_windows_can_include_partial_tails() {
        let actual: Vec<_> = sliding_windows([1, 2, 3, 4], 3, true).unwrap().collect();
        assert_eq!(
            actual,
            vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4], vec![4]]
        );
    }

    #[test]
    fn sliding_windows_rejects_zero_size() {
        let error = sliding_windows([1, 2, 3], 0, true).unwrap_err();
        assert_eq!(
            error,
            CollectionError::InvalidSize {
                parameter: "size",
                value: 0
            }
        );
    }

    #[test]
    fn chunked_by_starts_new_chunk_at_matching_item() {
        let actual = chunked_by([1, 2, 3, 4, 5], |value| *value == 3);
        assert_eq!(actual, vec![vec![1, 2], vec![3, 4, 5]]);
    }

    #[test]
    fn chunked_by_returns_empty_for_empty_input() {
        let actual = chunked_by(Vec::<i32>::new(), |_| true);
        assert!(actual.is_empty());
    }

    #[test]
    fn chunked_by_does_not_emit_empty_leading_chunk() {
        let actual = chunked_by([1, 2, 3], |value| *value == 1);
        assert_eq!(actual, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn frequencies_counts_items() {
        let actual = frequencies(["a", "b", "a"]);
        assert_eq!(actual.get("a"), Some(&2));
        assert_eq!(actual.get("b"), Some(&1));
    }

    #[test]
    fn group_by_groups_items_by_selected_key() {
        let actual = group_by(["ape", "ant", "bear"], |value| {
            value.chars().next().unwrap()
        });

        assert_eq!(actual.get(&'a'), Some(&vec!["ape", "ant"]));
        assert_eq!(actual.get(&'b'), Some(&vec!["bear"]));
    }

    #[test]
    fn sliding_windows_can_include_partial_windows_when_input_is_short() {
        let actual: Vec<_> = sliding_windows([1, 2], 3, true).unwrap().collect();
        assert_eq!(actual, vec![vec![1, 2], vec![2]]);
    }

    #[test]
    fn partition_results_unwraps_successes_and_errors() {
        let actual = partition_results([Ok(1), Err("bad"), Ok(2)]);
        assert_eq!(actual, (vec![1, 2], vec!["bad"]));
    }
}
