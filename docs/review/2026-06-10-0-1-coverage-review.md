# 0.1.x 커버리지 강화 검토

이슈: #49
브랜치: `feat/issue-49-coverage-0-1`
기준선: `origin/develop`
날짜: 2026-06-10

## 범위

이 검토는 0.1.0 및 0.1.1 기반 범위에서 테스트만 변경한 커버리지 강화를 다룬다.

- `bluetape-rs-core` validation error display 및 `source()` 계약.
- `bluetape-rs-collections` collection error display 및 `source()` 계약.
- `bluetape-rs-logging` correlation-id 거부 및 error display 계약.
- `bluetape-rs-test` 비동기 assertion과 동시성 테스터의 오류 표시, source 및
  잘못된 configuration branch.

## 커버리지

이 브랜치 작업 전 `develop`의 기준선:

| 크레이트 | 커버됨 / 전체 | 라인 커버리지 |
| --- | ---: | ---: |
| `logging` | 84 / 107 | 78.50% |
| `test` | 345 / 440 | 78.41% |

`coverage/lcov.info`의 최종 로컬 커버리지:

| 크레이트 | 커버됨 / 전체 | 라인 커버리지 |
| --- | ---: | ---: |
| `async` | 463 / 534 | 86.70% |
| `collections` | 355 / 370 | 95.95% |
| `core` | 203 / 209 | 97.13% |
| `logging` | 95 / 107 | 88.79% |
| `test` | 413 / 440 | 93.86% |

결과: 라인 커버리지가 80% 미만인 크레이트가 없다. 이전에 커버리지가 낮았던
`error.rs` 파일은 이제 공개 오류 계약에 집중한 테스트로 커버된다.

## 7-Tier 검토 결과

| 단계 | 게이트 | 결과 | 근거 |
| --- | --- | --- | --- |
| 1 | 범위 | PASS | 변경은 0.1.x 기반 크레이트의 테스트 커버리지와 검토 근거로 한정된다. |
| 2 | 오류 계약 | PASS | 검증, 컬렉션, 상관관계, 비동기 assertion, 동시성 assertion 오류의 공개 `Display` 및 `source()` 동작을 커버한다. |
| 3 | 경계 커버리지 | PASS | 공백/과도하게 긴/안전하지 않은 상관관계 ID, 잘못된 tester 경계, 소스가 없는 오류 변형을 커버한다. |
| 4 | 비동기/동시성 동작 | PASS | 기존 panic/join 경로에서 이제 `WorkerJoinFailed` 표시와 소스 전달을 검증한다. |
| 5 | 커버리지 목표 | PASS | `logging`과 `test`가 모두 80%를 넘고 모든 크레이트가 80%를 넘는다. |
| 6 | 로컬 검증 | PASS | fmt, diff 검사, 워크스페이스 테스트, clippy, rustdoc, llvm-cov를 성공적으로 완료했다. |
| 7 | 서브에이전트 검토 | PASS | 추적하는 검토 산출물과 `WorkerJoinFailed` 계약 커버리지를 추가한 뒤 재검토에서 P0=0 P1=0을 보고했다. |

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
| 80% 크레이트 커버리지 게이트 | PASS | 80% 미만 크레이트 없음; `logging` 88.79%, `test` 93.86%. |
| 낮은 `error.rs` 커버리지 해결 | PASS | `core`와 `collections`에 공개 오류 표시/소스 테스트를 추가했다. |
| 0.1.x 공개 오류 계약 | PASS | core, collections, logging, test 헬퍼의 오류 표시/소스 계약을 커버했다. |
| 로컬 검증 | PASS | fmt, diff 검사, 워크스페이스 테스트, clippy, rustdoc, llvm-cov가 로컬에서 통과했다. |
| 7-Tier 서브에이전트 검토 | PASS | 재검토에서 P0=0 P1=0을 보고했으며, 남은 P3는 이 산출물을 커밋에 포함해 충족했다. |
