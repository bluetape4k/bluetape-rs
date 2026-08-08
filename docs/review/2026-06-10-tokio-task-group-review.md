# Tokio 작업 그룹 검토

## 범위

- 이슈: #21
- 마일스톤: 0.2.0
- 변경 표면: 새 `bluetape-rs-async` 크레이트와 루트 파사드 feature
- 외부 참조: `JoinSet`, `abort_all`,
  abort한 뒤 `join_next`를 drain하는 shutdown

## 7-Tier 검토

| 단계 | 결과 | 근거 |
| --- | --- | --- |
| API 계약 | Pass | `try_map_bounded`가 첫 오류 abort/drain 동작을 정의하고 `map_bounded_collect`가 전체 수집 연산 결과 동작을 정의한다. |
| Rust 관용 | Pass | `Result`, 타입 지정 오류, `JoinSet`, `Send + 'static` 작업 경계, 순서가 있는 값 결과, 공개 Rustdoc 예제를 사용한다. |
| 취소 동작 | Pass | 첫 연산 오류와 Tokio join 실패가 `abort_all`을 호출하고 남은 작업을 drain한다. |
| 경계 | Pass | 0 및 과도한 동시성을 타입 지정 오류로 거부한다. |
| 문서 | Pass | README, README.ko, WIP, 크레이트 README, Rustdoc이 범위와 제외 항목을 설명한다. |
| 테스트 | Pass | 단위 테스트가 순서, 경계 적용, 형제 abort/drain, 전체 수집 결과, 잘못된 경계, join 실패 drain을 커버한다. |
| 위험 | 보통 | 새 운영 크레이트와 파사드 feature를 추가하지만 기본 feature를 확장하지 않는다. |

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
