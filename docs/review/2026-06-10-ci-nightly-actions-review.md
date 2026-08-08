# CI 및 Nightly Actions 검토

## 범위

- 이슈: #36
- 기준점: `origin/develop`의 `926ca2c`
- 검토 diff: `.github/workflows/ci.yml`, `.github/workflows/nightly-tests.yml`

## 참고 근거

- `bluetape-go/.github/workflows/ci.yml`: compact CI, `develop`/`main`, concurrency, checkout/setup/cache/test 흐름
- `bluetape-go/.github/workflows/nightly-tests.yml`: scheduled smoke/full 계획, workflow dispatch 범위, 무거운 test 재시도
- `bluetape4k-projects/.github/workflows/ci.yml`: permission, paths-ignore, concurrency, workflow dispatch, 분리된 validation job
- `exposed-workshop/.github/workflows/ci.yml`: paths-ignore와 validation/test aggregation을 사용하는 더 작은 repository CI 형태
- `actionlint .github/workflows/ci.yml .github/workflows/nightly-tests.yml`: PASS
- `cargo fmt --all --check`: PASS
- `cargo check --workspace --all-targets --all-features --locked`: PASS, 기본 Rust 1.96 및 MSRV 1.85.0
- `cargo test --workspace --all-features --locked`: PASS, 기본 Rust 1.96 및 MSRV 1.85.0
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS, 기본 Rust 1.96 및 MSRV 1.85.0
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked`: PASS, 기본 Rust 1.96 및 MSRV 1.85.0
- `git diff --check`: PASS

## 발견 사항

- P0: 없음
- P1: 없음
- P2: 없음
- P3: 없음

## 7-Tier 검토

| Tier | 결과 | 근거 |
| --- | --- | --- |
| Trigger contract | PASS | `develop` 및 `main`에 대한 `push`/`pull_request`와 manual dispatch가 활성화되어 있습니다. |
| Permission 경계 | PASS | `contents: read`만 사용합니다. |
| Rust toolchain | PASS | Workspace MSRV와 일치하는 `RUST_VERSION=1.85.0`입니다. |
| Validation coverage | PASS | fmt, check, test, clippy, rustdoc warning을 검증합니다. |
| Nightly 형태 | PASS | smoke/full/docs 범위와 매일 smoke, 매주 full schedule이 있습니다. |
| Dependency surface | PASS | 공식 checkout/cache action과 runner `rustup`만 사용하며 별도 Rust setup action은 없습니다. |
| 로컬 검증 | PASS | actionlint, fmt, check, test, clippy, rustdoc, diff whitespace를 실행했습니다. |

## Gate

P0=0 P1=0

판정: PASS
