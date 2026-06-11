# bluetape-rs-compression

[English](README.md) | [한국어](README.ko.md)

Rust-native opt-in compression helpers for `bluetape-rs`.

The crate exposes a small `Compressor` trait, typed configuration, typed errors,
one-shot byte helpers, stream copy helpers, and feature-gated adapters for
gzip, zlib, deflate, zstd, lz4, and snappy.
Default features are empty; select the algorithm features you need. Use `all`
for benchmark/dev comparison, not as a default downstream recommendation.
The `zstd` feature enables the native `zstd-sys` build path.
Decode helpers apply a 64 MiB decompressed-size safety limit by default. Use
`CompressionConfig::with_max_decompressed_size` for a smaller bound, or
`CompressionConfig::without_decompressed_size_limit` only when another trusted
layer already bounds decoded output.
`lz4` and `snappy` expose fixed-level block codecs and reject explicit
non-default compression levels. Their one-shot helpers keep block/raw payload
formats; their stream helpers use framed stream formats and should round-trip
through the matching stream API.

```toml
[dependencies]
bluetape-rs-compression = { version = "0.4.0", default-features = false, features = ["gzip"] }
```

```rust
use bluetape_rs_compression::{Compressor, Gzip};

let payload = b"{\"message\":\"hello\"}";
let compressed = Gzip.compress(payload)?;
let restored = Gzip.decompress(&compressed)?;

assert_eq!(restored, payload);
# Ok::<(), bluetape_rs_compression::CompressionError>(())
```

```rust
use bluetape_rs_compression::{CompressionConfig, Compressor, Gzip};

let payload = b"{\"message\":\"hello\"}";
let compressed = Gzip.compress(payload)?;

let restored = Gzip.decompress_with_config(
    &compressed,
    CompressionConfig::new().with_max_decompressed_size(1024),
)?;

assert_eq!(restored, payload);
# Ok::<(), bluetape_rs_compression::CompressionError>(())
```

```rust
use bluetape_rs_compression::{CompressionConfig, Compressor, Gzip};

let payload = b"{\"message\":\"hello\"}";
let mut compressed = Vec::new();
Gzip.compress_stream(&payload[..], &mut compressed, CompressionConfig::new())?;

let mut restored = Vec::new();
Gzip.decompress_stream(&compressed[..], &mut restored, CompressionConfig::new())?;

assert_eq!(restored, payload);
# Ok::<(), bluetape_rs_compression::CompressionError>(())
```

```rust
use bluetape_rs_compression::{CompressionConfig, Compressor, Gzip};
use std::io::{Read, Write};

let mut writer = Gzip.compression_writer(Vec::new(), CompressionConfig::new())?;
writer.write_all(b"{\"message\":\"hello\"}")?;
let compressed = writer.finish()?;

let mut reader = Gzip.decompression_reader(&compressed[..], CompressionConfig::new())?;
let mut restored = Vec::new();
reader.read_to_end(&mut restored)?;

assert_eq!(restored, b"{\"message\":\"hello\"}");
# Ok::<(), Box<dyn std::error::Error>>(())
```
