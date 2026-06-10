# bluetape-rs-codec

Codec and encoding helpers for bluetape-rs.

This crate is part of the `0.3.1` workspace release. It provides strict
hexadecimal encoding and decoding primitives plus focused Base64, Base58,
Base62, and UTF-8 text/byte boundary helpers.

## Scope

- strict hex encoding and decoding
- Base64 standard and URL-safe variants
- Bitcoin Base58 byte encoding
- byte-oriented Base62 encoding
- typed errors for caller-owned invalid encoded input
- UTF-8 text/byte boundary helpers for codec call sites

## Out Of Scope

- compression helpers; those belong to `0.4.0`
- serde-oriented serialization interfaces; those belong to `0.5.0`
- encryption, signing, checksums, database bind encoding, random string helpers,
  and broad text normalization

## Usage

```toml
[dependencies]
bluetape-rs-codec = "0.3.1"
```

Or enable the optional root facade:

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

The decoder is intentionally strict. It accepts uppercase and lowercase ASCII
hexadecimal digits, but rejects odd-length input, prefixes such as `0x`,
whitespace, separators, and non-ASCII digits with typed errors that include the
input byte position when available.

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

Standard helpers use the `+` and `/` alphabet. URL-safe helpers use `-` and
`_`. Function names ending in `_unpadded` reject `=` padding during decode.

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

Base58 uses the Bitcoin alphabet
`123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz` and preserves
leading zero bytes as `1`. Base62 uses the bluetape alphabet
`0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz` and preserves
leading zero bytes as `0`. The current Base62 primitive is byte-oriented;
integer, UUID, and ID-generator rendering APIs remain separate higher-level
scope.

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

`decode_utf8_text` is non-lossy and returns a typed [`TextDecodeError`] with the
valid byte prefix when the decoded bytes are not valid UTF-8. Lossy replacement
is only available through the explicitly named `decode_utf8_text_lossy` helper.
General string utilities, normalization, compression registries, and
serde-oriented serialization stay outside this crate.

## Test Layout

Public codec behavior is tested from the crate boundary under `tests/`:

- `tests/hex.rs`
- `tests/base64.rs`
- `tests/base58.rs`
- `tests/base62.rs`
- `tests/text.rs`

Source-local tests are reserved for private implementation details, such as the
shared internal base-N converter.
