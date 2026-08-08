# 이슈 #108 구현 검토 - 직렬화 크레이트 부트스트랩

날짜: 2026-06-13
범위: `origin/develop` 대비 이슈 #108 구현 diff
게이트: Step 6-R 구현 diff 검토

## 검토 범위

- `Cargo.toml`, `Cargo.lock` 및 루트 `src/lib.rs`
- 새 `crates/serialization/**`
- 루트 `README.md` 및 `README.ko.md`
- `WIP.md`
- `docs/superpowers/specs/2026-06-13-serde-0-5x-design.md`
- `docs/superpowers/plans/2026-06-13-serialization-crate-bootstrap-plan.md`
- Step 2-R 및 Step 3-R 검토 산출물

## 검증 근거

- `cargo fmt --all --check`
- `cargo test -p bluetape-rs-serialization --all-features --locked`
- `cargo metadata --no-deps --format-version 1 --locked`
- `cargo check -p bluetape-rs --locked`
- `cargo check -p bluetape-rs --no-default-features --locked`
- `cargo check -p bluetape-rs --features serialization --locked`
- `cargo tree -e features -p bluetape-rs --locked`
- `cargo tree -e features -p bluetape-rs --no-default-features --locked`
- `cargo tree -e features -p bluetape-rs --features serialization --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `git diff --check`

## 검토 레인

| 레인 | 초기 결과 | 최종 결과 | 근거 |
|---|---:|---:|---|
| 성능 | `P0=0 P1=0` | `P0=0 P1=0` | 비어 있는 직렬화 기본값, 새 의존성 없음, 옵트인 루트 파사드만 사용, 부트스트랩에 대한 벤치마크 주장 없음. |
| 안정성 | `P0=0 P1=0` | `P0=0 P1=0` | feature tree가 기본/기본 feature를 끈 빌드에서 직렬화를 제외함을 증명하고 WIP 추적성 P2를 수정했다. |
| 보안 | `P0=0 P1=0` | `P0=0 P1=0` | serializer 구현이나 안전하지 않은 경로가 없으며 문서가 숨겨진 전역/기본/환경 선택 serializer와 안전하지 않은 역직렬화를 거부한다. |
| 운영자/Ops | `P0=0 P1=1` | `P0=0 P1=0` | 추적하는 Step 6-R 산출물과 계획이 요구한 lesson 산출물을 추가했다. |
| 개발자/API | `P0=0 P1=0` | `P0=0 P1=0` | 크레이트 루트가 자매 스타일을 따르고 Cargo feature/루트 파사드 형태가 추가형이며 이 파일로 산출물 P2를 수정했다. |
| 사용자/호출자 | `P0=0 P1=1` | `P0=0 P1=0` | 게시하지 않은 `0.4.0` 레지스트리 조각을 git/사전 릴리스 및 `0.5` 게시 후 예제로 교체했고 루트 README가 부트스트랩 전용 범위를 명시한다. |

## 통합 발견 사항 및 수정

| 우선순위 | 영역 | 해결 |
|---|---|---|
| P1 | 검토 근거 | 커밋/PR 전에 `docs/review/2026-06-13-issue-108-serialization-crate-review.md`와 `docs/lessons/2026-06-13-serialization-crate-bootstrap.md`를 추가했다. |
| P1 | 공개 버전 조각 | 아직 릴리스하지 않은 직렬화 feature/크레이트의 공개 `0.4.0` 레지스트리 예제를 git 의존성 예제와 `0.5.0` 게시 후 예제로 교체했다. |
| P2 | WIP 추적성 | `0.5.0` WIP 섹션에 `bluetape-rs-serialization`, `bluetape_rs_serialization`, 루트 `serialization` feature, 부트스트랩 전용 범위를 추가했다. |
| P2 | 루트 README 과장 | 루트 패키지 표의 문구를 구현된 SerDe에서 예약된 경계로 바꾸고 부트스트랩 전용 주의 사항을 추가했다. |
| P3 | 집중 크레이트 예제 | 트레이트나 어댑터가 존재하기 전까지 `bluetape-rs-serialization`을 호출 가능한 API 예제에서 제외한다고 문서화했다. |

## 보류한 후속 확인

- 이후 어댑터 PR은 손상된 바이트, 잘린 바이트, 후행 바이트, 빈 바이트,
  알 수 없는 형식 ID, 콘텐츠 타입 불일치, 지원하지 않는 버전, 잘못된 대상
  타입, 신뢰 프로파일 불일치, 초과 페이로드, 압축된 잘못된 페이로드,
  어댑터 실패 사례에 대한 부정 테스트를 추가해야 한다.
- 이후 릴리스 준비 작업은 패키지 버전을 갱신하고 실제 게시 버전에 맞춰
  crates.io/docs.rs 예제를 검증해야 한다.
- 저장소 간 동일 조건 벤치마크 작업은 어댑터가 존재한 뒤 `0.5.5`
  마일스톤으로 미룬다.

## 게이트 판정

문서 및 근거를 수정한 뒤 Step 6-R을 통과했다.

P0=0 P1=0
