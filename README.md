# bluetape-rs

[English](README.md) | [한국어](README.ko.md)

Rust backend primitives for the bluetape ecosystem.

`bluetape-rs` is a new WIP repository. It is not a port of the Kotlin/JVM
`bluetape4k` libraries and it is not a rewrite of `bluetape-go`. The goal is to
provide Rust-native building blocks for backend services where compile-time
contracts, small native binaries, explicit error handling, and deterministic
integration tests matter.

## Current Status

This repository is in bootstrap state.

The first useful work should stay narrow:

- define the crate layout and release policy
- prototype a SQL DSL/repository layer backed by SQLx
- prototype Redis/SQL leader election with fencing tokens
- add Testcontainers-backed PostgreSQL and Redis smoke tests
- keep all APIs Rust-native instead of copying Kotlin extension APIs or Go
  package shapes

## Intended Package Families

| Area | Working name | Purpose |
|---|---|---|
| Core | `bt-rs-core` | Errors, validation, IDs, time, configuration, and small typed helpers. |
| SQL | `bt-rs-sql` | SQL AST, dialect rendering, bind collection, typed query construction. |
| SQLx | `bt-rs-sqlx` | SQLx executor, pool, transaction, migration, and repository adapters. |
| Resilience | `bt-rs-resilience` | Retry, timeout, circuit breaker, bulkhead, backoff, and service policies. |
| Leader | `bt-rs-leader` | Redis, SQL, etcd, and Kubernetes Lease leader election. |
| AWS | `bt-rs-aws` | Thin helpers around the official AWS SDK for Rust. |
| Audit | `bt-rs-audit` | Snapshot, diff, outbox, and event-stream primitives inspired by audit workloads. |
| Graph | `bt-rs-graph` | Graph model, bulk I/O, and backend adapters where Rust drivers are mature enough. |
| Text | `bt-rs-text` | Aho-Corasick search, blockword masking, tokenizer wrappers, and language detection. |
| Workshop | `bt-rs-workshop` | Runnable axum, Tokio, SQLx, Redis, AWS, graph, and text examples. |

## Design Position

Rust should provide a different value proposition from the existing libraries:

- stronger type-level contracts for SQL, nullability, transaction scope, and
  dialect capabilities
- explicit `Result`/`Option` based failure and absence handling
- `Send`/`Sync` aware concurrency boundaries
- low runtime overhead and small deployable binaries
- Testcontainers-backed tests for infrastructure-facing packages

The SQL DSL is expected to be the first representative feature. The initial
shape should be an inspectable SQL AST plus SQLx execution adapter, not a full
ORM.

## Development

```bash
cargo fmt --all
cargo test --workspace
```

## Project Management

- [Current WIP](WIP.md)
- [Research index](docs/research/README.md)
- [Backend library feasibility](docs/research/2026-06-08-backend-library-feasibility.md)

## Project Rules

- Keep APIs idiomatic to Rust.
- Do not mechanically port Kotlin/JVM or Go APIs.
- Prefer focused crates over a catch-all utility package.
- Add real container-backed smoke tests before claiming infrastructure support.
- Keep public documentation in English and maintain Korean README parity.
