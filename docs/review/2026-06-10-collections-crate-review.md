# 컬렉션 크레이트 검토

이슈: #19
브랜치: `feat/issue-19-collections-crate`
날짜: 2026-06-10

## 범위

첫 번째 `0.2.0` 워크스페이스 크레이트 경계를 추가한다.

- `crates/collections`
- 루트 워크스페이스 등록
- 루트 `collections` 파사드 feature
- 크레이트 README 및 Rustdoc 네임스페이스 문서

## 7-Tier 검토

### 단계 1 - Rust API 형태

PASS. 크레이트는 네임스페이스 모듈로만 시작하고 집중 헬퍼 API 이슈(#20)
전에 가짜 헬퍼 함수를 도입하지 않는다. 루트 파사드 feature는 선택적이고
추가형이다.

### 단계 2 - 워크스페이스 및 Cargo 메타데이터

PASS. `crates/collections`가 워크스페이스 멤버로 등록되고 집중된 패키지
이름을 가지며 워크스페이스 edition/rust-version/license/repository
메타데이터를 사용하고 워크스페이스 의존성으로 연결된다.

### 단계 3 - Feature 플래그

PASS. 루트 `collections` feature는 선택적이며 기본 feature에 포함되지 않아
좁은 기본 파사드를 보존한다.

### 단계 4 - 문서

PASS. 크레이트 README와 크레이트/모듈 Rustdoc이 의도한 헬퍼 네임스페이스를
설명하고 표준 라이브러리 API가 명확하면 우선한다는 규칙을 유지한다.

### 단계 5 - 동작 위험

PASS. 아직 컬렉션 헬퍼 동작을 주장하지 않는다. diff는 크레이트 구조와
파사드 연결만 추가한다.

### 단계 6 - 검증

PASS.

- `git diff --check`
- `cargo fmt --all --check`
- `cargo check --workspace`
- `cargo test --workspace --all-features`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo doc --workspace --no-deps`
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`

### 단계 7 - 릴리스/마일스톤 적합성

PASS. 변경은 크레이트 경계만 추가해 `0.2.0` 마일스톤을 시작한다. 헬퍼 API는
이슈 #20으로 미룬다.

## 발견 사항

P0/P1/P2/P3 발견 사항 없음.

## 판정

PASS.

P0 count: 0
P1 count: 0
