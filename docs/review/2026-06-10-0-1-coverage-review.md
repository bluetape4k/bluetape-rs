# 0.1.x coverage 강화 검토

이슈: #49
브랜치: `feat/issue-49-coverage-0-1`
기준점: `origin/develop`
날짜: 2026-06-10

## 범위

이 검토는 0.1.0 및 0.1.1 foundation 범위의 테스트 전용 coverage 강화를
다룹니다.

- `bluetape-rs-core` validation error 표시 및 `source()` contract
- `bluetape-rs-collections` collection error 표시 및 `source()` contract
- `bluetape-rs-logging` correlation-id 거부 및 error 표시 contract
- `bluetape-rs-test` async assertion과 concurrent tester의 error 표시, source, 잘못된 configuration 경로

## Coverage

이 브랜치 전 `develop` 기준점:

| Crate | Covered / Total | Line coverage |
| --- | ---: | ---: |
| `logging` | 84 / 107 | 78.50% |
| `test` | 345 / 440 | 78.41% |

`coverage/lcov.info`에서 얻은 최종 로컬 coverage:

| Crate | Covered / Total | Line coverage |
| --- | ---: | ---: |
| `async` | 463 / 534 | 86.70% |
| `collections` | 355 / 370 | 95.95% |
| `core` | 203 / 209 | 97.13% |
| `logging` | 95 / 107 | 88.79% |
| `test` | 413 / 440 | 93.86% |

결과: 80% line coverage 미만인 crate가 없습니다. 이전에 coverage가 낮았던
`error.rs` 파일은 공개 error contract에 집중한 테스트로 보강했습니다.

## 7-Tier 검토 결과

| Tier | Gate | 결과 | 근거 |
| --- | --- | --- | --- |
| 1 | 범위 | PASS | 변경은 0.1.x foundation crate의 테스트 coverage와 검토 근거로 제한됩니다. |
| 2 | Error contract | PASS | validation, collection, correlation, async assertion, concurrent assertion error의 공개 `Display` 및 `source()` 동작을 검증했습니다. |
| 3 | 경계 coverage | PASS | 빈/과도하게 긴/unsafe correlation ID, 잘못된 tester 경계, source가 없는 error variant를 검증했습니다. |
| 4 | Async/concurrency 동작 | PASS | 기존 panic/join 경로가 `WorkerJoinFailed` 표시와 source forwarding을 검증합니다. |
| 5 | Coverage 목표 | PASS | `logging`과 `test`가 모두 80%를 넘고, 모든 crate가 80%를 넘습니다. |
| 6 | 로컬 검증 | PASS | fmt, diff check, workspace test, clippy, rustdoc, llvm-cov를 성공적으로 완료했습니다. |
| 7 | Subagent 검토 | PASS | 추적되는 검토 artifact와 `WorkerJoinFailed` contract coverage를 추가한 뒤 재검토 결과 P0=0 P1=0이었습니다. |

## 검증

실행한 명령:

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test -p bluetape-rs-logging --all-features --locked`
- `cargo test -p bluetape-rs-test --all-features --locked`
- `cargo test -p bluetape-rs-core --all-features --locked`
- `cargo test -p bluetape-rs-collections --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- `LLVM_COV=/opt/homebrew/Cellar/llvm/22.1.7_1/bin/llvm-cov LLVM_PROFDATA=/opt/homebrew/Cellar/llvm/22.1.7_1/bin/llvm-profdata cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`

## DoD 상태

| 항목 | 상태 | 근거 |
| --- | --- | --- |
| 80% crate coverage gate | PASS | 80% 미만 crate가 없으며 `logging` 88.79%, `test` 93.86%입니다. |
| 낮은 `error.rs` coverage 보완 | PASS | `core`와 `collections`의 공개 error 표시/source 테스트를 추가했습니다. |
| 0.1.x 공개 error contract | PASS | core, collections, logging, test helper의 error 표시/source contract를 검증했습니다. |
| 로컬 검증 | PASS | fmt, diff check, workspace test, clippy, rustdoc, llvm-cov가 로컬에서 통과했습니다. |
| 7-Tier subagent 검토 | PASS | 재검토에서 P0=0 P1=0을 보고했고, 남은 P3는 이 artifact를 commit에 포함해 해소했습니다. |
