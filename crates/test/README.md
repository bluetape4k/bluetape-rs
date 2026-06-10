# bluetape-rs-test

Reusable test helpers for bluetape-rs crates.

This crate provides deterministic async assertions, `MultithreadingTester`,
`SuspendedJobTester`, and temporary resource cleanup helpers. Testcontainers
fixtures are intentionally deferred to the `bluetape-rs-testcontainers`
milestone.

## Usage

```toml
[dev-dependencies]
bluetape-rs-test = "0.1.0"
```

```rust
use bluetape_rs_test::TempDir;

let temp = TempDir::new("bluetape-rs-test").expect("temp dir");
assert!(temp.path().exists());
temp.close().expect("cleanup");
```
