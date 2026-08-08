# Strict Hex 코덱 검토

## 범위

- 이슈: #54
- 브랜치: `feat/issue-54-strict-hex`
- 검토 diff: `bluetape-rs-codec`의 strict hex 기본 기능, 공개 README 예시, 집중 unit/Rustdoc coverage

## 발견 사항

- P0: 0
- P1: 0
- P2: 0
- P3: 0

Blocker 발견 사항이 없습니다.

## 검토 메모

- API 형태가 Rust-native입니다. Byte slice는 `AsRef<[u8]>`로 받고, decode는 `AsRef<str>`를 받으며, encode는 owned `String`을 반환하고 decode는 `Result<Vec<u8>, HexDecodeError>`를 반환합니다.
- Error contract가 typed이고 service log에 충분한 진단 정보를 제공합니다: `OddLength { len }` 및 `InvalidCharacter { index, byte }`입니다.
- Decoder는 strict합니다. Prefix, whitespace, separator, non-ASCII digit을 허용하지 않습니다.
- Hex encoding에 dependency를 추가하지 않았습니다. 구현이 작으므로 first-party로 유지하고 #54에 wrapper dependency를 추가하지 않습니다.
- Test가 empty input, binary byte, lowercase/uppercase output, mixed-case decode, odd length, invalid high/low nibble position, non-ASCII input, error formatting을 다룹니다.

## 검증

| 검사 | 상태 | 근거 |
| --- | --- | --- |
| Diff whitespace | PASS | `git diff --check` |
| Format | PASS | `cargo fmt --all --check` |
| 이슈 acceptance test | PASS | `cargo test -p bluetape-rs-codec --all-features --locked` |
| Workspace test | PASS | `cargo test --workspace` |
| Clippy | PASS | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |

## 판정

PASS. P0=0 P1=0
