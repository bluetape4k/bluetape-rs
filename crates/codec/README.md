# bluetape-rs-codec

Codec and encoding helpers for bluetape-rs.

This crate starts the `0.3.0` codec milestone with strict hexadecimal encoding
and decoding primitives. Base64 and URL-safe helpers are planned as focused
follow-up work.

## Scope

- strict hex encoding and decoding
- Base64 standard and URL-safe variants
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
