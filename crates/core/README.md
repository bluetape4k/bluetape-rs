# bluetape-rs-core

Small Rust-native helpers shared by bluetape-rs crates.

The crate intentionally stays narrow. Prefer `std`, `Option`, and `Result`
combinators when they already express the operation clearly.

## Scope

- typed validation errors
- string validation and fallback helpers
- UTF-8 byte-boundary truncation
- checked numeric clamps and hex predicates
