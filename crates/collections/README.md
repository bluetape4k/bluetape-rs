# bluetape-rs-collections

Focused collection and iterator helpers for bluetape-rs.

This crate starts as the `0.2.0` collection helper boundary. It intentionally
keeps the initial surface small so helper APIs can be added only when they are
more expressive than standard library iterator, slice, and map methods.

## Scope

- iterator helpers under `iter`
- map helpers under `map`
- slice helpers under `slice`
- error-aware transforms when they improve `Result`-based flows

## Usage

```toml
[dependencies]
bluetape-rs-collections = "0.2.0"
```

```rust
use bluetape_rs_collections::{iter, map, slice};

// Helper APIs will be added under these focused namespaces.
```
