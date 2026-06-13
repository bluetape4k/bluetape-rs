# Lesson: Serialization Contracts

Date: 2026-06-13
Scope: issue #109, serialization format/error/metadata/config contracts

## What Changed

- Defined Rust-native serialization traits around caller-supplied `serde`
  types without choosing JSON, Protobuf, Avro, Fory, or a concrete binary
  adapter in this issue.
- Added typed format, content type, adapter id, payload version, trust profile,
  metadata, metadata policy, config, and error contracts.
- Kept payload bytes and metadata together while keeping diagnostics payload-free
  by default.
- Preserved adapter dependency boundaries: the contract crate uses `serde` and
  `thiserror`, but no concrete adapter crates.

## Lessons

- Public metadata structs should not expose all fields on the first contract
  release. Private fields plus typed accessors leave room for metadata growth
  without locking callers into struct literals.
- Payload containers should not derive `Debug` when they may hold cache payloads.
  Custom debug output should expose lengths and metadata, not raw bytes.
- Trust-profile vocabulary is not enough; safe setters must reject unsafe legacy
  modes, and migration-only opt-ins must be deliberately named.
- README code fences and Rustdoc examples have different needs. Hidden Rustdoc
  markers are fine in Rustdoc, but normal README examples should read as normal
  code.

## Checks That Caught The Gaps

- Step 6-R architecture review caught the public-field metadata shape before PR.
- Step 6-R security review caught the raw payload debug risk.
- Step 6-R library-user review caught README snippet quality and config setter
  ergonomics.
- Step 6-R verifier review caught the missing `# Errors` Rustdoc section for a
  public `Result` API.

## Forward Rule

For future serialization adapters, keep each adapter behind a separate milestone
and require the same scenario matrix: metadata mismatch, version mismatch,
trust-profile mismatch, oversized payload, malformed bytes, safe vs redacted
adapter failure, README parity, and benchmark evidence when performance is
claimed.
