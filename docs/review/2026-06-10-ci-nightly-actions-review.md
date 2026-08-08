# CI 및 nightly actions 검토

## 범위

- 이슈: #36
- 기준선: `926ca2c`의 `origin/develop`
- 검토 diff: `.github/workflows/ci.yml`, `.github/workflows/nightly-tests.yml`

## 참조 근거

- `bluetape-go/.github/workflows/ci.yml`: 간결한 CI, `develop`/`main`, 동시성,
  checkout/setup/cache/test 흐름.
- `bluetape-go/.github/workflows/nightly-tests.yml`: 예약된 smoke/full 계획,
  workflow dispatch 범위 및 무거운 테스트 재시도.
- `bluetape4k-projects/.github/workflows/ci.yml`: 권한, paths-ignore,
  동시성, workflow dispatch 및 별도 검증 작업.
- `exposed-workshop/.github/workflows/ci.yml`: paths-ignore와 validation/test
  aggregation을 사용하는 작은 저장소 CI 형태.
- `actionlint .github/workflows/ci.yml .github/workflows/nightly-tests.yml`: PASS
- `cargo fmt --all --check`: PASS
- `cargo check --workspace --all-targets --all-features --locked`: 기본 Rust 1.96 및
  MSRV 1.85.0에서 PASS
- `cargo test --workspace --all-features --locked`: 기본 Rust 1.96 및 MSRV 1.85.0에서 PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  기본 Rust 1.96 및 MSRV 1.85.0에서 PASS
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked`: 기본 Rust
  1.96 및 MSRV 1.85.0에서 PASS
- `git diff --check`: PASS

## 발견 사항

- P0: 없음
- P1: 없음
- P2: 없음
- P3: 없음

## 7-Tier 검토

| 단계 | 결과 | 근거 |
| --- | --- | --- |
| 트리거 계약 | PASS | `develop` 및 `main`의 `push`/`pull_request`, 수동 dispatch 활성화 |
| 권한 경계 | PASS | `contents: read`만 사용 |
| Rust 툴체인 | PASS | `RUST_VERSION=1.85.0`, 워크스페이스 MSRV와 일치 |
| 검증 커버리지 | PASS | fmt, check, test, clippy, rustdoc 경고 검사 |
| nightly 형태 | PASS | 매일 smoke와 매주 full을 예약한 smoke/full/docs 범위 |
| 의존성 표면 | PASS | 공식 checkout/cache actions와 runner `rustup`; 추가 Rust setup action 없음 |
| 로컬 검증 | PASS | actionlint, fmt, check, test, clippy, rustdoc, diff 공백 검사 |

## 게이트

P0=0 P1=0

판정: PASS.
