# Changelog

All notable changes to this project are documented in this file.

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
