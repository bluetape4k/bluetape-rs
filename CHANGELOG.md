# Changelog

All notable changes to this project are documented in this file.

## Unreleased

### Added

- Added the opt-in `bluetape-rs-compression` crate with gzip, zlib, deflate,
  zstd, lz4, and snappy compressors behind additive feature flags.
- Added the optional root crate `compression` facade feature for callers that
  want compression helpers through `bluetape-rs`.
- Added same-condition benchmark fixture generation, raw Go capture, and a
  reproducible Rust benchmark runner for comparing `bluetape-rs`,
  `bluetape-go`, and `bluetape4k-io` compressors across JSON, text, binary,
  and random payloads.
- Added benchmark CSVs, a Markdown comparison report, and chart assets under
  `docs/benchmark` and `docs/images/readme-charts`.
- Added config-aware decompression limits, a 64 MiB default decode safety
  limit, `Read`/`Write` stream copy helpers, and direct stream reader/writer
  constructors to `bluetape-rs-compression`.

### Validation

- Pending final PR validation.

## [v0.3.1] - 2026-06-10

### Changed

- Corrected the `0.3.0` milestone release by aligning every workspace package
  to `0.3.1`.
- Published the root facade and all focused crates under one workspace release
  version:
  - `bluetape-rs`
  - `bluetape-rs-core`
  - `bluetape-rs-logging`
  - `bluetape-rs-test`
  - `bluetape-rs-collections`
  - `bluetape-rs-async`
  - `bluetape-rs-codec`
- Updated README installation examples so downstream users can depend on the
  workspace crates with a consistent `0.3.1` version.

### Release Correction

- `v0.3.0` remains immutable because `bluetape-rs-codec@0.3.0` was already
  published to crates.io.
- Use `v0.3.1` for the complete `0.3.x` workspace release.

### Validation

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `cargo publish --workspace --dry-run --locked`

## [v0.3.0] - 2026-06-10

Superseded by `v0.3.1` for the complete workspace release.

### Added

- Added `bluetape-rs-codec` `0.3.0` with strict hex, Base64 standard,
  Base64 URL-safe, Bitcoin Base58, byte-oriented Base62, and UTF-8 text
  boundary helpers.
- Added the optional root crate `codec` facade feature for callers that want
  codec helpers through `bluetape-rs`.
- Added typed decode errors for caller-owned invalid input, including
  position-aware hex and base-N failures and non-lossy UTF-8 text failures.
- Added public crate-boundary integration tests for hex, Base64, Base58,
  Base62, and UTF-8 text helpers.

### Changed

- Separated public codec tests under `crates/codec/tests/` while keeping
  source-local tests for the private shared base-N implementation.
- Confirmed compression remains deferred to `0.4.0` and serde-oriented
  serialization remains deferred to `0.5.0`.

### Validation

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test -p bluetape-rs-codec --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`
- `cargo publish -p bluetape-rs-codec --dry-run --locked`

## [v0.2.0] - 2026-06-10

### Added

- Added `bluetape-rs-collections` `0.2.0` with focused iterator, slice,
  map, pagination, grouping, chunking, and error-aware transform helpers.
- Added `bluetape-rs-async` `0.2.0` with Tokio-first bounded task execution,
  cancellation, timeout, deadline, and shutdown coordination helpers.
- Added deterministic async and concurrency tests for cancellation, dropped
  futures, join failures, bounded execution, and collection helper boundaries.
- Added README architecture diagrams and crate-level usage examples for the
  current collections and async/concurrency release line.

### Changed

- Split implementation modules away from large `lib.rs` files so future crate
  work follows Rust module conventions from the start.
- Hardened CI with pull-request checks, coverage reporting, clippy, rustdoc,
  and nightly workflow coverage.

### Validation

- `cargo fmt --all --check`
- `git diff --check`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- `cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`
- `cargo publish --workspace --dry-run --locked`

## [v0.1.1] - 2026-06-10

### Changed

- Enriched Rustdoc for foundation public APIs across `bluetape-rs-core`,
  `bluetape-rs-logging`, and `bluetape-rs-test`.
- Added compile-checked examples and explicit error contracts for validation,
  logging, async assertion, concurrency, and temporary directory helpers.
- Documented repository-local Rust ecosystem convention guidance for future
  contributors and agents.

### Validation

- `cargo fmt --all --check`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo doc --workspace --no-deps`
