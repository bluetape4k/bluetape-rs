# 0.2.0 테스트 강화 검토

이슈: #47
브랜치: `feat/issue-47-test-hardening`
기준선: `origin/develop`
날짜: 2026-06-10

## 범위

이 검토는 0.2.0 비동기 및 컬렉션 테스트 강화 diff를 다룬다.

- 비동기 취소, 타임아웃, 종료, 제한된 task-group, join-failure 및
  task-cleanup 동작.
- 컬렉션 오류 형식 지정, iterator helper edge case, `HashMap` 순서
  계약 및 page metadata policy.
- 새로 강화한 동작을 위한 Rustdoc 예제와 공개 API 계약 주석.

## 7-Tier 검토 결과

| 단계 | 게이트 | 결과 | 근거 |
| --- | --- | --- | --- |
| 1 | 범위 및 API 계약 | PASS | 취소 소스 drop, join 실패 인덱스 정책, `HashMap` 순서, 페이지 메타데이터 보존에 대한 공개 계약을 문서화했다. |
| 2 | 오류 계약 | PASS | `AsyncControlError`, `TaskGroupError`, `CollectionError`, `PageError`의 형식 지정과 `source()` 동작을 테스트로 커버했다. |
| 3 | 비동기 수명 주기 | PASS | 제한된 Tokio 테스트로 취소, 종료, join 실패 drain, future-drop 작업 정리를 커버했다. |
| 4 | 스트레스 및 결정성 | PASS | 제한된 동시성 스트레스 테스트가 정확한 최대치와 완료 수를 검증하며, 스케줄러 의존 join 테스트는 `Notify`로 동기화했다. |
| 5 | 공개 문서 | PASS | Rustdoc에서 source-drop 의미론, `TaskJoinFailed.index`, `try_map_values` 순서, `chunked_by`, `frequencies`, `group_by`, `Page::with_meta`를 명확히 했다. |
| 6 | 로컬 검증 | PASS | 포맷, 공백, 워크스페이스 테스트, clippy, rustdoc, llvm-cov를 성공적으로 완료했다. |
| 7 | 서브에이전트 검토 | PASS | 코드 검토 서브에이전트: P0=0 P1=0. 테스트 검토 서브에이전트: 초기 P1/P2를 모두 수정하고 재검토에서 P0=0 P1=0으로 확인했다. |

## 발견 사항

P0=0 P1=0

최종 검토 후 남은 P0/P1/P2/P3 발견 사항은 없다.

테스트 검토 서브에이전트는 처음에 join-failure drain 테스트의 P1 스케줄러
의존성과 P2 `map_bounded_collect` future-drop 정리 테스트 누락을 보고했다.
수정에서는 panic 전에 형제 작업의 시작을 동기화하고 누락된 공개 헬퍼 정리
테스트를 추가했다.

## 커버리지

이전 0.2.0 커버리지 보고서의 기준선:

- 워크스페이스: 1379/1585, 87.00%
- `collections`: 349/359, 97.21%
- `async`: 424/481, 88.15%

`coverage/lcov.info`의 최종 로컬 커버리지:

- 워크스페이스: 1693/1912, 88.55%
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
| 이슈 #47 범위 구현 | PASS | 비동기 및 컬렉션 테스트, 스트레스 커버리지, Rustdoc 계약을 강화했다. |
| 서브에이전트와 7-Tier 검토 | PASS | `code-reviewer`와 `test-engineer` 서브에이전트가 P0=0 P1=0으로 최종 검토를 완료했다. |
| P0/P1 차단 요소 해결 | PASS | 초기 테스트 검토 P1을 수정하고 PASS로 재검토했다. |
| 커버리지 보고서 생성 | PASS | `coverage/lcov.info`, 워크스페이스 라인 커버리지 88.55%. |
| Rust 검증 | PASS | fmt, diff 검사, 워크스페이스 테스트, clippy, rustdoc가 통과했다. |
