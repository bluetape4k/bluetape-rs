# 엄격한 hex 코덱 검토

## 범위

- 이슈: #54
- 브랜치: `feat/issue-54-strict-hex`
- 검토 diff: `bluetape-rs-codec`의 엄격한 hex 기본 요소, 공개 README 예제,
  집중 단위/Rustdoc 커버리지

## 발견 사항

- P0: 0
- P1: 0
- P2: 0
- P3: 0

차단 발견 사항 없음.

## 검토 메모

- API 형태는 Rust 네이티브다. `AsRef<[u8]>`를 통해 byte slice를 받고,
  decode는 `AsRef<str>`를 받으며, encode는 소유 `String`을 반환하고, decode는
  `Result<Vec<u8>, HexDecodeError>`를 반환한다.
- 오류 계약은 타입이 지정되어 서비스 로그에 충분한 진단 정보를 제공한다:
  `OddLength { len }` 및 `InvalidCharacter { index, byte }`.
- decoder는 엄격하다. prefix, whitespace, separator 및 non-ASCII digit를
  허용하지 않는다.
- hex encoding을 위한 의존성은 추가하지 않았다. 구현이 충분히 작아 first-party로
  유지할 수 있고 이슈 #54에 wrapper dependency를 추가하지 않는다.
- 테스트는 빈 입력, 바이너리 바이트, lowercase/uppercase 출력, mixed-case
  decode, 홀수 길이, 잘못된 high/low nibble 위치, non-ASCII 입력 및 오류
  formatting을 다룬다.

## 검증

| 확인 항목 | 상태 | 근거 |
| --- | --- | --- |
| Diff 공백 | PASS | `git diff --check` |
| 포맷 | PASS | `cargo fmt --all --check` |
| 이슈 인수 테스트 | PASS | `cargo test -p bluetape-rs-codec --all-features --locked` |
| 워크스페이스 테스트 | PASS | `cargo test --workspace` |
| Clippy | PASS | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |

## 판정

PASS. P0=0 P1=0.
