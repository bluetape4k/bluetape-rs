# 코덱 크레이트 부트스트랩 검토

날짜: 2026-06-10
이슈: #53
브랜치: `feat/issue-53-codec-crate`

## 범위

코덱 동작은 구현하지 않고 `0.3.0` 코덱 크레이트 경계를 추가한다.

- `crates/codec`
- 루트 워크스페이스 등록
- 루트 선택적 파사드 feature
- README 및 README.ko 사용 메모
- `Cargo.lock` 워크스페이스 패키지 항목

## 7-Tier 검토

| 단계 | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / 계약 | PASS | 아직 encoder API를 노출하지 않는다. 유일한 공개 변경은 옵트인 루트 파사드 feature `codec`이며 `default = ["core"]`는 변경하지 않았다. |
| 2. 아키텍처 / 경계 | PASS | `crates/codec`은 집중된 크레이트 경계다. Hex/Base64 작업은 후속 범위로 문서화했고 압축 및 serde 직렬화는 `0.4.0`, `0.5.0`으로 명시적으로 미뤘다. |
| 3. Rust API 형태 | PASS | 새 크레이트가 Rust 2024 워크스페이스 메타데이터를 사용하고 Kotlin/JVM 또는 Go 형태의 API, 광범위한 유틸리티 모듈, 안전하지 않은 코드, 런타임/전역 상태를 사용하지 않는다. |
| 4. 테스트 | PASS | `cargo test -p bluetape-rs-codec --all-features --locked`가 메타데이터 스모크 테스트와 함께 통과한다. 전체 워크스페이스 테스트도 통과한다. |
| 5. 정적 검사 / 문서 | PASS | `cargo fmt --all --check`, `git diff --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `-D warnings`를 적용한 rustdoc이 통과한다. |
| 6. 릴리스 / Cargo | PASS | `Cargo.toml`이 새 워크스페이스 멤버, 워크스페이스 의존성, 선택적 루트 의존성, 추가형 feature를 등록한다. `Cargo.lock`에 `bluetape-rs-codec` `0.3.0`이 기록된다. |
| 7. 근거 무결성 | PASS | code-review-graph가 `origin/develop` 대비 staged diff를 분석했다. 파일 8개, 위험 점수 0.00, 변경 함수 0개, 테스트 공백 0이다. |

## P0/P1 게이트

P0=0 P1=0

#53에는 P2/P3 후속 작업이 필요하지 않다. 구현 작업은 이미 생성된 하위
이슈 #54와 #55에 남긴다.

## 검증

- `cargo fmt --all --check`: PASS
- `git diff --check`: PASS
- `cargo check --workspace --all-targets --all-features --locked`: PASS
- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`: PASS
