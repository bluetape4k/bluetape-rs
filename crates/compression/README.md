# bluetape-rs-compression

Rust-native opt-in compression helpers for `bluetape-rs`.

The crate exposes a small `Compressor` trait, typed configuration, typed errors,
and feature-gated adapters for gzip, zlib, deflate, zstd, lz4, and snappy.
Default features are empty; select the algorithm features you need. Use `all`
for benchmark/dev comparison, not as a default downstream recommendation.
The `zstd` feature enables the native `zstd-sys` build path.
`lz4` and `snappy` expose fixed-level block codecs and reject explicit
non-default compression levels.

```toml
[dependencies]
# `bluetape-rs-compression` is in the unreleased 0.4.0 line.
# Use a workspace path or Git dependency until the 0.4.0 release.
bluetape-rs-compression = { path = "crates/compression", default-features = false, features = ["gzip"] }
```

```rust
use bluetape_rs_compression::{Compressor, Gzip};

let payload = b"{\"message\":\"hello\"}";
let compressed = Gzip.compress(payload)?;
let restored = Gzip.decompress(&compressed)?;

assert_eq!(restored, payload);
# Ok::<(), bluetape_rs_compression::CompressionError>(())
```
