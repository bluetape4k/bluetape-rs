# Collection helper 검토

## 범위

- 이슈: #20
- 기준점: `origin/develop`의 `2998bc9`
- 검토 diff: `crates/collections` helper 구현과 research note

## 근거

- `bluetape4k-core` source 검토: `CollectionSupport.kt`, `IterableSupport.kt`, `SequenceSupport.kt`, `ListSupport.kt`, `IteratorSupport.kt`, `MapEntrySupport.kt`, `PaginatedList.kt`
- Rust API 검토: Rust 1.85 표준 iterator/slice/map API가 직접 다루지 않는 gap으로 helper를 제한
- 후속 분리: slice/list 경계 helper는 #32, pagination value type은 #33
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test -p bluetape-rs-collections`: PASS, unit test 15개 및 doctest 10개
- `cargo test --workspace --all-features`: PASS
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: PASS

## 발견 사항

- P0: 없음
- P1: 없음
- P2: 없음
- P3: 없음

## 7-Tier 검토

| Tier | 결과 | 근거 |
| --- | --- | --- |
| API contract | PASS | Rust-native free function과 명시적인 `Result`를 사용하며 Kotlin extension 형태를 기계적으로 포팅하지 않았습니다. |
| 정확성 | PASS | chunk/window/group/frequency/map/result 동작을 테스트로 다뤘습니다. |
| Error 처리 | PASS | 잘못된 size는 `CollectionError::InvalidSize`를 반환하고 fallible map은 호출자의 error type을 보존합니다. |
| 표준 library 중복 | PASS | slice `chunks/windows`, range collect, `Vec` mutation, primitive conversion wrapper는 제외했습니다. |
| 문서 | PASS | Rustdoc 예시가 doctest로 통과합니다. |
| 범위 통제 | PASS | 더 큰 pagination 및 slice/list 정책 helper를 #32와 #33으로 분리했습니다. |
| 검증 | PASS | fmt, test, clippy, rustdoc warning gate가 통과했습니다. |

## Gate

P0=0 P1=0

판정: PASS. API는 Rust-native이고 직접적인 Kotlin/JVM 포팅을 피하며,
allocation/error 동작을 문서화하고 success, grouping, empty/boundary,
error-path test를 포함합니다.
