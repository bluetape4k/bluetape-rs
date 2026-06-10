# 0.2.0 Documentation Scope Review

## Scope

- Issue: #24
- Milestone: 0.2.0
- Changed surface: `README.md`, `README.ko.md`, `WIP.md`, `.github/workflows/ci.yml`

## 7-Tier Review

| Tier | Result | Evidence |
| --- | --- | --- |
| README parity | Pass | English and Korean README files describe the same 0.2.0 crate scope, examples, and deferred tracks. |
| WIP parity | Pass | WIP lists completed 0.2.0 child issues and actual implemented helper surfaces. |
| Examples | Pass | README snippets use exported APIs confirmed in source; workspace doctests passed for the embedded crate examples. |
| Out-of-scope clarity | Pass | Codec, compression, serialization, Testcontainers, SQL, resilience, and leader election remain deferred. |
| Runtime caveats | Pass | Async helper examples keep Tokio explicit and README points to focused crate usage. |
| Coverage reporting | Pass | CI now runs `cargo llvm-cov`, writes coverage to the GitHub step summary, and uploads `coverage-rust`. |
| CI trigger scope | Pass | `pull_request.paths-ignore` skips Markdown/docs-only changes while workflow/source changes still trigger CI. |
| Scope control | Pass | Documentation and CI reporting only; no Rust source behavior changes. |
| Risk | Low | Workflow adds one independent coverage job and does not change existing check/test/clippy/rustdoc jobs. |

## Findings

- P0: 0
- P1: 0
- P2: 0
- P3: 0

## Validation

- Pass: source alignment check for README APIs: `Page::with_meta`, `iter::chunks`, `try_map_bounded`, and `with_timeout`
- Pass: README image reference check
- Pass: local coverage report with `cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`
  - Workspace line coverage: 87.00% (1379/1585)
  - `bluetape-rs-collections`: 97.21% (349/359)
  - `bluetape-rs-async`: 88.15% (424/481)
- Pass: `actionlint .github/workflows/ci.yml`
- Pass: CI trigger review confirms Markdown/docs-only pull requests are ignored by `ci.yml`
- Pass: `cargo fmt --all --check`
- Pass: `cargo test --workspace --all-features --locked`
- Pass: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- Pass: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- Pass: `git diff --check`
