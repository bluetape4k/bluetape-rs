# 코덱 텍스트 경계 검토

날짜: 2026-06-10
이슈: #56
브랜치: `feat/issue-56-codec-helper-boundaries`

## 범위

`bluetape-rs-codec`의 최소 바이너리/텍스트 헬퍼 경계를 정의하고 구현한다.

- binary encoder 전에 UTF-8 text를 owned bytes로 변환
- typed non-lossy error와 함께 decoded bytes를 UTF-8 text로 변환
- 이름이 명시된 lossy UTF-8 replacement helper
- 압축, 직렬화, 광범위한 텍스트 유틸리티, 암호화, 서명, 체크섬,
  임의 문자열 및 데이터베이스 bind 인코딩에 대한 README/Rustdoc 비목표

## 7-Tier 검토

| 단계 | 판정 | 근거 |
|---|---|---|
| 1. 공개 API / 계약 | PASS | `encode_utf8_text`, `decode_utf8_text`, `decode_utf8_text_lossy`는 UTF-8 텍스트/바이트 경계 동작만 노출한다. 손실 없는 디코드는 `Result<String, TextDecodeError>`를 반환한다. |
| 2. 아키텍처 / 경계 | PASS | 변경은 `crates/codec`과 README/검토 문서로 한정된다. 일반 문자열 유틸리티, 정규화, 압축, 직렬화, 암호화, 서명, checksum, 임의 문자열, 데이터베이스 bind 인코딩은 범위 밖이다. |
| 3. Rust API 형태 | PASS | API는 소유 `Vec<u8>`/`String` 출력, `impl AsRef` 또는 `impl Into<Vec<u8>>` 입력, 순수 변환에 `#[must_use]`, non-exhaustive 공개 오류 열거형을 사용한다. |
| 4. 테스트 | PASS | 공개 API 테스트는 `crates/codec/tests/text.rs`에 있고 빈/비ASCII 텍스트 인코딩, Base64 디코드 후 텍스트 변환, 잘못된 UTF-8 거부, 불완전한 UTF-8 진단, 명시적 손실 대체, 오류 형식을 커버한다. |
| 5. 정적 검사 / 문서 | PASS | Rustdoc 예제가 컴파일된다. README.md, README.ko.md, `crates/codec/README.md`가 UTF-8 및 손실/손실 없는 동작을 문서화한다. |
| 6. 릴리스 / Cargo | PASS | Cargo 메타데이터나 의존성을 변경하지 않았다. 광범위한 유틸리티 모듈을 도입하지 않았다. |
| 7. 근거 무결성 | PASS | Codegraph 검토 맥락에서 `origin/develop` 대비 낮은 위험, 소스/문서 변경 파일 7개, 영향 노드 0개, 테스트 공백 0개를 보고했다. `git diff --cached --name-only`에 추가된 통합 테스트 파일이 포함된다. Base64 UTF-8 예제 벡터를 수정한 뒤 로컬 검증을 실행했다. |

## P0/P1 게이트

P0=0 P1=0

#56에는 P2/P3 후속 작업이 필요하지 않다.

이번 실행에서는 사용 가능한 서브에이전트 도구가 명시적으로 요청한 위임으로
제한되어 네이티브 서브에이전트를 생성하지 않았다. code-review-graph 맥락과
전체 워크스페이스 검증으로 로컬 검토 게이트를 완료했다.

## 검증

- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS, 41 unit tests + 6 integration tests + 18 doctests
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
