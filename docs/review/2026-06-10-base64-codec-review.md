# Base64 코덱 검토

날짜: 2026-06-10
이슈: #55
브랜치: `feat/issue-55-base64`

## 범위

`bluetape-rs-codec`에 목적이 분명한 Base64 코덱 기본 기능을 추가합니다.

- 표준 알파벳의 padded 및 unpadded variant
- URL-safe 알파벳의 padded 및 unpadded variant
- 프로젝트가 소유하는 typed decode error
- README 및 README.ko 예시
- workspace dependency를 통한 `base64` crate dependency

Base58과 Base62는 의도적으로 제외하고 #63에서 추적합니다.

## 7-Tier 검토

| Tier | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / Contract | PASS | 공개 이름이 알파벳과 padding 정책을 명시합니다. 표준과 URL-safe, padded와 `_unpadded`를 구분하며 Decode는 `Result<Vec<u8>, Base64DecodeError>`를 반환합니다. |
| 2. Architecture / Boundary | PASS | 변경은 `crates/codec`, workspace dependency, README wiring 안에 있습니다. #55에 compression, serde, Base58, Base62 범위를 섞지 않았습니다. |
| 3. Rust API 형태 | PASS | Encode 입력은 `impl AsRef<[u8]>`, decode 입력은 `impl AsRef<str>`를 사용하고 owned output value와 non-exhaustive error enum을 제공합니다. Unsafe code와 runtime state는 추가하지 않았습니다. |
| 4. 테스트 | PASS | Unit test가 빈 입력, 표준 및 URL-safe 알파벳, padded 및 unpadded round trip, 알파벳 거부, padding 누락/추가 거부, 잘못된 길이, 진단 formatting을 다룹니다. |
| 5. 정적 검사 / 문서 | PASS | `RUSTDOCFLAGS="-D warnings"`에서 Rustdoc 예시가 컴파일되고 README와 README.ko가 일치하는 Base64 예시를 노출합니다. |
| 6. Release / Cargo | PASS | `base64 = "0.22.1"`을 workspace dependency로 추가하고 `bluetape-rs-codec`만 사용합니다. `Cargo.lock`을 갱신했습니다. |
| 7. 근거 무결성 | PASS | 구현을 upstream `base64` 0.22.1 padding mode와 대조했습니다. Padded engine에는 `RequireCanonical`, no-padding engine에는 `RequireNone`을 사용합니다. Staged graph review는 10개 파일을 분석해 risk score 0.00, test gap 0을 보고했습니다. |

## P0/P1 Gate

P0=0 P1=0

Native review lane:

- `code-reviewer`: PASS, P0=0 P1=0. API 범위, Rust surface, README parity, test, cargo gate, unsafe/debug/secret pattern 부재를 확인했습니다.
- `verifier`: 로컬 구현은 PASS, P0=0 P1=0. PR/CI/post-PR gate가 완료되기 전까지 end-to-end workflow는 부분 상태입니다.

#55에 P2/P3 후속 작업은 필요하지 않습니다. PR #62가 아직 열려 있으므로
두 PR이 codec README/Rustdoc 영역을 모두 수정한다는 점을 고려해 #62 merge
후 이 브랜치를 rebase해야 할 수 있습니다.

## 검증

- `cargo test -p bluetape-rs-codec base64::tests:: --all-features --locked`: PASS
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
