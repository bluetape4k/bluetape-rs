# 이슈 #81/#82 Step 6-R 검토

범위: `origin/develop` 대비 `issue-81-82-benchmark-evidence`

기준점: `5b211ccaa2e2967179580260095e05690c2319f5`

## 검토 대상

- `benchmark/compression-benchmark`
- `docs/benchmark`
- `docs/images/readme-charts`
- `README.md`, `README.ko.md`

## Subagent lane

| lane | 역할 | 최종 결과 | 메모 |
|---|---|---|---|
| General diff | `code-reviewer` | `P0=0 P1=0` | Matrix row 누락 시 fail-fast 동작과 Rust revision provenance를 수정했습니다. |
| Acceptance verifier | `verifier` | `P0=0 P1=0` | #81/#82를 위한 ecosystem별 section, winner summary, full matrix 근거를 추가했습니다. |
| Benchmark/runtime | `performance-reviewer` | `P0=0 P1=0` | Provenance와 chart scale을 수정했고, 잔여 P2 메모리 snapshot date/host는 고정 metadata입니다. |
| Library user/docs | `library-user-reviewer` | `P0=0 P1=0` | cwd, sibling checkout 가정, Go local replace, JVM rerun BLOCKED 상태를 문서화했습니다. |

## 로컬 검증 근거

- `cargo test -p compression-benchmark --locked`
- `cargo test --workspace --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `(cd benchmark/compression-benchmark/go && go test ./...)`
- `python3 -m py_compile benchmark/compression-benchmark/scripts/normalize_csv.py benchmark/compression-benchmark/scripts/render_report.py`
- `(cd benchmark/compression-benchmark/go && go run ./cmd/generate-payloads --output-dir /tmp/bluetape-compression-bench/payloads --manifest ../../../docs/benchmark/compression-fixtures-manifest.csv)`
- `cargo run -p compression-benchmark --release --locked -- --payload-dir /tmp/bluetape-compression-bench/payloads --output /tmp/bluetape-compression-bench/rust-schema-check.csv`
- `python3 benchmark/compression-benchmark/scripts/render_report.py`
- `rsvg-convert`가 SVG에서 throughput 및 ratio PNG chart를 재생성
- `git diff --check`

## 초기 검토에서 수정한 내용

- Rust benchmark runner가 normalized `timing_provenance` column을 직접 출력하도록 수정했습니다.
- 필요한 matrix row가 없으면 report rendering이 fail-fast하도록 수정했습니다.
- Benchmark fixture와 동일한 payload byte에서 fixture manifest를 재생성했습니다.
- Normalized comparison 전에 ecosystem별 large-payload section을 추가했습니다.
- Throughput winner와 compression-ratio winner를 구분하는 winner summary를 추가했습니다.
- Full payload matrix에 payload kind, size, compressor, ecosystem, direction을 추가했습니다.
- 현재 large-payload throughput value를 포함하도록 chart scale을 갱신했습니다.
- Go-only allocation 근거와 non-normalized Rust/JVM memory data를 명확히 했습니다.
- Rust, Go, JVM 근거에 cwd와 sibling checkout 가정을 문서화했습니다.
- 기록된 `bluetape4k-projects` revision에 추적되는 동일 조건 compression benchmark selector가 없으므로 JVM rerun을 BLOCKED로 표시했습니다.
- 전체 matrix를 탐색하는 benchmark 문구와 README 및 README.ko.md를 동기화했습니다.

## Gate 판정

`P0=0 P1=0`

Step 6-R local/native 7-Tier review가 PR 생성 기준을 통과합니다.
