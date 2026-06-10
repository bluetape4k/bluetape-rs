//! Slice helper namespace.
//!
//! Helper APIs added here should prefer borrowing over allocation and document
//! boundary behavior for empty and single-element slices.

use std::borrow::Cow;

/// Returns a subslice after clamping signed bounds into the valid slice range.
///
/// This mirrors the useful part of bluetape4k-core's `safeSubList` without
/// allocating. Negative bounds clamp to `0`, bounds past the end clamp to
/// `slice.len()`, and `end` below the clamped start returns an empty subslice at
/// the start position.
///
/// Prefer normal indexing, [`slice::get`], or range patterns when the caller
/// already has trusted `usize` bounds. This helper is for user-facing or
/// protocol-facing signed offsets where clamping is the desired policy.
///
/// # Examples
///
/// ```
/// use bluetape_rs_collections::slice;
///
/// let values = [1, 2, 3, 4, 5];
/// assert_eq!(slice::clamped_subslice(&values, -10, 3), &[1, 2, 3]);
/// assert_eq!(slice::clamped_subslice(&values, 2, 99), &[3, 4, 5]);
/// assert_eq!(slice::clamped_subslice(&values, 4, 1), &[]);
/// ```
#[must_use]
pub fn clamped_subslice<T>(slice: &[T], start: isize, end: isize) -> &[T] {
    let start = clamp_bound(start, slice.len());
    let end = clamp_bound(end, slice.len()).max(start);
    &slice[start..end]
}

/// Pads a slice to `new_len`, returning a borrowed slice when no padding is
/// needed.
///
/// This is intentionally not a wrapper around [`Vec::resize`]. It accepts a
/// borrowed slice and returns [`Cow::Borrowed`] when `slice.len() >= new_len`,
/// avoiding allocation in the common no-op path. When padding is required, the
/// returned [`Cow::Owned`] contains the original elements followed by clones of
/// `fill`.
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
///
/// use bluetape_rs_collections::slice;
///
/// let values = [1, 2, 3];
/// let padded = slice::pad_to(&values, 5, 0);
/// assert_eq!(&*padded, &[1, 2, 3, 0, 0]);
/// assert!(matches!(slice::pad_to(&values, 2, 0), Cow::Borrowed(_)));
/// ```
#[must_use]
pub fn pad_to<T>(slice: &[T], new_len: usize, fill: T) -> Cow<'_, [T]>
where
    T: Clone,
{
    if slice.len() >= new_len {
        return Cow::Borrowed(slice);
    }

    let mut output = Vec::with_capacity(new_len);
    output.extend_from_slice(slice);
    output.resize(new_len, fill);
    Cow::Owned(output)
}

fn clamp_bound(bound: isize, len: usize) -> usize {
    if bound <= 0 {
        0
    } else {
        (bound as usize).min(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamped_subslice_clamps_negative_start() {
        let values = [1, 2, 3, 4, 5];
        assert_eq!(clamped_subslice(&values, -10, 3), &[1, 2, 3]);
    }

    #[test]
    fn clamped_subslice_clamps_end_past_len() {
        let values = [1, 2, 3, 4, 5];
        assert_eq!(clamped_subslice(&values, 2, 99), &[3, 4, 5]);
    }

    #[test]
    fn clamped_subslice_returns_empty_when_end_is_before_start() {
        let values = [1, 2, 3, 4, 5];
        assert_eq!(clamped_subslice(&values, 4, 1), &[]);
    }

    #[test]
    fn clamped_subslice_handles_empty_input() {
        let values: [i32; 0] = [];
        assert_eq!(clamped_subslice(&values, -1, 10), &[]);
    }

    #[test]
    fn pad_to_returns_borrowed_slice_when_no_padding_is_needed() {
        let values = [1, 2, 3];
        let padded = pad_to(&values, 2, 0);

        assert!(matches!(padded, Cow::Borrowed(_)));
        assert_eq!(&*padded, &[1, 2, 3]);
    }

    #[test]
    fn pad_to_returns_owned_vec_when_padding_is_needed() {
        let values = [1, 2, 3];
        let padded = pad_to(&values, 5, 0);

        assert!(matches!(padded, Cow::Owned(_)));
        assert_eq!(&*padded, &[1, 2, 3, 0, 0]);
    }

    #[test]
    fn pad_to_can_pad_empty_slice() {
        let values: [i32; 0] = [];
        assert_eq!(&*pad_to(&values, 3, 7), &[7, 7, 7]);
    }
}
