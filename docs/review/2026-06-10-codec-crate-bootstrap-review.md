# 코덱 crate 초기 구성 검토

날짜: 2026-06-10
이슈: #53
브랜치: `feat/issue-53-codec-crate`

## 범위

코덱 동작을 구현하지 않고 `0.3.0` 코덱 crate 경계를 추가합니다.

- `crates/codec`
- root workspace 등록
- root optional facade feature
- README 및 README.ko 사용 메모
- `Cargo.lock` workspace package 항목

## 7-Tier 검토

| Tier | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / Contract | PASS | 아직 encoder API를 노출하지 않습니다. 유일한 공개 변경은 opt-in root facade feature `codec`이며 `default = ["core"]`는 그대로입니다. |
| 2. Architecture / Boundary | PASS | `crates/codec`는 목적이 분명한 crate 경계입니다. Hex/Base64 작업은 후속 범위로 기록했고 compression과 serde serialization은 각각 `0.4.0`, `0.5.0`으로 명시적으로 보류했습니다. |
| 3. Rust API 형태 | PASS | 새 crate가 Rust 2024 workspace metadata를 사용하고 Kotlin/JVM 또는 Go 형태 API, 광범위한 utility module, unsafe code, runtime/global state를 도입하지 않습니다. |
| 4. 테스트 | PASS | `cargo test -p bluetape-rs-codec --all-features --locked`가 metadata smoke test와 함께 통과하고 전체 workspace test도 통과합니다. |
| 5. 정적 검사 / 문서 | PASS | `cargo fmt --all --check`, `git diff --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `-D warnings` rustdoc이 통과합니다. |
| 6. Release / Cargo | PASS | `Cargo.toml`이 새 workspace member, workspace dependency, optional root dependency, additive feature를 등록합니다. `Cargo.lock`은 `bluetape-rs-codec` `0.3.0`을 기록합니다. |
| 7. 근거 무결성 | PASS | code-review-graph가 `origin/develop` 대비 staged diff를 분석해 파일 8개, risk score 0.00, 변경 함수 0개, test gap 0을 보고했습니다. |

## P0/P1 Gate

P0=0 P1=0

#53에는 P2/P3 후속 작업이 필요하지 않습니다. 구현 작업은 이미 생성된
하위 이슈 #54와 #55에 남아 있습니다.

## 검증

- `cargo fmt --all --check`: PASS
- `git diff --check`: PASS
- `cargo check --workspace --all-targets --all-features --locked`: PASS
- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`: PASS
