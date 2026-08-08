# 코덱 테스트 배치 검토

날짜: 2026-06-10
이슈: #57
브랜치: `feat/issue-57-codec-test-separation`

## 범위

공개 `bluetape-rs-codec` test를 source module에서 분리합니다.

- hex 공개 API test를 `crates/codec/tests/hex.rs`로 이동
- Base64 공개 API test를 `crates/codec/tests/base64.rs`로 이동
- Base58 공개 API 및 stress test를 `crates/codec/tests/base58.rs`로 이동
- Base62 공개 API 및 stress test를 `crates/codec/tests/base62.rs`로 이동
- UTF-8 text 공개 API test를 `crates/codec/tests/text.rs`에 유지
- private `base_n` test는 source-local로 유지

## 7-Tier 검토

| Tier | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / Contract | PASS | 공개 API signature는 변경하지 않았고, test가 이제 `bluetape_rs_codec`를 통해 import합니다. |
| 2. Architecture / Boundary | PASS | 공개 동작 test를 integration test로 옮겼고 private `base_n` algorithm test는 source-local로 남겼습니다. |
| 3. Rust API 형태 | PASS | API 형태 변경이 없습니다. Test file은 crate 경계를 통해 owned output과 typed error를 검증합니다. |
| 4. 테스트 | PASS | Codec validation이 source unit test 3개, integration test 44개, doctest 18개를 보고합니다. |
| 5. 정적 검사 / 문서 | PASS | README.md, README.ko.md, `crates/codec/README.md`가 테스트 배치를 문서화합니다. |
| 6. Release / Cargo | PASS | Dependency와 Cargo metadata 변경이 없습니다. |
| 7. 근거 무결성 | PASS | Code-review-graph context가 `origin/develop` 대비 risk가 낮고 source/doc 변경 파일 7개, 영향 node 0개, test gap 0개라고 보고했습니다. Test 이동 후 전체 workspace validation도 통과했습니다. |

## P0/P1 Gate

P0=0 P1=0

## 검증

- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS, unit test 3개 + integration test 44개 + doctest 18개
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
