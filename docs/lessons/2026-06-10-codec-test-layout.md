# Codec Test Layout

Date: 2026-06-10
Issue: #57

## Decision

Keep public `bluetape-rs-codec` behavior tests in integration tests under
`crates/codec/tests/`:

- `base58.rs`
- `base62.rs`
- `base64.rs`
- `hex.rs`
- `text.rs`

This exercises the public crate boundary that downstream users call.

## Exception

Private implementation details can keep source-local tests. The shared
`base_n` converter remains crate-private, so its algorithm tests stay in
`crates/codec/src/base_n.rs`.

## Rejected

Moving every workspace test at once would create unrelated churn across crates.
For #57, the scope is the codec milestone test layout.
