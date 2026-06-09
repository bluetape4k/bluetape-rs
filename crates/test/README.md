# bluetape-rs-test

Reusable test helpers for bluetape-rs crates.

This crate provides deterministic async assertions, `MultithreadingTester`,
`SuspendedJobTester`, and temporary resource cleanup helpers. Testcontainers
fixtures are intentionally deferred to the `bluetape-rs-testcontainers`
milestone.
