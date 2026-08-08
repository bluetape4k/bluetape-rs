# 모듈 분할 검토

## 범위

- 이슈: #16
- 브랜치: `refactor/module-split`
- 기준선: `origin/develop`
- 변경 유형: 초기 기반 크레이트의 동작 보존 모듈 분할

## 검토한 변경

- `bluetape-rs-core`: `lib.rs`를 `error`, `number`, `string`, `hex`, `tests` 모듈로
  분할.
- `bluetape-rs-logging`: `lib.rs`를 `correlation`, `capture`, `subscriber`, `tests`
  모듈로 분할.
- `bluetape-rs-test`: `lib.rs`를 `async_assert`, `concurrent`, `temp_dir`, `tests`
  모듈로 분할.
- 루트 공개 API는 `pub use` 재내보내기를 통해 계속 사용할 수 있다.

## 7-Tier 검토 요약

- 단계 1 API 호환성: PASS. 기존 공개 이름을 크레이트 루트에서 계속 재내보낸다.
- 단계 2 동작 보존: PASS. 분할 후 기존 단위 테스트와 doctest가 통과한다.
- 단계 3 Rust 모듈 경계: PASS. `lib.rs` 파일이 이제 크레이트 진입점과 모듈 지도로 동작한다.
- 단계 4 테스트: PASS. `cargo test --workspace`가 통과했다.
- 단계 5 lint/정적 검사: PASS. `cargo clippy --workspace --all-targets --all-features -- -D warnings`가 통과했다.
- 단계 6 문서: PASS. 이동한 공개 항목에서 검토자가 찾은 Rustdoc 누락을 수정했다.
- 단계 7 검토자 게이트: PASS. 네이티브 `code-reviewer`가 `P0=0 P1=0`을 보고했고 P2 Rustdoc 이슈 하나를 커밋 전에 수정했다.

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

이 리팩터링은 의도적으로 구조만 변경한다. 새 동작, 새 테스트, 새 공개 API
이름을 추가하지 않는다. 잔여 위험은 실수로 재 export가 달라지는 경우로
한정되며 컴파일, doctest, 기존 공개 API 테스트로 커버한다.
