# Tokio task group 검토

## 범위

- 이슈: #21
- Milestone: 0.2.0
- 변경 대상: 새 `bluetape-rs-async` crate 및 root facade feature
- 외부 참고: `JoinSet`, `abort_all`, abort 후 `join_next`를 drain하는 shutdown에 대한 Tokio 1.49 문서

## 7-Tier 검토

| Tier | 결과 | 근거 |
| --- | --- | --- |
| API contract | 통과 | `try_map_bounded`가 first-error abort/drain 동작을 정의하고 `map_bounded_collect`가 전체 operation result 수집 동작을 정의합니다. |
| Rust 관용성 | 통과 | `Result`, typed error, `JoinSet`, `Send + 'static` task 경계, 순서가 있는 value result, 공개 Rustdoc 예시를 사용합니다. |
| Cancellation 동작 | 통과 | 첫 operation error와 Tokio join failure에서 `abort_all`을 호출하고 남은 task를 drain합니다. |
| Bound | 통과 | 0 및 과도한 concurrency를 typed error로 거부합니다. |
| 문서 | 통과 | README, README.ko, WIP, crate README, Rustdoc이 범위와 제외 항목을 설명합니다. |
| 테스트 | 통과 | Unit test가 순서, bound enforcement, sibling abort/drain, collect-all result, invalid bound, join failure drain을 다룹니다. |
| 위험 | 보통 | 새 production crate와 facade feature를 추가하지만 default feature는 확장하지 않습니다. |

## 발견 사항

- P0: 0
- P1: 0
- P2: 0
- P3: 0

## 검증

- 통과: `cargo fmt --all --check`
- 통과: `cargo check --workspace --all-targets --all-features --locked`
- 통과: `cargo test -p bluetape-rs-async`
- 통과: `cargo test --workspace --all-features --locked`
- 통과: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- 통과: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- 통과: `git diff --check`
