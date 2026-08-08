# Async timeout 및 shutdown 검토

## 범위

- 이슈: #22
- Milestone: 0.2.0
- 변경 대상: `bluetape-rs-async`
- 외부 참고: `tokio::time::timeout`, `timeout_at`, `watch`, `select!` cancellation pattern에 대한 Tokio 1.49 문서

## 7-Tier 검토

| Tier | 결과 | 근거 |
| --- | --- | --- |
| API contract | 통과 | `AsyncControlError`가 timeout과 cancellation을 구분하고, timeout/deadline helper가 typed error를 반환합니다. |
| Cancellation 동작 | 통과 | `run_until_cancelled`와 `with_timeout_or_cancel`이 호출자 소유 token을 사용하며, drop된 wrapper future를 synthetic error로 바꾸지 않습니다. |
| Cleanup | 통과 | 테스트가 in-flight future drop과 shutdown listener notification을 증명합니다. |
| Runtime 경계 | 통과 | Crate README가 Tokio 가정을 설명하고 core async task에서 blocking work를 제외합니다. |
| 문서 | 통과 | 공개 Rustdoc과 README가 timeout, deadline, cancellation, shutdown 범위를 설명합니다. |
| 테스트 | 통과 | Unit test가 success, timeout, deadline, cancellation, timeout-vs-cancel precedence, shutdown notification을 다룹니다. |
| 위험 | 보통 | 새 async crate를 확장하지만 기본 root facade feature는 변경하지 않습니다. |

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
