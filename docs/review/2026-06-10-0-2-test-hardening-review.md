# 0.2.0 테스트 강화 검토

이슈: #47
브랜치: `feat/issue-47-test-hardening`
기준점: `origin/develop`
날짜: 2026-06-10

## 범위

이 검토는 0.2.0 async 및 collections 테스트 강화 diff를 다룹니다.

- Async cancellation, timeout, shutdown, bounded task-group, join-failure, task-cleanup 동작
- Collections error formatting, iterator helper 경계 사례, `HashMap` ordering contract, page metadata 정책
- 새로 강화한 동작의 Rustdoc 예시와 공개 API contract 주석

## 7-Tier 검토 결과

| Tier | Gate | 결과 | 근거 |
| --- | --- | --- | --- |
| 1 | 범위 및 API contract | PASS | cancellation source drop, join failure index 정책, `HashMap` ordering, page metadata 보존을 공개 contract에 기록했습니다. |
| 2 | Error contract | PASS | `AsyncControlError`, `TaskGroupError`, `CollectionError`, `PageError`의 formatting과 `source()` 동작을 테스트로 검증했습니다. |
| 3 | Async lifecycle | PASS | bounded Tokio test로 cancellation, shutdown, join failure drain, future-drop task cleanup을 검증했습니다. |
| 4 | Stress 및 결정성 | PASS | bounded concurrency stress test가 정확한 peak limit과 완료 수를 검증하고, scheduler 의존 join test는 `Notify`로 동기화했습니다. |
| 5 | 공개 문서 | PASS | Rustdoc이 source-drop semantics, `TaskJoinFailed.index`, `try_map_values` 순서, `chunked_by`, `frequencies`, `group_by`, `Page::with_meta`를 설명합니다. |
| 6 | 로컬 검증 | PASS | Format, whitespace, workspace test, clippy, rustdoc, llvm-cov를 성공적으로 완료했습니다. |
| 7 | Subagent 검토 | PASS | Code review subagent는 P0=0 P1=0을 보고했습니다. Test review subagent의 초기 P1/P2는 모두 수정하고 재검토해 P0=0 P1=0이 되었습니다. |

## 발견 사항

P0=0 P1=0

최종 검토 이후 남은 P0/P1/P2/P3 발견 사항은 없습니다.

Test-review subagent는 처음에 join-failure drain test의 P1 scheduler dependency와
`map_bounded_collect` future-drop cleanup test 누락을 보고했습니다. sibling
startup을 panic 전에 동기화하고 누락된 공개 helper cleanup test를 추가해
수정했습니다.

## Coverage

이전 0.2.0 coverage 보고서의 기준점:

- Workspace: 1379/1585, 87.00%
- `collections`: 349/359, 97.21%
- `async`: 424/481, 88.15%

`coverage/lcov.info`에서 얻은 최종 로컬 coverage:

- Workspace: 1693/1912, 88.55%
- `async`: 708/768, 92.19%
- `collections`: 378/388, 97.42%
- `core`: 178/209, 85.17%
- `logging`: 84/107, 78.50%
- `test`: 345/440, 78.41%

## 검증

실행한 명령:

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test -p bluetape-rs-async --all-features --locked`
- `cargo test -p bluetape-rs-collections --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- `LLVM_COV=/opt/homebrew/Cellar/llvm/22.1.7_1/bin/llvm-cov LLVM_PROFDATA=/opt/homebrew/Cellar/llvm/22.1.7_1/bin/llvm-profdata cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`

## DoD 상태

| 항목 | 상태 | 근거 |
| --- | --- | --- |
| 이슈 #47 범위 구현 | PASS | Async 및 collections test, stress coverage, Rustdoc contract를 강화했습니다. |
| Subagent를 포함한 7-Tier 검토 | PASS | `code-reviewer`와 `test-engineer`가 P0=0 P1=0으로 최종 검토했습니다. |
| P0/P1 blocker 해소 | PASS | 초기 test-review P1을 수정하고 PASS로 재검토했습니다. |
| Coverage 보고서 생성 | PASS | `coverage/lcov.info`에 workspace line coverage 88.55%가 생성되었습니다. |
| Rust 검증 | PASS | fmt, diff check, workspace test, clippy, rustdoc가 통과했습니다. |
