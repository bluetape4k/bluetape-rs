# Base64 코덱 검토

날짜: 2026-06-10
이슈: #55
브랜치: `feat/issue-55-base64`

## 범위

`bluetape-rs-codec`에 다음의 집중된 Base64 코덱 기본 요소를 추가한다.

- standard alphabet, padded 및 unpadded variant
- URL-safe alphabet, padded 및 unpadded variant
- 프로젝트가 소유하는 typed decode error
- README 및 README.ko 예제
- workspace dependency를 통한 `base64` crate dependency

Base58과 Base62는 의도적으로 제외하고 #63에서 추적한다.

## 7-Tier 검토

| 단계 | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / 계약 | PASS | 공개 이름이 알파벳과 패딩 정책을 명시한다. standard 대 URL-safe, padded 대 `_unpadded`를 구분한다. 디코드는 `Result<Vec<u8>, Base64DecodeError>`를 반환한다. |
| 2. 아키텍처 / 경계 | PASS | 변경은 `crates/codec`, 워크스페이스 의존성, README 연결로 한정된다. 압축, serde, Base58, Base62 범위를 #55에 섞지 않았다. |
| 3. Rust API 형태 | PASS | API는 인코드 입력에 `impl AsRef<[u8]>`, 디코드 입력에 `impl AsRef<str>`, 소유 출력 값, non-exhaustive 오류 열거형을 사용한다. 안전하지 않은 코드나 런타임 상태를 도입하지 않았다. |
| 4. 테스트 | PASS | 단위 테스트가 빈 입력, standard 및 URL-safe 알파벳, padded 및 unpadded 왕복, 알파벳 거부, 패딩 누락/초과 거부, 잘못된 길이, 진단 형식을 커버한다. |
| 5. 정적 검사 / 문서 | PASS | Rustdoc 예제가 `RUSTDOCFLAGS="-D warnings"`에서 컴파일되며 README와 README.ko가 일치하는 Base64 예제를 노출한다. |
| 6. 릴리스 / Cargo | PASS | `base64 = "0.22.1"`을 워크스페이스 의존성으로 추가하고 `bluetape-rs-codec`만 사용한다. `Cargo.lock`을 갱신했다. |
| 7. 근거 무결성 | PASS | 업스트림 `base64` 0.22.1 패딩 모드인 padded engine의 `RequireCanonical`, no-padding engine의 `RequireNone`과 구현을 대조했다. staged graph 검토는 위험 점수 0.00, 테스트 공백 0으로 10개 파일을 분석했다. |

## P0/P1 게이트

P0=0 P1=0

네이티브 검토 레인:

- `code-reviewer`: PASS, P0=0 P1=0. API 범위, Rust surface, README 동등성,
  테스트, cargo gate 및 unsafe/debug/secret pattern 없음을 확인했다.
- `verifier`: 로컬 구현은 PASS, P0=0 P1=0. PR/CI/post-PR gate를 완료하기 전까지
  end-to-end workflow는 부분 상태로 남는다.

#55에는 P2/P3 후속 작업이 필요하지 않다. PR #62가 아직 열려 있으므로 두
PR이 코덱 README/Rustdoc 표면을 함께 수정하기 때문에 #62가 병합된 후 이
브랜치를 rebase해야 할 수 있다.

## 검증

- `cargo test -p bluetape-rs-codec base64::tests:: --all-features --locked`: PASS
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
