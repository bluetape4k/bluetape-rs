# Codec Text Boundaries

Date: 2026-06-10
Issue: #56

## Decision

Keep `bluetape-rs-codec` binary/text helpers limited to explicit UTF-8
text/byte boundary functions:

- `encode_utf8_text`
- `decode_utf8_text`
- `decode_utf8_text_lossy`

These helpers belong in codec only when a caller is moving between text and a
binary encoder/decoder. They are not a replacement for `bluetape-rs-core`
string utilities.

## Rejected

- Broad text normalization helpers: string utility scope.
- Compression registry helpers: `0.4.0`.
- serde/JSON/CBOR/MessagePack wrappers: `0.5.0`.
- Encryption, signing, checksums, and database bind encoding: separate future
  package boundaries.

## Validation Note

Tests must prove both non-lossy UTF-8 rejection and explicit lossy opt-in so the
codec boundary does not silently corrupt decoded text.
