# Strict Hex 코덱

## 배경

이슈 #54는 `0.3.0` 코덱 crate의 첫 동작 API입니다. Base64와 binary/text
helper의 착수를 막지 않으면서 `bluetape-rs-codec`가 광범위한 유틸리티
모음으로 변하지 않도록 범위를 좁게 유지해야 했습니다.

## 결정

strict hex는 다음 네 가지 공개 항목을 갖는 first-party code로 유지합니다.

- `encode_hex_lower`
- `encode_hex_upper`
- `decode_hex`
- `HexDecodeError`

Decoder는 ASCII hexadecimal digit만 허용하고 nibble을 순회하기 전에 홀수
길이를 거부합니다. 잘못된 문자는 0부터 시작하는 바이트 위치와 잘못된
바이트 값을 보고합니다.

## 근거

- 구현이 작고 allocation 동작이 명시적이며 외부 dependency가 필요하지 않습니다.
- 바이트 위치 error는 광범위한 문자열 parse failure보다 service 진단에 유용합니다.
- Prefix 처리는 strict decoder 외부에 둡니다. `0x` 또는 separator를 허용하는 호출자는 `decode_hex`를 호출하기 전에 입력을 명시적으로 정규화해야 합니다.

## 후속 작업

#55의 Base64 작업도 작은 공개 함수, typed decode error, strict 기본 동작,
집중된 README/Rustdoc 예시라는 동일한 형태를 따를 수 있습니다.
