# Compression Benchmark Metadata

This benchmark data is a local same-condition snapshot, not a production
ranking. It is intended to make `bluetape-rs`, `bluetape-go`, and
`bluetape4k-io` comparable on the same payload bytes while the Rust compression
crate is being shaped.

## Source Revisions

| ecosystem | repository | revision |
|---|---|---|
| bluetape-rs | `bluetape4k/bluetape-rs` | `5b211ccaa2e2967179580260095e05690c2319f5` plus this PR diff |
| bluetape-go | `/Users/debop/work/bluetape4k/bluetape-go` | `076d6f98bd5670e8a7c2c3911ac28194541bf42b` |
| bluetape4k-io | `/Users/debop/work/bluetape4k/bluetape4k-projects` | `aa7aa4171e520c90cf8418bfebb885990068021c` |

## Reproducibility Files

- Go benchmark harness: `benchmark/compression-benchmark/go`
- Fixture generator: `benchmark/compression-benchmark/go/cmd/generate-payloads`
- Fixture manifest generated with the payload files:
  `docs/benchmark/compression-fixtures-manifest.csv`
- Go raw benchmark capture: `docs/benchmark/raw/go-same-condition.txt`
- Rust CSV: `docs/benchmark/compression-same-condition-rust.csv`
- Go CSV: `docs/benchmark/compression-same-condition-go.csv`
- JVM CSV: `docs/benchmark/compression-same-condition-jvm.csv`
- Report renderer: `benchmark/compression-benchmark/scripts/render_report.py`
- CSV normalizer: `benchmark/compression-benchmark/scripts/normalize_csv.py`

All CSV files use the same schema:

```text
ecosystem,compressor,direction,payload_kind,payload_size,original_bytes,compressed_bytes,ratio,iterations,total_ns,ns_op,mib_s,timing_provenance
```

## Commands

Run the Rust commands from `/Users/debop/work/bluetape4k/bluetape-rs`. The Go
benchmark module uses a local `replace` to
`/Users/debop/work/bluetape4k/bluetape-go`, so that sibling checkout must exist
at the revision listed above. The JVM CSV is preserved as tracked snapshot data;
rerunning it from `bluetape4k-projects` is currently BLOCKED because the
recorded revision does not contain a tracked same-condition compression
benchmark test selector.

```bash
(cd benchmark/compression-benchmark/go && go run ./cmd/generate-payloads \
  --output-dir /tmp/bluetape-compression-bench/payloads \
  --manifest ../../../docs/benchmark/compression-fixtures-manifest.csv)

cargo run -p compression-benchmark --release --locked -- \
  --payload-dir /tmp/bluetape-compression-bench/payloads \
  --output docs/benchmark/compression-same-condition-rust.csv

(cd benchmark/compression-benchmark/go && \
  go test -run '^$' -bench '^BenchmarkSameConditionCompressors' \
    -benchmem -benchtime=100ms -count=1 ./... \
    > ../../../docs/benchmark/raw/go-same-condition.txt)

python3 benchmark/compression-benchmark/scripts/normalize_csv.py
python3 benchmark/compression-benchmark/scripts/render_report.py
rsvg-convert -o docs/images/readme-charts/compression-throughput-large-payloads.png \
  docs/images/readme-charts/compression-throughput-large-payloads.svg
rsvg-convert -o docs/images/readme-charts/compression-ratio-large-payloads.png \
  docs/images/readme-charts/compression-ratio-large-payloads.svg
```

## Caveats

- The run is a single local snapshot on Apple M5, darwin/arm64.
- Throughput is normalized to MiB/s across all ecosystems.
- The fixture generator writes the payload manifest from the same bytes used by
  all three benchmark harnesses.
- The harnesses are intentionally lightweight and not statistically equivalent:
  Rust uses a fixed-iteration `Instant` loop, Go uses `testing.B`, and JVM uses
  a Gradle/JUnit benchmark-style test.
- Go benchmark allocation counters are kept in the raw `-benchmem` output; the
  normalized CSV schema keeps only metrics that all harnesses can provide.
  Rust/JVM allocation and memory counters are not collected in this snapshot.
- Use the results to compare broad behavior under identical payload bytes, not
  to claim stable production rankings or regression thresholds.
- `zlib` exists in Rust and Go raw CSVs but is excluded from common charts
  because the JVM comparison set does not include zlib.
