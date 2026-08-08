# 모듈 분리 검토

## 범위

- 이슈: #16
- 브랜치: `refactor/module-split`
- 기준점: `origin/develop`
- 변경 유형: 초기 foundation crate를 동작을 보존하며 분리

## 검토한 변경

- `bluetape-rs-core`: `lib.rs`를 `error`, `number`, `string`, `hex`, `tests` 모듈로 분리
- `bluetape-rs-logging`: `lib.rs`를 `correlation`, `capture`, `subscriber`, `tests` 모듈로 분리
- `bluetape-rs-test`: `lib.rs`를 `async_assert`, `concurrent`, `temp_dir`, `tests` 모듈로 분리
- Root 공개 API는 `pub use` re-export를 통해 계속 사용할 수 있습니다.

## 7-Tier 검토 요약

- Tier 1 API 호환성: PASS. 기존 공개 이름은 crate root에서 계속 re-export됩니다.
- Tier 2 동작 보존: PASS. 분리 후 기존 unit 및 doctest가 통과합니다.
- Tier 3 Rust 모듈 경계: PASS. `lib.rs`가 이제 crate entrypoint와 module map 역할을 합니다.
- Tier 4 테스트: PASS. `cargo test --workspace`가 통과했습니다.
- Tier 5 lint/static 검사: PASS. `cargo clippy --workspace --all-targets --all-features -- -D warnings`가 통과했습니다.
- Tier 6 문서: PASS. 이동한 공개 항목에서 reviewer가 지적한 Rustdoc 누락을 수정했습니다.
- Tier 7 reviewer gate: PASS. Native `code-reviewer`가 `P0=0 P1=0`을 보고했고, Rustdoc P2 하나를 commit 전에 수정했습니다.

## 발견 사항

- P0: 0
- P1: 0
- P2: Rustdoc 복원 후 0
- P3: 0

## 검증

- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test --workspace`: PASS
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS
- `cargo doc --workspace --no-deps`: PASS

## 잔여 위험

이 refactor는 의도적으로 구조만 변경합니다. 새 동작, 새 테스트, 새 공개
API 이름을 추가하지 않았습니다. 잔여 위험은 실수로 re-export가 달라지는
것이며, compilation, doctest, 기존 공개 API test가 이를 다룹니다.
