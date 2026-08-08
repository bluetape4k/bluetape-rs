# Slice 및 List 경계 헬퍼 검토

## 범위

- 이슈: #32
- 기준선: `bea771f`의 `origin/develop`
- 검토 diff: `crates/collections/src/slice.rs`, README, research update

## 근거

- `bluetape4k-core` source 검토: `CollectionSupport.kt`의 `safeSubList`, `padTo`,
  prepend/append 및 swap helper
- Rust std 중복 검토: `Vec::swap`, `Vec::extend`, `Vec::insert`, `Vec::resize`,
  slice `chunks` 및 slice `windows`는 의도적으로 래핑하지 않음
- 채택한 API 근거: `clamped_subslice`는 할당 없이 signed external bound를
  처리하고, `pad_to`는 padding이 필요 없을 때 할당을 피하려고 `Cow<[T]>`를
  사용한다.
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test -p bluetape-rs-collections`: PASS, 22 unit tests and 12 doctests
- `cargo test --workspace --all-features`: PASS
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: PASS

## 발견 사항

- P0: 없음
- P1: 없음
- P2: 없음
- P3: 없음

## 7-Tier 검토

| 단계 | 결과 | 근거 |
| --- | --- | --- |
| API 계약 | PASS | Rust 네이티브 빌린 slice와 `Cow` API를 사용하며 mutable Vec wrapper 헬퍼가 없다. |
| 정확성 | PASS | clamp된 음수, 초과, 역순, 빈 경계를 커버한다. |
| 할당 동작 | PASS | `pad_to`가 빌린 no-op 결과와 소유 패딩 결과를 반환한다. |
| 표준 라이브러리 중복 | PASS | `Vec` 및 slice 내장 기능의 직접 wrapper를 제외했다. |
| 문서 | PASS | Rustdoc 예제가 doctest로 통과한다. |
| 범위 통제 | PASS | 페이지 매김은 #33으로 계속 분리한다. |
| 검증 | PASS | fmt, 테스트, clippy, rustdoc 경고 게이트가 통과했다. |

## 게이트

P0=0 P1=0

판정: PASS.
