# 코덱 테스트 배치 검토

날짜: 2026-06-10
이슈: #57
브랜치: `feat/issue-57-codec-test-separation`

## 범위

공개 `bluetape-rs-codec` 테스트를 소스 모듈에서 분리한다.

- hex public API test는 `crates/codec/tests/hex.rs`로 이동
- Base64 public API test는 `crates/codec/tests/base64.rs`로 이동
- Base58 public API 및 stress test는 `crates/codec/tests/base58.rs`로 이동
- Base62 public API 및 stress test는 `crates/codec/tests/base62.rs`로 이동
- UTF-8 text public API test는 `crates/codec/tests/text.rs`에 유지
- private `base_n` test는 source-local로 유지

## 7-Tier 검토

| 단계 | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / 계약 | PASS | 공개 API 시그니처는 변경하지 않았고 테스트는 이제 `bluetape_rs_codec`를 통해 import한다. |
| 2. 아키텍처 / 경계 | PASS | 공개 동작 테스트를 통합 테스트로 옮겼고 비공개 `base_n` 알고리즘 테스트는 소스 가까이에 유지한다. |
| 3. Rust API 형태 | PASS | API 형태를 변경하지 않았다. 테스트 파일은 크레이트 경계를 통해 소유 출력과 타입 지정 오류를 검증한다. |
| 4. 테스트 | PASS | 코덱 검증 결과 소스 단위 테스트 3개, 통합 테스트 44개, doctest 18개다. |
| 5. 정적 검사 / 문서 | PASS | README.md, README.ko.md, `crates/codec/README.md`가 테스트 배치를 문서화한다. |
| 6. 릴리스 / Cargo | PASS | 의존성이나 Cargo 메타데이터를 변경하지 않았다. |
| 7. 근거 무결성 | PASS | Code-review-graph 맥락에서 `origin/develop` 대비 낮은 위험, 소스/문서 변경 파일 7개, 영향 노드 0개, 테스트 공백 0개를 보고했다. 테스트를 옮긴 뒤 전체 워크스페이스 검증이 통과했다. |

## P0/P1 게이트

P0=0 P1=0

## 검증

- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS, 3 unit tests + 44 integration tests + 18 doctests
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
