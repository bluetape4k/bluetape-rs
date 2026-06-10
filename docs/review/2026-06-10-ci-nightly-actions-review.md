# CI And Nightly Actions Review

## Scope

- Issue: #36
- Baseline: `origin/develop` at `926ca2c`
- Reviewed diff: `.github/workflows/ci.yml`, `.github/workflows/nightly-tests.yml`

## Reference Evidence

- `bluetape-go/.github/workflows/ci.yml`: compact CI, `develop`/`main`, concurrency, checkout/setup/cache/test flow.
- `bluetape-go/.github/workflows/nightly-tests.yml`: scheduled smoke/full planning, workflow dispatch scope, retry on heavier tests.
- `bluetape4k-projects/.github/workflows/ci.yml`: permissions, paths-ignore, concurrency, workflow dispatch, separate validation jobs.
- `exposed-workshop/.github/workflows/ci.yml`: smaller repo CI shape with paths-ignore and validation/test aggregation.
- `actionlint .github/workflows/ci.yml .github/workflows/nightly-tests.yml`: PASS
- `cargo fmt --all --check`: PASS
- `cargo check --workspace --all-targets --all-features --locked`: PASS on default Rust 1.96 and MSRV 1.85.0
- `cargo test --workspace --all-features --locked`: PASS on default Rust 1.96 and MSRV 1.85.0
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: PASS on default Rust 1.96 and MSRV 1.85.0
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked`: PASS on default Rust 1.96 and MSRV 1.85.0
- `git diff --check`: PASS

## Findings

- P0: none
- P1: none
- P2: none
- P3: none

## 7-Tier Review

| Tier | Result | Evidence |
| --- | --- | --- |
| Trigger contract | PASS | `push`/`pull_request` for `develop` and `main`, manual dispatch enabled |
| Permission boundary | PASS | `contents: read` only |
| Rust toolchain | PASS | `RUST_VERSION=1.85.0`, matching workspace MSRV |
| Validation coverage | PASS | fmt, check, test, clippy, rustdoc warnings |
| Nightly shape | PASS | smoke/full/docs scopes with scheduled daily smoke and weekly full |
| Dependency surface | PASS | official checkout/cache actions plus runner `rustup`; no extra Rust setup action |
| Local validation | PASS | actionlint, fmt, check, test, clippy, rustdoc, diff whitespace |

## Gate

P0=0 P1=0

Verdict: PASS.
