# 코덱 텍스트 경계 검토

날짜: 2026-06-10
이슈: #56
브랜치: `feat/issue-56-codec-helper-boundaries`

## 범위

`bluetape-rs-codec`에 필요한 최소 binary/text helper 경계를 정의하고
구현합니다.

- Binary encoder 이전에 UTF-8 text를 owned bytes로 변환
- Decode한 bytes를 typed non-lossy error와 함께 UTF-8 text로 변환
- 이름에서 lossy 동작을 명시하는 UTF-8 replacement helper
- Compression, serialization, 광범위한 text utility, encryption, signing, checksum, random string, database bind encoding에 대한 README/Rustdoc non-goal

## 7-Tier 검토

| Tier | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / Contract | PASS | `encode_utf8_text`, `decode_utf8_text`, `decode_utf8_text_lossy`가 UTF-8 text/byte 경계 동작만 노출합니다. Non-lossy decode는 `Result<String, TextDecodeError>`를 반환합니다. |
| 2. Architecture / Boundary | PASS | 변경은 `crates/codec`와 README/review docs에 머뭅니다. General string utility, normalization, compression, serialization, encryption, signing, checksum, random string, database bind encoding은 범위 밖입니다. |
| 3. Rust API 형태 | PASS | API가 owned `Vec<u8>`/`String` output, `impl AsRef` 또는 `impl Into<Vec<u8>>` input, 순수 변환에 대한 `#[must_use]`, non-exhaustive 공개 error enum을 사용합니다. |
| 4. 테스트 | PASS | 공개 API test가 `crates/codec/tests/text.rs`에 있으며 빈/비 ASCII text encoding, Base64 decode 이후 text conversion, invalid UTF-8 rejection, incomplete UTF-8 진단, 명시적 lossy replacement, error formatting을 다룹니다. |
| 5. 정적 검사 / 문서 | PASS | Rustdoc 예시가 컴파일됩니다. README.md, README.ko.md, `crates/codec/README.md`가 UTF-8 및 lossy/non-lossy 동작을 설명합니다. |
| 6. Release / Cargo | PASS | Cargo metadata와 dependency 변경이 없습니다. 광범위한 utility-bag module을 추가하지 않았습니다. |
| 7. 근거 무결성 | PASS | Codegraph review context가 `origin/develop` 대비 risk가 낮고 source/doc 변경 파일 7개, 영향 node 0개, test gap 0개라고 보고했습니다. `git diff --cached --name-only`에 새 integration test file도 포함되어 있습니다. Base64 UTF-8 예시 vector를 수정한 뒤 로컬 validation을 실행했습니다. |

## P0/P1 Gate

P0=0 P1=0

#56에는 P2/P3 후속 작업이 필요하지 않습니다.

이번 실행에서는 사용 가능한 subagent 도구가 명시적인 사용자 위임으로
제한되어 native subagent를 생성하지 않았습니다. Code-review-graph context와
전체 workspace validation으로 검토 gate를 로컬에서 완료했습니다.

## 검증

- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS, unit test 41개 + integration test 6개 + doctest 18개
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
