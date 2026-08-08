# 페이지 매김 값 타입 검토

## 범위

- 이슈: #33
- 마일스톤: 0.2.0
- 변경 표면: `bluetape-rs-collections`
- 확인한 참조: 다음의 `PaginatedList.kt`, `PaginatedListTest.kt`
  `bluetape4k-projects/bluetape4k/core`

## 7-Tier 검토

| 단계 | 결과 | 근거 |
| --- | --- | --- |
| API 계약 | Pass | `Page<T>`가 명시적인 페이지 메타데이터를 갖는 값 타입이며 DB, SQL, cursor 페이지 매김은 범위 밖이다. |
| Rust 관용 | Pass | 음수가 아닌 메타데이터에 `u64`, 타입 지정 `PageError`, `Result` 생성자, 빌린 접근에 slice, 실체화한 페이지에 소유 `Vec<T>`를 사용한다. |
| 오류 처리 | Pass | `page_size == 0`은 `PageError::InvalidPageSize`를 반환하며 음수 값은 타입으로 제외한다. |
| 문서 | Pass | 공개 API에 Rustdoc 예제와 README 사용법이 있다. |
| 테스트 | Pass | 단위 테스트가 기본값, 전체 페이지 반올림, 0 합계, 잘못된 페이지 크기, 소유 항목 추출을 커버한다. |
| 호환성 | Pass | 새 의존성이나 feature flag가 없다. |
| 위험 | 낮음 | 새 모듈과 export만 추가하고 기존 헬퍼는 변경하지 않는다. |

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
