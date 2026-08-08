# Slice 및 List 경계 helper 검토

## 범위

- 이슈: #32
- 기준점: `origin/develop`의 `bea771f`
- 검토 diff: `crates/collections/src/slice.rs`, README, research update

## 근거

- `bluetape4k-core` source 검토: `CollectionSupport.kt`의 `safeSubList`, `padTo`, prepend/append, swap helper
- Rust std 중복 검토: `Vec::swap`, `Vec::extend`, `Vec::insert`, `Vec::resize`, slice `chunks`, slice `windows`를 의도적으로 wrapper로 만들지 않음
- 채택한 API 근거: `clamped_subslice`는 allocation 없이 signed external bound를 처리하고 `pad_to`는 padding이 필요 없을 때 allocation을 피하려고 `Cow<[T]>`를 사용
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test -p bluetape-rs-collections`: PASS, unit test 22개 및 doctest 12개
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
| API contract | PASS | Rust-native borrowed slice와 `Cow` API를 사용하며 mutable Vec wrapper helper는 없습니다. |
| 정확성 | PASS | clamped negative, oversized, reversed, empty bound를 다룹니다. |
| Allocation 동작 | PASS | `pad_to`가 no-op에서는 borrowed result, padding에서는 owned result를 반환합니다. |
| 표준 library 중복 | PASS | `Vec`와 slice 내장 기능의 직접 wrapper는 제외했습니다. |
| 문서 | PASS | Rustdoc 예시가 doctest로 통과합니다. |
| 범위 통제 | PASS | Pagination은 #33으로 분리되어 있습니다. |
| 검증 | PASS | fmt, test, clippy, rustdoc warning gate가 통과했습니다. |

## Gate

P0=0 P1=0

판정: PASS
