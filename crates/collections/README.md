# bluetape-rs-collections

Focused collection and iterator helpers for bluetape-rs.

This crate starts as the `0.2.0` collection helper boundary. It intentionally
keeps the initial surface small so helper APIs can be added only when they are
more expressive than standard library iterator, slice, and map methods.

## Scope

- iterator helpers under `iter`: owned chunks, sliding windows, predicate
  chunking, grouping, frequency counts, and `Result` partitioning
- map helpers under `map`: value transforms for `HashMap`
- slice helpers under `slice`: clamped signed ranges and zero-copy padding when
  borrowing can add value beyond `std`
- error-aware transforms when they improve `Result`-based flows

## Usage

```toml
[dependencies]
bluetape-rs-collections = "0.2.0"
```

```rust
use std::collections::HashMap;

use bluetape_rs_collections::{iter, map, slice};

let windows: Vec<_> = iter::sliding_windows([1, 2, 3, 4], 3, true)
    .unwrap()
    .collect();
assert_eq!(windows, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4], vec![4]]);

let values = HashMap::from([("a", 1), ("b", 2)]);
let doubled = map::map_values(values, |value| value * 2);
assert_eq!(doubled.get("a"), Some(&2));

let values = [1, 2, 3, 4, 5];
assert_eq!(slice::clamped_subslice(&values, -10, 3), &[1, 2, 3]);
```
