//! Map helper namespace.
//!
//! Helper APIs added here should keep key ownership, collision behavior, and
//! ordering guarantees explicit.

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Transforms every value in a [`HashMap`] while preserving keys and hasher.
///
/// This is a small named helper for a common backend flow that Rust's standard
/// library expresses only as a manual loop or `into_iter().map(...).collect()`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use bluetape_rs_collections::map;
///
/// let values = HashMap::from([("a", 1), ("b", 2)]);
/// let doubled = map::map_values(values, |value| value * 2);
/// assert_eq!(doubled.get("a"), Some(&2));
/// assert_eq!(doubled.get("b"), Some(&4));
/// ```
#[must_use]
pub fn map_values<K, V, U, S, F>(map: HashMap<K, V, S>, mut transform: F) -> HashMap<K, U, S>
where
    K: Eq + Hash,
    S: BuildHasher + Clone,
    F: FnMut(V) -> U,
{
    let mut output = HashMap::with_capacity_and_hasher(map.len(), map.hasher().clone());
    for (key, value) in map {
        output.insert(key, transform(value));
    }
    output
}

/// Transforms every value in a [`HashMap`], stopping on the first error.
///
/// Keys and the original hasher are preserved for all values transformed before
/// an error occurs. The partially transformed map is discarded when an error is
/// returned.
///
/// # Errors
///
/// Returns the first error produced by `transform`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use bluetape_rs_collections::map;
///
/// let values = HashMap::from([("a", "1"), ("b", "2")]);
/// let parsed = map::try_map_values(values, str::parse::<i32>).unwrap();
/// assert_eq!(parsed.get("a"), Some(&1));
/// assert_eq!(parsed.get("b"), Some(&2));
/// ```
pub fn try_map_values<K, V, U, S, F, E>(
    map: HashMap<K, V, S>,
    mut transform: F,
) -> Result<HashMap<K, U, S>, E>
where
    K: Eq + Hash,
    S: BuildHasher + Clone,
    F: FnMut(V) -> Result<U, E>,
{
    let mut output = HashMap::with_capacity_and_hasher(map.len(), map.hasher().clone());
    for (key, value) in map {
        output.insert(key, transform(value)?);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_values_preserves_keys_and_transforms_values() {
        let map = HashMap::from([("a", 1), ("b", 2)]);
        let actual = map_values(map, |value| value * 10);

        assert_eq!(actual.get("a"), Some(&10));
        assert_eq!(actual.get("b"), Some(&20));
    }

    #[test]
    fn try_map_values_returns_transformed_map() {
        let map = HashMap::from([("a", "1"), ("b", "2")]);
        let actual = try_map_values(map, str::parse::<i32>).unwrap();

        assert_eq!(actual.get("a"), Some(&1));
        assert_eq!(actual.get("b"), Some(&2));
    }

    #[test]
    fn try_map_values_stops_on_first_error() {
        let map = HashMap::from([("a", "1"), ("b", "not-a-number")]);
        let actual = try_map_values(map, str::parse::<i32>);

        assert!(actual.is_err());
    }
}
