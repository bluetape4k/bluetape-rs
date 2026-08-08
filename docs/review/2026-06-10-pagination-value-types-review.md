# Pagination value type 검토

## 범위

- 이슈: #33
- Milestone: 0.2.0
- 변경 대상: `bluetape-rs-collections`
- 확인한 참고: `bluetape4k-projects/bluetape4k/core`의 `PaginatedList.kt` 및 `PaginatedListTest.kt`

## 7-Tier 검토

| Tier | 결과 | 근거 |
| --- | --- | --- |
| API contract | 통과 | `Page<T>`는 명시적인 page metadata를 가진 value type이며 DB, SQL, cursor pagination은 범위 밖입니다. |
| Rust 관용성 | 통과 | 음수가 아닌 metadata에 `u64`, typed `PageError`, `Result` constructor, borrowed access에 slice, materialized page에 owned `Vec<T>`를 사용합니다. |
| Error 처리 | 통과 | `page_size == 0`은 `PageError::InvalidPageSize`를 반환하고 음수는 type으로 제외됩니다. |
| 문서 | 통과 | 공개 API에 Rustdoc 예시와 README 사용법이 있습니다. |
| 테스트 | 통과 | Unit test가 default, total-page rounding, zero total, invalid page size, owned item extraction을 다룹니다. |
| 호환성 | 통과 | 새 dependency나 feature flag가 없습니다. |
| 위험 | 낮음 | 새 module과 export만 추가했고 기존 helper는 변경하지 않았습니다. |

## 발견 사항

- P0: 0
- P1: 0
- P2: 0
- P3: 0

## 검증

- 통과: `cargo fmt --all --check`
- 통과: `cargo test -p bluetape-rs-collections`
- 통과: `cargo test --workspace --all-features --locked`
- 통과: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- 통과: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- 통과: `git diff --check`
