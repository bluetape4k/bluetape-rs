# Collections crate 검토

이슈: #19
브랜치: `feat/issue-19-collections-crate`
날짜: 2026-06-10

## 범위

첫 `0.2.0` workspace crate 경계를 추가합니다.

- `crates/collections`
- root workspace 등록
- root `collections` facade feature
- crate README 및 Rustdoc namespace 문서

## 7-Tier 검토

### Tier 1 - Rust API 형태

통과. Crate는 namespace module만으로 시작하며 집중된 helper API 이슈(#20)
전에는 가짜 helper function을 추가하지 않습니다. Root facade feature는
optional이고 additive입니다.

### Tier 2 - Workspace 및 Cargo metadata

통과. `crates/collections`를 workspace member로 등록하고 목적이 분명한
package name을 사용하며 workspace edition/rust-version/license/repository
metadata를 사용하고 workspace dependency로 연결했습니다.

### Tier 3 - Feature flag

통과. Root `collections` feature는 optional이고 default feature에 포함되지
않아 좁은 default facade를 보존합니다.

### Tier 4 - 문서

통과. Crate README와 crate/module Rustdoc이 의도한 helper namespace를
설명하고, 명확한 경우 표준 library API를 우선한다는 규칙을 보존합니다.

### Tier 5 - 동작 위험

통과. 아직 collection helper 동작을 주장하지 않습니다. Diff는 crate 구조와
facade wiring만 추가합니다.

### Tier 6 - 검증

통과.

- `git diff --check`
- `cargo fmt --all --check`
- `cargo check --workspace`
- `cargo test --workspace --all-features`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo doc --workspace --no-deps`
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`

### Tier 7 - Release/Milestone 적합성

통과. 변경은 crate 경계만으로 milestone `0.2.0`을 시작합니다. Helper API는
이슈 #20으로 보류합니다.

## 발견 사항

P0/P1/P2/P3 발견 사항이 없습니다.

## 판정

PASS

P0 개수: 0
P1 개수: 0
