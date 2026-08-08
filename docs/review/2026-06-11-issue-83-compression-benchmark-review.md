# 이슈 #83 Step 6-R 검토

범위: `develop` 대비 `issue-83-compression-benchmark-report`.

기준선: `90a937f5f422d020253749e6188eac2f2a623a1a`.

## 검토 표면

- `crates/compression`
- 루트 파사드 `compression*` feature
- `benchmark/compression-benchmark`
- `docs/benchmark`
- `docs/images/readme-charts`
- `README.md`, `README.ko.md`, `WIP.md`, `CHANGELOG.md`

## 서브에이전트 레인

| 레인 | 역할 | 최종 결과 | 비고 |
|---|---|---|---|
| Rust/API | `code-reviewer` | `P0=0 P1=0` | 초기 P2 기본 fallback/문서 다듬기를 수정했다. |
| 테스트 | `test-engineer` | `P0=0 P1=0` | 기본 feature를 끈 검증과 벤치마크 헬퍼 테스트를 추가했다. |
| 벤치마크 | `performance-reviewer` | `P0=0 P1=0` | fixture manifest, 메타데이터, 정규화 CSV 스키마, MiB/s 단위, snapshot 주의 사항을 추가했다. |
| 문서/API UX | `library-user-reviewer` | `P0=0 P1=0` | 존재하지 않는 API 예제와 미게시 설치 지침을 수정했다. |
| 보안/공급망 | `security-reviewer` | `P0=0 P1=0` | 빈 압축 기본값, 명시적인 루트 feature 전달, 로컬 SVG font-file 경로 없음을 확인했다. |
| 최종 검증 | `verifier` | `P0=0 P1=0` | PR 전 근거 요구 사항을 확인했다. 이 산출물과 lesson으로 추적 문서 공백을 해소했다. |

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
- `gzip`, `zlib`, `deflate`, `zstd`, `lz4`, `snappy` 단일 feature 검사
- `compression`, `compression-*`, `compression-all` 루트 파사드 검사
- 정규화된 `mib_s` 및 `timing_provenance`의 CSV 스키마 검사
- `xmllint --noout` for generated SVG charts
- 처리량 및 비율 차트의 PNG 시각 검사

## 게이트 판정

`P0=0 P1=0`

Step 6-R 로컬/네이티브 7-Tier 검토가 PR 생성 기준으로 통과했다.

## Step 7-R PR 후 검토

PR 후 레인:

- `code-reviewer`: 초기에는 `P0=0 P1=1 P2=1`을 발견했다.
- `verifier`: 초기에는 `P0=0 P1=0`이었지만 GitHub PR 검토/댓글 근거가
  누락되었다.

수정 사항:

- `benchmark/compression-benchmark/go` 아래에 추적되는 Go benchmark harness를
  추가했다.
- `docs/benchmark/raw/go-same-condition.txt` 아래에 추적되는 raw Go benchmark
  capture를 추가했다.
- CSV normalizer가 이전 임시 raw-output 경로 대신 저장소 로컬 raw Go capture를
  읽도록 변경했다.
- `docs/benchmark/compression-same-condition-metadata.md`에 fixture 생성 및 Go
  raw-output capture 명령을 문서화했다.
- `i32`에 들어가지 않는 zstd custom level을 위한 타입이 지정된
  `CompressionError::UnsupportedLevel`과 regression test를 추가했다.
- `crates/compression/src/lib.rs`를 파사드 전용 export와 전용 `config`,
  `error`, `traits`, `registry`, `adapters/*` 모듈로 분리했다.
- 압축 동작 테스트를 `lib.rs`에서 `crates/compression/tests/compression.rs`로
  옮겼다.
- 컴파일된 compression algorithm을 위한 제한된 multi-thread stress 범위를
  추가했다.
- `CompressionConfig`를 `#[non_exhaustive]`로 지정하고
  `CompressionConfig::new()` 및 `with_level(...)`을 추가했으며, 공개 구조체
  리터럴 결합을 피하도록 예제를 갱신했다.
- registry dispatch가 파사드 재내보내기에 의존하지 않고 내부 `adapters` 모듈을
  사용하도록 변경했다.
- `lz4` 및 `snappy`가 명시적인 non-default level을
  `CompressionError::UnsupportedLevel`로 거부하도록 했다.

추가 검증:

- Fixture generator 출력이 `docs/benchmark/compression-fixtures-manifest.csv`의
  12개 행과 일치한다.
- Go benchmark harness가 예상한 151줄 `testing.B` raw output을 생성한다.
- `cargo test -p bluetape-rs-compression --no-default-features --features zstd --locked`가
  통과하며 zstd level 거부를 포함한다.
- 모듈/테스트를 분리한 뒤 `cargo test -p bluetape-rs-compression --no-default-features --locked`와
  `cargo test -p bluetape-rs-compression --all-features --locked`가 통과한다.
- 제한된 multi-thread stress 범위를 추가한 뒤
  `cargo test -p bluetape-rs-compression compiled_algorithms_round_trip_stays_stable_across_threads --all-features --locked`가 통과한다.
- 모듈/테스트를 분리한 뒤
  `cargo clippy -p bluetape-rs-compression --all-targets --all-features --locked -- -D warnings`가
  통과한다.
- API 경계를 수정한 뒤 `gzip`, `zlib`, `deflate`, `zstd`, `lz4`, `snappy`의
  단일 feature matrix가 통과한다.

수정 후 Step 7-R 차단 상태: `P0=0 P1=0`.
