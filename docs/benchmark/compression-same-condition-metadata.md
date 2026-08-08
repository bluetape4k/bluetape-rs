# 압축 벤치마크 메타데이터

이 benchmark data는 production ranking이 아닌 local same-condition snapshot입니다.
Rust compression crate를 구성하는 동안 동일한 payload byte에서
`bluetape-rs`, `bluetape-go`, `bluetape4k-io`를 비교할 수 있게 하는
것이 목적입니다.

## 소스 리비전

| ecosystem | repository | revision |
|---|---|---|
| bluetape-rs | `bluetape4k/bluetape-rs` | `5b211ccaa2e2967179580260095e05690c2319f5` plus this PR diff |
| bluetape-go | `/Users/debop/work/bluetape4k/bluetape-go` | `076d6f98bd5670e8a7c2c3911ac28194541bf42b` |
| bluetape4k-io | `/Users/debop/work/bluetape4k/bluetape4k-projects` | `aa7aa4171e520c90cf8418bfebb885990068021c` |

## 재현성 파일

- Go benchmark harness: `benchmark/compression-benchmark/go`
- fixture generator: `benchmark/compression-benchmark/go/cmd/generate-payloads`
- payload file로 생성한 fixture manifest:
  `docs/benchmark/compression-fixtures-manifest.csv`
- Go 원시 벤치마크 캡처: `docs/benchmark/raw/go-same-condition.txt`
- Rust CSV: `docs/benchmark/compression-same-condition-rust.csv`
- Go CSV: `docs/benchmark/compression-same-condition-go.csv`
- JVM CSV: `docs/benchmark/compression-same-condition-jvm.csv`
- report renderer: `benchmark/compression-benchmark/scripts/render_report.py`
- CSV normalizer: `benchmark/compression-benchmark/scripts/normalize_csv.py`

모든 CSV 파일은 동일한 schema를 사용합니다.

```text
ecosystem,compressor,direction,payload_kind,payload_size,original_bytes,compressed_bytes,ratio,iterations,total_ns,ns_op,mib_s,timing_provenance
```

## 명령

Rust command는 `/Users/debop/work/bluetape4k/bluetape-rs`에서 실행합니다. Go
benchmark module은 `/Users/debop/work/bluetape4k/bluetape-go`를 가리키는
local `replace`를 사용하므로 위에 기록한 revision의 sibling checkout이
있어야 합니다. JVM CSV는 tracked snapshot data로 보존합니다.
`bluetape4k-projects`에서 다시 실행하는 작업은 기록한 revision에 tracked
same-condition compression benchmark test selector가 없어 현재 BLOCKED입니다.

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

## 주의 사항

- run은 Apple M5, darwin/arm64에서 실행한 단일 local snapshot입니다.
- 모든 ecosystem에서 throughput을 MiB/s로 정규화했습니다.
- fixture generator는 세 benchmark harness가 사용하는 동일한 byte에서
  payload manifest를 작성합니다.
- harness는 의도적으로 lightweight하며 통계적으로 동등하지 않습니다.
  Rust는 고정 iteration `Instant` loop, Go는 `testing.B`, JVM은
  Gradle/JUnit benchmark-style test를 사용합니다.
- Go benchmark allocation counter는 raw `-benchmem` output에 보존합니다.
  normalized CSV schema에는 모든 harness가 제공할 수 있는 metric만
  유지합니다. 이 snapshot에서는 Rust/JVM allocation 및 memory counter를
  수집하지 않았습니다.
- 결과는 동일한 payload byte에서 broad behavior를 비교하는 데 사용하고,
  stable production ranking이나 regression threshold를 주장하는 데
  사용하지 않습니다.
- `zlib`은 Rust와 Go raw CSV에 존재하지만 JVM comparison set에 zlib이
  없어 common chart에서는 제외합니다.
