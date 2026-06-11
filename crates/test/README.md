# bluetape-rs-test

[English](README.md) | [한국어](README.ko.md)

Reusable test helpers for bluetape-rs crates.

![bluetape-rs-test crate overview](../../docs/images/readme-diagrams/bluetape-rs-test-crate.png)

This crate provides deterministic async assertions, `MultithreadingTester`,
`SuspendedJobTester`, and temporary resource cleanup helpers. Testcontainers
fixtures are intentionally deferred to the `bluetape-rs-testcontainers`
milestone.

## Usage

```toml
[dev-dependencies]
bluetape-rs-test = "0.4.0"
```

```rust
use bluetape_rs_test::TempDir;

let temp = TempDir::new("bluetape-rs-test").expect("temp dir");
assert!(temp.path().exists());
temp.close().expect("cleanup");
```
