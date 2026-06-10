# Strict Hex Codec

## Context

Issue #54 is the first behavioral API in the `0.3.0` codec crate. It needed to
stay narrow enough to unblock Base64 and binary/text helpers without turning
`bluetape-rs-codec` into a broad utility bag.

## Decision

Keep strict hex as first-party code with four public items:

- `encode_hex_lower`
- `encode_hex_upper`
- `decode_hex`
- `HexDecodeError`

The decoder accepts only ASCII hexadecimal digits and rejects odd lengths before
scanning nibbles. Invalid characters report zero-based byte positions and the
invalid byte value.

## Rationale

- The implementation is small, allocation behavior is explicit, and no external
  dependency is needed.
- Byte-position errors are more useful for service diagnostics than broad string
  parse failures.
- Prefix handling belongs outside the strict decoder. Callers that accept `0x`
  or separators should normalize those inputs explicitly before calling
  `decode_hex`.

## Follow-Up

Base64 work in #55 can follow the same shape: small public functions, typed
decode errors, strict default behavior, and focused README/Rustdoc examples.
