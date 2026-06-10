# Changelog

All notable changes to this project are documented in this file.

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
