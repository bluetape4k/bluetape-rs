# bluetape-rs-codec

[English](README.md) | [한국어](README.ko.md)

bluetape-rs를 위한 codec 및 encoding helper입니다.

이 crate는 `0.3.1` workspace release에 포함됩니다. Strict hexadecimal
encoding/decoding primitive와 focused Base64, Base58, Base62, UTF-8 text/byte
boundary helper를 제공합니다.

## 범위

- strict hex encoding 및 decoding
- Base64 standard 및 URL-safe variant
- Bitcoin Base58 byte encoding
- byte-oriented Base62 encoding
- caller-owned invalid encoded input을 위한 typed error
- codec call site를 위한 UTF-8 text/byte boundary helper

## 범위 밖

- compression helper: `0.4.0` 범위
- serde-oriented serialization interface: `0.5.0` 범위
- encryption, signing, checksum, database bind encoding, random string helper,
  broad text normalization

## 사용 예

```toml
[dependencies]
bluetape-rs-codec = "0.3.1"
```

또는 optional root facade를 활성화합니다.

```toml
[dependencies]
bluetape-rs = { version = "0.3.1", features = ["codec"] }
```

## Hex

```rust
use bluetape_rs_codec::{decode_hex, encode_hex_lower, encode_hex_upper};

let bytes = [0x00, 0xab, 0xff];

assert_eq!(encode_hex_lower(bytes), "00abff");
assert_eq!(encode_hex_upper(bytes), "00ABFF");
assert_eq!(
    decode_hex("00abFF").expect("valid hex"),
    vec![0x00, 0xab, 0xff]
);
```

Decoder는 의도적으로 strict합니다. Upper/lowercase ASCII hexadecimal digit은
허용하지만, odd-length input, `0x` prefix, whitespace, separator, non-ASCII digit은
거부합니다. 가능한 경우 typed error에 input byte position을 포함합니다.

## Base64

```rust
use bluetape_rs_codec::{
    decode_base64, decode_base64_unpadded, decode_base64_url, decode_base64_url_unpadded,
    encode_base64, encode_base64_unpadded, encode_base64_url, encode_base64_url_unpadded,
};

assert_eq!(encode_base64(b"fo"), "Zm8=");
assert_eq!(decode_base64("Zm8=").expect("valid Base64"), b"fo");

assert_eq!(encode_base64_unpadded(b"fo"), "Zm8");
assert_eq!(decode_base64_unpadded("Zm8").expect("valid Base64"), b"fo");

assert_eq!(encode_base64_url([0xfb, 0xff]), "-_8=");
assert_eq!(decode_base64_url("-_8=").expect("valid URL-safe Base64"), vec![0xfb, 0xff]);

assert_eq!(encode_base64_url_unpadded([0xfb, 0xff]), "-_8");
assert_eq!(
    decode_base64_url_unpadded("-_8").expect("valid URL-safe Base64"),
    vec![0xfb, 0xff]
);
```

Standard helper는 `+`와 `/` alphabet을 사용하고, URL-safe helper는 `-`와 `_`를
사용합니다. 이름이 `_unpadded`로 끝나는 함수는 decode 시 `=` padding을 거부합니다.

## Base58 And Base62

```rust
use bluetape_rs_codec::{decode_base58, decode_base62, encode_base58, encode_base62};

assert_eq!(encode_base58(b"Hello, World!"), "72k1xXWG59fYdzSNoA");
assert_eq!(
    decode_base58("72k1xXWG59fYdzSNoA").expect("valid Base58"),
    b"Hello, World!"
);

assert_eq!(encode_base62(b"Hello, World!"), "1wJfrzvdbtXUOlUjUf");
assert_eq!(
    decode_base62("1wJfrzvdbtXUOlUjUf").expect("valid Base62"),
    b"Hello, World!"
);
```

Base58은 Bitcoin alphabet
`123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`를 사용하며 leading
zero byte를 `1`로 보존합니다. Base62는 bluetape alphabet
`0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`를 사용하며 leading
zero byte를 `0`으로 보존합니다. 현재 Base62 primitive는 byte-oriented입니다.
Integer, UUID, ID-generator rendering API는 별도 higher-level scope로 남깁니다.

## UTF-8 Text Boundaries

```rust
use bluetape_rs_codec::{
    decode_base64_url_unpadded, decode_utf8_text, decode_utf8_text_lossy,
    encode_base64_url_unpadded, encode_utf8_text,
};

let token = encode_base64_url_unpadded(encode_utf8_text("blue테이프"));
assert_eq!(token, "Ymx1Ze2FjOydtO2UhA");

let bytes = decode_base64_url_unpadded(token).expect("valid URL-safe Base64");
assert_eq!(decode_utf8_text(bytes).expect("valid UTF-8"), "blue테이프");

assert_eq!(decode_utf8_text_lossy([b'a', 0xff, b'z']), "a\u{fffd}z");
```

`decode_utf8_text`는 lossy 변환을 하지 않으며 decoded byte가 valid UTF-8이 아니면
typed [`TextDecodeError`]를 반환합니다. Lossy replacement는 명시적으로 이름 붙인
`decode_utf8_text_lossy` helper에서만 사용할 수 있습니다. General string utility,
normalization, compression registry, serde-oriented serialization은 이 crate 밖에
둡니다.

## Test Layout

Public codec behavior는 crate boundary의 `tests/`에서 검증합니다.

- `tests/hex.rs`
- `tests/base64.rs`
- `tests/base58.rs`
- `tests/base62.rs`
- `tests/text.rs`

Source-local test는 shared internal base-N converter 같은 private implementation
detail에만 사용합니다.
