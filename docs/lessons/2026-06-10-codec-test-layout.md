# 코덱 테스트 배치

날짜: 2026-06-10
이슈: #57

## 결정

공개 `bluetape-rs-codec` 동작 테스트는
`crates/codec/tests/` 아래 integration test로 유지합니다.

- `base58.rs`
- `base62.rs`
- `base64.rs`
- `hex.rs`
- `text.rs`

이 구조는 downstream 사용자가 호출하는 공개 crate 경계를 검증합니다.

## 예외

비공개 구현 세부 사항은 source-local test로 유지할 수 있습니다. 공유
`base_n` converter는 crate 비공개이므로 해당 알고리즘 테스트는
`crates/codec/src/base_n.rs`에 둡니다.

## 기각한 대안

모든 workspace test를 한 번에 옮기면 crate 전반에 무관한 변경이 발생합니다.
#57의 범위는 코덱 milestone의 테스트 배치로 한정합니다.
