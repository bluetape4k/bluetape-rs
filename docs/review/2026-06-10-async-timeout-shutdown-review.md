# Async Timeout And Shutdown Review

## Scope

- Issue: #22
- Milestone: 0.2.0
- Changed surface: `bluetape-rs-async`
- External reference: Tokio 1.49 documentation for `tokio::time::timeout`,
  `timeout_at`, `watch`, and `select!` cancellation patterns

## 7-Tier Review

| Tier | Result | Evidence |
| --- | --- | --- |
| API contract | Pass | `AsyncControlError` distinguishes timeout from cancellation; timeout/deadline helpers return typed errors. |
| Cancellation behavior | Pass | `run_until_cancelled` and `with_timeout_or_cancel` use caller-owned tokens and do not convert dropped wrapper futures into synthetic errors. |
| Cleanup | Pass | Tests prove cancellation drops the in-flight future and shutdown listeners are notified. |
| Runtime boundary | Pass | Crate README documents Tokio assumptions and excludes blocking work on core async tasks. |
| Documentation | Pass | Public Rustdoc and README describe timeout, deadline, cancellation, and shutdown scope. |
| Tests | Pass | Unit tests cover success, timeout, deadline, cancellation, timeout-vs-cancel precedence, and shutdown notification. |
| Risk | Moderate | Extends the new async crate but does not change default root facade features. |

## Findings

- P0: 0
- P1: 0
- P2: 0
- P3: 0

## Validation

- Pass: `cargo fmt --all --check`
- Pass: `cargo check --workspace --all-targets --all-features --locked`
- Pass: `cargo test -p bluetape-rs-async`
- Pass: `cargo test --workspace --all-features --locked`
- Pass: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- Pass: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- Pass: `git diff --check`
