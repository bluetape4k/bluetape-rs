# 비동기 타임아웃 및 종료 검토

## 범위

- 이슈: #22
- 마일스톤: 0.2.0
- 변경 표면: `bluetape-rs-async`
- 외부 참조: `tokio::time::timeout`,
  `timeout_at`, `watch` 및 `select!` cancellation pattern

## 7-Tier 검토

| 단계 | 결과 | 근거 |
| --- | --- | --- |
| API 계약 | Pass | `AsyncControlError`가 타임아웃과 취소를 구분하며 타임아웃/기한 헬퍼가 타입 지정 오류를 반환한다. |
| 취소 동작 | Pass | `run_until_cancelled`와 `with_timeout_or_cancel`은 호출자가 소유한 토큰을 사용하고 drop된 wrapper future를 인공 오류로 변환하지 않는다. |
| 정리 | Pass | 테스트에서 취소가 진행 중인 future를 drop하고 종료 listener에 알림을 보내는지 증명한다. |
| 런타임 경계 | Pass | 크레이트 README가 Tokio 전제를 문서화하고 핵심 비동기 작업의 blocking 작업을 제외한다. |
| 문서 | Pass | 공개 Rustdoc과 README가 타임아웃, 기한, 취소, 종료 범위를 설명한다. |
| 테스트 | Pass | 단위 테스트가 성공, 타임아웃, 기한, 취소, 타임아웃 대 취소 우선순위, 종료 알림을 커버한다. |
| 위험 | 보통 | 새 비동기 크레이트를 확장하지만 기본 루트 파사드 feature는 변경하지 않는다. |

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
