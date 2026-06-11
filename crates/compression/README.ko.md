# bluetape-rs-compression

[English](README.md) | [한국어](README.ko.md)

`bluetape-rs`를 위한 Rust-native opt-in compression helper입니다.

이 crate는 작은 `Compressor` trait, typed configuration, typed error, one-shot byte
helper, stream copy helper, 그리고 gzip, zlib, deflate, zstd, lz4, snappy를 위한
feature-gated adapter를 제공합니다.

Default feature는 비어 있습니다. 필요한 algorithm feature만 선택하세요. `all`은
benchmark/dev 비교용이며 downstream default recommendation으로 사용하지 않습니다.
`zstd` feature는 native `zstd-sys` build path를 활성화합니다.

Decode helper는 기본적으로 64 MiB decompressed-size safety limit을 적용합니다.
더 작은 bound가 필요하면 `CompressionConfig::with_max_decompressed_size`를
사용하고, 이미 다른 신뢰 계층이 decoded output을 제한할 때만
`CompressionConfig::without_decompressed_size_limit`를 사용하세요.

`lz4`와 `snappy`는 fixed-level block codec을 노출하며 명시적인 non-default
compression level을 거부합니다. One-shot helper는 block/raw payload format을
유지하고, stream helper는 framed stream format을 사용하므로 같은 stream API로
round-trip해야 합니다.

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
