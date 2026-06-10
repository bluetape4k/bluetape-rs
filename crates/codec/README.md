# bluetape-rs-codec

Codec and encoding helpers for bluetape-rs.

This crate starts the `0.3.0` codec milestone with strict hexadecimal encoding
and decoding primitives plus focused Base64, Base58, and Base62 helpers. Small
binary/text helpers are tracked as separate follow-up issues.

## Scope

- strict hex encoding and decoding
- Base64 standard and URL-safe variants
- Bitcoin Base58 byte encoding
- byte-oriented Base62 encoding
- typed errors for caller-owned invalid encoded input
- small binary/text helpers when they make codec call sites clearer

## Out Of Scope

- compression helpers; those belong to `0.4.0`
- serde-oriented serialization interfaces; those belong to `0.5.0`
- encryption, signing, checksums, database bind encoding, and broad text
  normalization

## Usage

```toml
[dependencies]
bluetape-rs-codec = "0.3.0"
```

Or enable the optional root facade:

```toml
[dependencies]
bluetape-rs = { version = "0.1.1", features = ["codec"] }
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
