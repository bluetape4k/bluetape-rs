# 이슈 #83 Step 6-R 검토

범위: `develop` 대비 `issue-83-compression-benchmark-report`

기준점: `90a937f5f422d020253749e6188eac2f2a623a1a`

## 검토 대상

- `crates/compression`
- root facade `compression*` feature
- `benchmark/compression-benchmark`
- `docs/benchmark`
- `docs/images/readme-charts`
- `README.md`, `README.ko.md`, `WIP.md`, `CHANGELOG.md`

## Subagent lane

| lane | 역할 | 최종 결과 | 메모 |
|---|---|---|---|
| Rust/API | `code-reviewer` | `P0=0 P1=0` | Initial P2 default fallback/docs polish를 수정했습니다. |
| Tests | `test-engineer` | `P0=0 P1=0` | No-default feature validation과 benchmark helper test를 추가했습니다. |
| Benchmark | `performance-reviewer` | `P0=0 P1=0` | Fixture manifest, metadata, normalized CSV schema, MiB/s 단위, snapshot caveat를 추가했습니다. |
| Docs/API UX | `library-user-reviewer` | `P0=0 P1=0` | 존재하지 않는 API 예시와 미출시 install guidance를 수정했습니다. |
| Security/Supply chain | `security-reviewer` | `P0=0 P1=0` | Empty compression default, 명시적 root feature forwarding, 로컬 SVG font-file path 부재를 확인했습니다. |
| Final verifier | `verifier` | `P0=0 P1=0` | PR 전 근거 요구 사항을 확인했습니다. 이 artifact와 lesson이 남은 추적 문서 gap을 완료합니다. |

## 로컬 검증 근거

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test -p bluetape-rs-compression --no-default-features --locked`
- `cargo test -p bluetape-rs-compression --all-features --locked`
- `cargo test -p bluetape-rs-compression compiled_algorithms_round_trip_stays_stable_across_threads --all-features --locked`
- `cargo test -p compression-benchmark --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `cargo audit --file Cargo.lock`
- `gzip`, `zlib`, `deflate`, `zstd`, `lz4`, `snappy` single-feature 검사
- `compression`, `compression-*`, `compression-all` root facade 검사
- Normalized `mib_s` 및 `timing_provenance` CSV schema 검사
- 생성된 SVG chart에 `xmllint --noout`
- Throughput 및 ratio chart PNG visual inspection

## Gate 판정

`P0=0 P1=0`

Step 6-R local/native 7-Tier review가 PR 생성 기준을 통과합니다.

## Step 7-R Post-PR 검토

Post-PR lane:

- `code-reviewer`: 처음에 `P0=0 P1=1 P2=1`을 발견했습니다.
- `verifier`: 처음에는 `P0=0 P1=0`이었지만 GitHub PR review/comment 근거가 누락되었습니다.

수정:

- `benchmark/compression-benchmark/go` 아래에 추적되는 Go benchmark harness를 추가했습니다.
- `docs/benchmark/raw/go-same-condition.txt` 아래에 추적되는 raw Go benchmark capture를 추가했습니다.
- CSV normalizer가 이전 임시 raw-output path 대신 repository-local raw Go capture를 읽도록 변경했습니다.
- `docs/benchmark/compression-same-condition-metadata.md`에 fixture 생성과 Go raw-output capture 명령을 문서화했습니다.
- `i32`에 맞지 않는 zstd custom level에 typed `CompressionError::UnsupportedLevel`과 regression test를 추가했습니다.
- `crates/compression/src/lib.rs`를 facade-only export와 전용 `config`, `error`, `traits`, `registry`, `adapters/*` module로 분리했습니다.
- Compression 동작 test를 `lib.rs`에서 `crates/compression/tests/compression.rs`로 이동했습니다.
- Compiled compression algorithm에 bounded multi-thread stress coverage를 추가했습니다.
- `CompressionConfig`를 `#[non_exhaustive]`로 표시하고 `CompressionConfig::new()` 및 `with_level(...)`을 추가했으며, public struct-literal coupling을 피하도록 예시를 갱신했습니다.
- Registry dispatch가 facade re-export에 의존하지 않고 내부 `adapters` module을 사용하도록 변경했습니다.
- `lz4`와 `snappy`가 명시적인 non-default level을 `CompressionError::UnsupportedLevel`로 거부하도록 했습니다.

추가 검증:

- Fixture generator output이 `docs/benchmark/compression-fixtures-manifest.csv`의 12개 모든 행과 일치합니다.
- Go benchmark harness가 예상한 151-line `testing.B` raw output을 생성합니다.
- `cargo test -p bluetape-rs-compression --no-default-features --features zstd --locked`가 통과하고 zstd level rejection을 다룹니다.
- Module/test 분리 후 `cargo test -p bluetape-rs-compression --no-default-features --locked` 및 `cargo test -p bluetape-rs-compression --all-features --locked`가 통과합니다.
- Bounded multi-thread stress coverage를 추가한 후 `cargo test -p bluetape-rs-compression compiled_algorithms_round_trip_stays_stable_across_threads --all-features --locked`가 통과합니다.
- Module/test 분리 후 `cargo clippy -p bluetape-rs-compression --all-targets --all-features --locked -- -D warnings`가 통과합니다.
- API 경계 수정 후 `gzip`, `zlib`, `deflate`, `zstd`, `lz4`, `snappy` single-feature matrix가 통과합니다.

수정 후 Step 7-R blocker 상태: `P0=0 P1=0`
