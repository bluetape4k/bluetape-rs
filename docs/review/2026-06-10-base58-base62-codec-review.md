# Base58 및 Base62 코덱 검토

날짜: 2026-06-10
이슈: #63
브랜치: `feat/issue-63-base58-base62`

## 범위

`bluetape-rs-codec`에 목적이 분명한 Base58 및 Base62 코덱 기본 기능을
추가합니다.

- Bitcoin Base58 바이트 encoding 및 decoding
- 바이트 지향 Base62 encoding 및 decoding
- 프로젝트가 소유하는 typed decode error
- 선행 0 바이트 보존
- README 및 README.ko 예시
- 무상태 encode/decode 경로를 위한 bounded thread stress test

UUID, 정수, ID-generator rendering, checksum, random string helper는
의도적으로 범위에서 제외합니다.

## 7-Tier 검토

| Tier | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / Contract | PASS | 공개 이름이 바이트 지향 `encode_base58`, `decode_base58`, `encode_base62`, `decode_base62` API를 노출합니다. Decode는 typed `Result<Vec<u8>, ...DecodeError>`를 반환합니다. |
| 2. Architecture / Boundary | PASS | 변경은 `crates/codec`와 README wiring 안에 있습니다. #63에 UUID, 정수, random ID, checksum, serde, compression 동작을 섞지 않았습니다. |
| 3. Rust API 형태 | PASS | API가 `impl AsRef<[u8]>`, `impl AsRef<str>`, owned output value, non-exhaustive error enum을 사용하며 unsafe code가 없습니다. |
| 4. 테스트 | PASS | Unit test가 빈 입력, known vector, binary round trip, 선행 0 보존, 잘못된 문자, UTF-8 바이트 위치, 진단, thread stress round trip을 다룹니다. |
| 5. 정적 검사 / 문서 | PASS | `RUSTDOCFLAGS="-D warnings"`에서 Rustdoc 예시가 컴파일되고, README와 README.ko가 일치하는 Base58/Base62 예시와 호환성 정책을 노출합니다. |
| 6. Release / Cargo | PASS | 새 third-party dependency를 추가하지 않았고 `crates/codec/Cargo.toml` keyword metadata만 갱신했습니다. |
| 7. 근거 무결성 | PASS | 구현 전에 bluetape4k Kotlin `Base58`, `Base62`, `Url62`, KSUID `BytesBase62` 참조와 결정을 대조했습니다. Staged graph review는 10개 파일을 분석해 risk score 0.00, test gap 0을 보고했고 native review gate는 P0=0 P1=0을 보고했습니다. |

## P0/P1 Gate

P0=0 P1=0

- `code-reviewer`: PASS, P0=0 P1=0 P2=0 P3=0
- `verifier`: PASS, P0=0 P1=0 P2=0. README의 current-status 명확성 P3 하나는 commit 전에 수정했습니다.

상위 수준의 정수/UUID rendering은 ID-generator crate 범위를 계획할 때
별도로 추적할 수 있습니다.

## 검증

- `cargo test -p bluetape-rs-codec stress_round_trips_are_stable_across_threads --all-features --locked`: PASS, stress test 2개
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS, unit test 41개 + doctest 15개
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
