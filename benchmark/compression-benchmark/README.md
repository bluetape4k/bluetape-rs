# compression-benchmark

[English](README.md) | [한국어](README.ko.md)

Same-condition compression benchmark runner for the `bluetape-rs-compression`
crate.

The runner reads the shared payload matrix produced for the bluetape compression
comparison work and writes a CSV that can be merged with Go and Kotlin results.
It keeps payload kind, payload size, compressor name, direction, throughput, and
compression ratio in a common schema.

## Scope

- payload kinds: `json`, `text`, `binary`, `random`
- payload sizes: `small`, `medium`, `large`
- compressors: every algorithm returned by `bluetape_rs_compression::algorithms()`
- directions: `compress` and `decompress`
- output: CSV rows for same-condition reporting and chart generation

## Usage

```bash
cargo run -p compression-benchmark -- \
  --payload-dir /tmp/bluetape-compression-bench/payloads \
  --output docs/benchmark/compression-same-condition-rust.csv
```

The payload directory must contain the full `{kind}-{size}.bin` matrix, for
example `json-small.bin`, `binary-medium.bin`, and `random-large.bin`.

## Output Columns

- `ecosystem`
- `compressor`
- `direction`
- `payload_kind`
- `payload_size`
- `original_bytes`
- `compressed_bytes`
- `ratio`
- `iterations`
- `total_ns`
- `ns_op`
- `mib_s`
- `timing_provenance`
