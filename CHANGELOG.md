# 변경 기록

이 프로젝트의 주요 변경 사항을 이 파일에 기록합니다.

## 미공개

아직 미공개 변경 사항은 없습니다.

## [v0.4.0] - 2026-06-11

### 추가

- 추가 기능 플래그 뒤에서 gzip, zlib, deflate, zstd, lz4, snappy 압축기를
  제공하는 선택형 `bluetape-rs-compression` crate를 추가했습니다.
- `bluetape-rs`를 통해 압축 헬퍼를 사용하려는 호출자를 위해 선택형 루트
  crate `compression` facade feature를 추가했습니다.
- 동일 조건의 benchmark fixture 생성, Go raw capture, 그리고 JSON, text,
  binary, random payload에서 `bluetape-rs`, `bluetape-go`,
  `bluetape4k-io` 압축기를 비교하는 재현 가능한 Rust benchmark runner를
  추가했습니다.
- `docs/benchmark`와 `docs/images/readme-charts` 아래에 benchmark CSV,
  Markdown 비교 보고서, chart asset을 추가했습니다.
- 설정을 인식하는 decompression limit, 64 MiB 기본 decode safety limit,
  `Read`/`Write` stream copy helper, `bluetape-rs-compression`용 직접
  stream reader/writer constructor를 추가했습니다.

### 검증

- `cargo metadata --no-deps --format-version 1`
- `cargo fmt --all --check`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `cargo publish --workspace --dry-run --locked`

## [v0.3.1] - 2026-06-10

### 변경

- `0.3.0` 마일스톤 릴리스에서 모든 workspace package를 `0.3.1`로
  맞추어 정정했습니다.
- 루트 facade와 모든 집중형 crate를 하나의 workspace release version으로
  배포했습니다.
  - `bluetape-rs`
  - `bluetape-rs-core`
  - `bluetape-rs-logging`
  - `bluetape-rs-test`
  - `bluetape-rs-collections`
  - `bluetape-rs-async`
  - `bluetape-rs-codec`
- downstream 사용자가 일관된 `0.3.1` version으로 workspace crate에
  의존할 수 있도록 README 설치 예제를 업데이트했습니다.

### 릴리스 정정

- `bluetape-rs-codec@0.3.0`이 이미 crates.io에 배포되었으므로
  `v0.3.0`은 변경하지 않습니다.
- 완전한 `0.3.x` workspace release에는 `v0.3.1`을 사용합니다.

### 검증

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `cargo publish --workspace --dry-run --locked`

## [v0.3.0] - 2026-06-10

완전한 workspace release에서는 `v0.3.1`이 이를 대체합니다.

### 추가

- strict hex, Base64 standard, Base64 URL-safe, Bitcoin Base58, byte-oriented
  Base62, UTF-8 text boundary helper를 제공하는 `bluetape-rs-codec`
  `0.3.0`을 추가했습니다.
- `bluetape-rs`를 통해 codec helper를 사용하려는 호출자를 위해 선택형
  루트 crate `codec` facade feature를 추가했습니다.
- 호출자가 소유한 잘못된 입력에 대해 위치 정보가 포함된 hex와 base-N
  실패, 손실 없는 UTF-8 text 실패를 포함하는 typed decode error를
  추가했습니다.
- hex, Base64, Base58, Base62, UTF-8 text helper를 대상으로 public
  crate-boundary integration test를 추가했습니다.

### 변경

- private shared base-N 구현의 source-local test는 유지하면서 public codec
  test를 `crates/codec/tests/` 아래로 분리했습니다.
- compression은 `0.4.0`, serde 기반 serialization은 `0.5.0`으로
  계속 연기됨을 확인했습니다.

### 검증

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test -p bluetape-rs-codec --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`
- `cargo publish -p bluetape-rs-codec --dry-run --locked`

## [v0.2.0] - 2026-06-10

### 추가

- 집중형 iterator, slice, map, pagination, grouping, chunking, error-aware
  transform helper를 제공하는 `bluetape-rs-collections` `0.2.0`을
  추가했습니다.
- Tokio 우선 bounded task execution, cancellation, timeout, deadline,
  shutdown coordination helper를 제공하는 `bluetape-rs-async` `0.2.0`을
  추가했습니다.
- cancellation, dropped future, join failure, bounded execution, collection
  helper 경계를 검증하는 결정적 async 및 concurrency test를 추가했습니다.
- 현재 collections 및 async/concurrency release train을 위한 README
  architecture diagram과 crate-level usage example을 추가했습니다.

### 변경

- 향후 crate 작업이 처음부터 Rust module convention을 따르도록 큰
  `lib.rs` 파일에서 implementation module을 분리했습니다.
- pull-request check, coverage reporting, clippy, rustdoc, nightly workflow
  coverage를 추가하여 CI를 강화했습니다.

### 검증

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- `cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`
- `cargo publish --workspace --dry-run --locked`

## [v0.1.1] - 2026-06-10

### 변경

- `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-test`의 기반
  public API Rustdoc을 보강했습니다.
- validation, logging, async assertion, concurrency, temporary directory
  helper에 대해 compile-checked example과 명시적 error contract를
  추가했습니다.
- 향후 contributor와 agent를 위한 repository-local Rust ecosystem
  convention 지침을 문서화했습니다.

### 검증

- `cargo fmt --all --check`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo doc --workspace --no-deps`
