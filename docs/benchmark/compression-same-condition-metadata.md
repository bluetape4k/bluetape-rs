# 압축 벤치마크 메타데이터

이 벤치마크 데이터는 운영 순위가 아닌 로컬 동일 조건 스냅샷이다.
Rust 압축 크레이트를 설계하는 동안 동일한 페이로드 바이트를 기준으로
`bluetape-rs`, `bluetape-go`, `bluetape4k-io`를 비교하기 위한 자료다.

## 소스 리비전

| 생태계 | 저장소 | 리비전 |
|---|---|---|
| bluetape-rs | `bluetape4k/bluetape-rs` | `5b211ccaa2e2967179580260095e05690c2319f5` 및 이 PR의 diff |
| bluetape-go | `/Users/debop/work/bluetape4k/bluetape-go` | `076d6f98bd5670e8a7c2c3911ac28194541bf42b` |
| bluetape4k-io | `/Users/debop/work/bluetape4k/bluetape4k-projects` | `aa7aa4171e520c90cf8418bfebb885990068021c` |

## 재현성 파일

- Go 벤치마크 하네스: `benchmark/compression-benchmark/go`
- fixture 생성기: `benchmark/compression-benchmark/go/cmd/generate-payloads`
- 페이로드 파일로 생성한 fixture 매니페스트:
  `docs/benchmark/compression-fixtures-manifest.csv`
- Go 원시 벤치마크 캡처: `docs/benchmark/raw/go-same-condition.txt`
- Rust CSV: `docs/benchmark/compression-same-condition-rust.csv`
- Go CSV: `docs/benchmark/compression-same-condition-go.csv`
- JVM CSV: `docs/benchmark/compression-same-condition-jvm.csv`
- 보고서 렌더러: `benchmark/compression-benchmark/scripts/render_report.py`
- CSV 정규화 도구: `benchmark/compression-benchmark/scripts/normalize_csv.py`

모든 CSV 파일은 다음 스키마를 사용한다.

```text
ecosystem,compressor,direction,payload_kind,payload_size,original_bytes,compressed_bytes,ratio,iterations,total_ns,ns_op,mib_s,timing_provenance
```

## 명령

Rust 명령은 `/Users/debop/work/bluetape4k/bluetape-rs`에서 실행한다. Go
벤치마크 모듈은 `/Users/debop/work/bluetape4k/bluetape-go`를 가리키는 로컬
`replace`를 사용하므로 위에 기록한 리비전의 자매 체크아웃이 있어야 한다.
JVM CSV는 추적된 스냅샷 데이터로 보존한다. 기록된 리비전에 추적된 동일 조건
압축 벤치마크 테스트 selector가 없어 `bluetape4k-projects`에서 재실행하는
작업은 현재 BLOCKED다.

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

- 실행은 Apple M5, darwin/arm64에서 수행한 단일 로컬 스냅샷이다.
- 모든 생태계의 처리량은 MiB/s로 정규화했다.
- fixture 생성기는 세 벤치마크 하네스가 사용하는 동일한 바이트에서
  페이로드 매니페스트를 기록한다.
- 하네스는 의도적으로 가볍고 통계적으로 동등하지 않다. Rust는 고정 반복
  `Instant` 루프, Go는 `testing.B`, JVM은 Gradle/JUnit 벤치마크 형태의
  테스트를 사용한다.
- Go 벤치마크 할당 카운터는 원시 `-benchmem` 출력에 보존한다. 정규화 CSV
  스키마에는 모든 하네스가 제공할 수 있는 지표만 유지한다. 이 스냅샷에는
  Rust/JVM 할당 및 메모리 카운터가 수집되지 않았다.
- 동일한 페이로드 바이트의 대략적인 동작 비교에 결과를 사용하고, 안정적인
  운영 순위나 회귀 임계값을 주장하는 데 사용하지 않는다.
- `zlib`은 Rust와 Go 원시 CSV에 존재하지만 JVM 비교 집합에 zlib이 없어
  공통 차트에서는 제외한다.
