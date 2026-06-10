# Codec Test Layout Review

Date: 2026-06-10
Issue: #57
Branch: `feat/issue-57-codec-test-separation`

## Scope

Separate public `bluetape-rs-codec` tests from source modules:

- move hex public API tests to `crates/codec/tests/hex.rs`
- move Base64 public API tests to `crates/codec/tests/base64.rs`
- move Base58 public API and stress tests to `crates/codec/tests/base58.rs`
- move Base62 public API and stress tests to `crates/codec/tests/base62.rs`
- keep UTF-8 text public API tests in `crates/codec/tests/text.rs`
- keep private `base_n` tests source-local

## 7-Tier Review

| Tier | Verdict | Evidence |
|---|---|---|
| 1. Public API / Contract | PASS | No public API signatures changed; tests now import through `bluetape_rs_codec`. |
| 2. Architecture / Boundary | PASS | Public behavior tests moved to integration tests; private `base_n` algorithm tests remain source-local. |
| 3. Rust API Shape | PASS | No API shape changes. Test files exercise owned outputs and typed errors through the crate boundary. |
| 4. Tests | PASS | Codec validation reports 3 source unit tests, 44 integration tests, and 18 doctests. |
| 5. Static / Docs | PASS | README.md, README.ko.md, and `crates/codec/README.md` document the test layout. |
| 6. Release / Cargo | PASS | No dependency or Cargo metadata changes. |
| 7. Evidence Integrity | PASS | Code-review-graph context reported low risk, 7 source/doc changed files, 0 impacted nodes, and 0 test gaps against `origin/develop`. Full workspace validation passed after the test move. |

## P0/P1 Gate

P0=0 P1=0

## Validation

- `cargo test -p bluetape-rs-codec --all-features --locked`: PASS, 3 unit tests + 44 integration tests + 18 doctests
- `git diff --check`: PASS
- `cargo fmt --all --check`: PASS
- `cargo test --workspace --all-features --locked`: PASS
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: PASS
