# Base58 및 Base62 코덱 검토

날짜: 2026-06-10
이슈: #63
브랜치: `feat/issue-63-base58-base62`

## 범위

`bluetape-rs-codec`에 다음의 집중된 Base58 및 Base62 코덱 기본 요소를
추가한다.

- Bitcoin Base58 byte encoding 및 decoding
- byte-oriented Base62 encoding 및 decoding
- 프로젝트가 소유하는 typed decode error
- leading zero byte 보존
- README 및 README.ko 예제
- stateless encode/decode 경로의 제한된 thread stress test

UUID, 정수, ID-generator 렌더링, checksum, 임의 문자열 헬퍼는 의도적으로
범위에서 제외한다.

## 7-Tier 검토

| 단계 | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / 계약 | PASS | 공개 이름이 바이트 지향 `encode_base58`, `decode_base58`, `encode_base62`, `decode_base62` API를 노출한다. 디코드는 타입이 지정된 `Result<Vec<u8>, ...DecodeError>`를 반환한다. |
| 2. 아키텍처 / 경계 | PASS | 변경은 `crates/codec`과 README 연결로 한정된다. UUID, 정수, 임의 ID, checksum, serde, 압축 동작을 #63에 섞지 않았다. |
| 3. Rust API 형태 | PASS | API가 `impl AsRef<[u8]>`, `impl AsRef<str>`, 소유 출력 값, non-exhaustive 오류 열거형을 사용하며 안전하지 않은 코드를 사용하지 않는다. |
| 4. 테스트 | PASS | 단위 테스트가 빈 입력, 알려진 벡터, 바이너리 왕복, 선행 0 보존, 잘못된 문자, UTF-8 바이트 위치, 진단, 스레드 스트레스 왕복을 커버한다. |
| 5. 정적 검사 / 문서 | PASS | Rustdoc 예제가 `RUSTDOCFLAGS="-D warnings"`에서 컴파일되며 README와 README.ko가 일치하는 Base58/Base62 예제와 호환성 정책을 노출한다. |
| 6. 릴리스 / Cargo | PASS | 새 서드파티 의존성을 추가하지 않았다. `crates/codec/Cargo.toml`의 keyword 메타데이터만 변경했다. |
| 7. 근거 무결성 | PASS | 구현 전에 bluetape4k Kotlin `Base58`, `Base62`, `Url62`, KSUID `BytesBase62` 참조와 구현 결정을 대조했다. staged graph 검토는 위험 점수 0.00, 테스트 공백 0으로 10개 파일을 분석했다. 네이티브 검토 게이트는 P0=0 P1=0을 보고했다. |

## P0/P1 게이트

P0=0 P1=0

- `code-reviewer`: PASS, P0=0 P1=0 P2=0 P3=0.
- `verifier`: PASS, P0=0 P1=0 P2=0. 커밋 전에 P3 README current-status 명확성
  공백 하나를 수정했다.

상위 수준의 정수/UUID 렌더링은 ID-generator 크레이트 범위를 계획할 때
별도로 추적할 수 있다.

## 검증

- `cargo test -p bluetape-rs-codec stress_round_trips_are_stable_across_threads --all-features --locked`: PASS, 2 stress tests
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS, 41 unit tests + 15 doctests
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
