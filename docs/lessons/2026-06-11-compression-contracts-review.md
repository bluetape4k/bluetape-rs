# Compression Contracts Review Lessons

## Context

Issue #75 added the first public compression config, error, and stream contracts
for the 0.4.0 compression line.

## Lessons

- Safety-limit contracts must be allocation-aware. For lz4 and snappy one-shot
  decoders, check declared decompressed size before calling APIs that allocate
  the full output buffer.
- Public trait expansion should preserve existing implementor source
  compatibility when possible. Add default methods or split extension traits
  before adding required methods.
- Empty default feature sets need lint gates, not only tests. `default = []`
  should pass no-default clippy with `-D warnings`.
- Streaming contracts need failure-path tests. Inject failing `Read` and
  `Write` implementations to prove typed error variants and `source()`
  traversal, not just successful `Vec` round-trips.
- Rustdoc must carry wire-format caveats at the API discovery point. README
  warnings are not enough when registry enum variants expose both one-shot and
  stream helpers.

## Follow-up Guard

For future codec/compression APIs, include source-compatibility tests for public
traits and feature-matrix clippy in the local Step 6-R evidence before PR
creation.
