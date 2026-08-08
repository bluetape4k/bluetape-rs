# 코덱 텍스트 경계

날짜: 2026-06-10
이슈: #56

## 결정

`bluetape-rs-codec`의 binary/text helper는 명시적인 UTF-8 텍스트/바이트
경계 함수로 제한합니다.

- `encode_utf8_text`
- `decode_utf8_text`
- `decode_utf8_text_lossy`

호출자가 텍스트와 binary encoder/decoder 사이를 이동할 때만 이 helper를
코덱에 둡니다. `bluetape-rs-core`의 문자열 유틸리티를 대체하지 않습니다.

## 기각한 대안

- 광범위한 text normalization helper: 문자열 유틸리티 범위입니다.
- Compression registry helper: `0.4.0` 범위입니다.
- serde/JSON/CBOR/MessagePack wrapper: `0.5.0` 범위입니다.
- Encryption, signing, checksum, database bind encoding: 향후 별도 package 경계로 분리합니다.

## 검증 메모

테스트는 비손실 UTF-8 거부와 명시적인 lossy opt-in을 모두 증명해야 합니다.
그래야 코덱 경계가 decode된 텍스트를 조용히 손상시키지 않습니다.
