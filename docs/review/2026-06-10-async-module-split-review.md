# Async 모듈 분리 검토

## 범위

- 이슈: #42
- Milestone: 0.2.0
- 변경 대상: `bluetape-rs-async` source layout

## 7-Tier 검토

| Tier | 결과 | 근거 |
| --- | --- | --- |
| 모듈 구조 | 통과 | `lib.rs`가 목적별 `control` 및 `task_group` 모듈을 감싸는 facade입니다. |
| API 호환성 | 통과 | 공개 이름은 `bluetape_rs_async::*`에서 계속 re-export되고, crate test와 doctest가 통과합니다. |
| 동작 | 통과 | 기존 async unit, integration, workspace, all-feature test가 통과합니다. |
| 문서 | 통과 | 공개 Rustdoc이 crate facade와 목적별 구현 모듈에 남아 있고 rustdoc warning은 거부됩니다. |
| 위험 | 낮음 | 공개 API를 바꾸지 않는 refactor-only 분리입니다. |

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
