# 이슈 #81/#82 Step 6-R 검토

범위: `origin/develop` 대비 `issue-81-82-benchmark-evidence`.

기준선: `5b211ccaa2e2967179580260095e05690c2319f5`.

## 검토 표면

- `benchmark/compression-benchmark`
- `docs/benchmark`
- `docs/images/readme-charts`
- `README.md`, `README.ko.md`

## 서브에이전트 레인

| 레인 | 역할 | 최종 결과 | 비고 |
|---|---|---|---|
| 일반 diff | `code-reviewer` | `P0=0 P1=0` | 매트릭스 행 누락 시 fail-fast 동작을 수정하고 Rust revision 출처를 갱신했다. |
| 인수 검증 | `verifier` | `P0=0 P1=0` | #81/#82에 생태계별 섹션, 승자 요약, 전체 매트릭스 근거를 추가했다. |
| 벤치마크/런타임 | `performance-reviewer` | `P0=0 P1=0` | 출처와 차트 스케일을 수정했다. 잔여 P2 메모의 snapshot 날짜/host는 고정 메타데이터다. |
| 라이브러리 사용자/문서 | `library-user-reviewer` | `P0=0 P1=0` | cwd, 자매 checkout 전제, Go 로컬 replace, JVM 재실행 BLOCKED 상태를 문서화했다. |

## 로컬 검증 근거

- `cargo test -p compression-benchmark --locked`
- `cargo test --workspace --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `(cd benchmark/compression-benchmark/go && go test ./...)`
- `python3 -m py_compile benchmark/compression-benchmark/scripts/normalize_csv.py benchmark/compression-benchmark/scripts/render_report.py`
- `(cd benchmark/compression-benchmark/go && go run ./cmd/generate-payloads --output-dir /tmp/bluetape-compression-bench/payloads --manifest ../../../docs/benchmark/compression-fixtures-manifest.csv)`
- `cargo run -p compression-benchmark --release --locked -- --payload-dir /tmp/bluetape-compression-bench/payloads --output /tmp/bluetape-compression-bench/rust-schema-check.csv`
- `python3 benchmark/compression-benchmark/scripts/render_report.py`
- `rsvg-convert` regenerated throughput and ratio PNG charts from SVG.
- `git diff --check`

## 초기 검토에서 수정한 사항

- Rust 벤치마크 실행기가 정규화된 `timing_provenance` 열을 직접 내보내도록 했다.
- 필요한 매트릭스 행이 없으면 보고서 렌더링이 즉시 실패하도록 했다.
- 벤치마크 fixture와 동일한 페이로드 바이트로 fixture manifest를 다시 생성했다.
- 정규화된 비교 전에 생태계별 대형 페이로드 섹션을 추가했다.
- 처리량 승자와 압축 비율 승자를 분리한 승자 요약을 추가했다.
- 전체 페이로드 매트릭스에 페이로드 종류, 크기, compressor, 생태계, 방향을 추가했다.
- 현재 대형 페이로드 처리량 값을 포함하도록 차트 스케일을 갱신했다.
- Go 전용 할당 근거와 정규화하지 않은 Rust/JVM 메모리 데이터를 명확히 했다.
- Rust, Go, JVM 근거의 cwd 및 자매 checkout 전제를 문서화했다.
- 기록된 `bluetape4k-projects` revision에 동일 조건 압축 벤치마크 selector가 추적되지 않아 JVM 재실행을 BLOCKED로 표시했다.
- 전체 매트릭스에 맞춰 README와 README.ko.md의 벤치마크 탐색 문구를 동기화했다.

## 게이트 판정

`P0=0 P1=0`

Step 6-R 로컬/네이티브 7-Tier 검토가 PR 생성 기준으로 통과했다.
