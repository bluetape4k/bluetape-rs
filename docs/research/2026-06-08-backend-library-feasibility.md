# Backend library feasibility

Date: 2026-06-08
Status: repo-local summary
Source: [`bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md`](../../../bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md)

## Decision

`bluetape-rs` is feasible as a Rust-native backend library line, but it should
not copy the Kotlin/JVM `bluetape4k` APIs or the Go package shapes from
`bluetape-go`.

The repo should focus on Rust-specific value:

- compile-time contracts for SQL, nullability, transaction scope, and dialect
  capability
- explicit `Result` and `Option` based failure and absence handling
- `Send` and `Sync` aware concurrency boundaries
- low runtime overhead and small native binaries
- Testcontainers-backed tests for infrastructure-facing crates

## Initial Product Position

| Existing line | Position |
|---|---|
| `bluetape4k` | Kotlin/Spring/JVM ecosystem integration and DSL productivity. |
| `bluetape-go` | Simple, explicit, operations-friendly backend primitives. |
| `bluetape-rs` | Strong type-level contracts, async-native infrastructure primitives, and small native deployables. |

## Recommended Crate Families

| Area | Working name | Scope |
|---|---|---|
| Core | `bluetape-rs-core` | Typed validation errors, validation helpers, string helpers, and small numeric checks. |
| Logging | `bluetape-rs-logging` | Tracing setup helpers, structured fields, correlation IDs, test capture, and redaction conventions. |
| Testing | `bluetape-rs-test` | Async assertions, deterministic fixtures, temporary resources, and future Testcontainers boundaries. |
| Collections | `bluetape-rs-collections` | Focused iterator, slice, map, grouping, chunking, and error-aware transform helpers. |
| Codec | `bluetape-rs-codec` | Base encoders, hex, URL-safe codecs, and small binary/text codec helpers. |
| Compression | `bluetape-rs-compression` | Opt-in compression helpers and registry-style codec selection. |
| Serialization | `bluetape-rs-serde` | Safe serializer/deserializer interfaces and test helpers around serde-compatible formats. |
| Testcontainers | `bluetape-rs-testcontainers` | PostgreSQL, Redis, MySQL, NATS, Kafka, and emulator fixture helpers behind explicit features. |
| SQL | `bluetape-rs-sql` | SQL AST, dialect rendering, bind collection, typed query construction. |
| SQLx | `bluetape-rs-sqlx` | SQLx executor, pool, transaction, migration, and repository adapters. |
| Resilience | `bluetape-rs-resilience` | Retry, timeout, circuit breaker, bulkhead, backoff, and service policies. |
| Leader | `bluetape-rs-leader` | Redis, RDB, etcd, and Kubernetes Lease leader election after SQL and resilience foundations. |
| AWS | `bluetape-rs-aws` | Thin helpers around the official AWS SDK for Rust. |
| Audit | `bluetape-rs-audit` | Snapshot, diff, outbox, and event-stream primitives. |
| Graph | `bluetape-rs-graph` | Graph model, bulk I/O, and backend adapters where Rust drivers are mature enough. |
| Text | `bluetape-rs-text` | Aho-Corasick search, blockword masking, tokenizer wrappers, and language detection. |
| Workshop | `bluetape-rs-workshop` | Runnable axum, Tokio, SQLx, Redis, AWS, graph, and text examples. |

## SQL Direction

The SQL DSL should be implemented after the general helper, Testcontainers, and
testing foundations are stable, and before resilience. It should not be part of
`0.1.0`.

Recommended first shape:

1. `bluetape-rs-sql-ast`: `Select`, `Insert`, `Update`, `Delete`, `Expr`,
   `Condition`, `Value`, `Bind`, and `Dialect`.
2. `bluetape-rs-sql-render`: Postgres/MySQL/SQLite renderers, bind placeholder policy,
   and identifier quoting.
3. `bluetape-rs-sql-schema`: optional table/column derive macro or declarative macro.
4. `bluetape-rs-sqlx`: SQLx `Executor`, `Pool`, and `Transaction` adapter.
5. `bluetape-rs-repository`: repository traits, pagination, optimistic locking, and
   transaction context.

The MVP should be an inspectable SQL AST plus SQLx execution adapter. It should
not claim full ORM support until lifecycle, relation loading, migrations, and
transaction semantics are explicitly designed and tested.

## Capability Notes

| Area | Feasibility | Recommended approach |
|---|---:|---|
| SQL DSL/repository | High | SQL AST + SQLx adapter + optional typed schema macros. |
| Leader election | High | Redis owner-token lease first, then SQL row lease, etcd, and Kubernetes Lease. |
| AWS helpers | High | Thin wrappers over the official AWS SDK for Rust with tracing and LocalStack examples. |
| Audit/diff | Medium | Serde snapshots, JSON Patch or semantic diff, outbox/event stream primitives. |
| Graph | Medium | Core graph model and backend adapters only where driver maturity is acceptable. |
| Text | Medium-high | Aho-Corasick first, then tokenizer/language-detection quality gates. |
| Workshop | High | axum/Tokio/SQLx/Testcontainers examples rather than Spring/Ktor auto-configuration. |

## First Milestones

1. `0.1.0`: create workspace layout plus `bluetape-rs-core`,
   `bluetape-rs-logging`, and `bluetape-rs-test` with async assertions,
   `MultithreadingTester`, `SuspendedJobTester`, and temporary resources.
2. `0.2.0`: add focused collections and async/concurrency helpers.
3. `0.3.0` through `0.5.0`: split codec, compression, and serialization.
4. `0.6.0`: add explicit Testcontainers fixtures.
5. `0.7.0`: implement relational SQL before resilience.
6. `0.8.0`: implement resilience.
7. `0.9.0`: implement leader election after SQL and resilience because Redis,
   RDB, etcd, and Kubernetes Lease make it a larger multi-backend track.

## Risks

- Macro-heavy APIs can create poor compiler error UX.
- Dynamic query ergonomics and compile-time proof trade off against each other.
- Graph database driver maturity is weaker than the JVM ecosystem.
- Korean/Japanese tokenizer quality must be measured against a local corpus, not
  inferred from crate availability.
- AWS SDK feature management and compile time need active control.

## External Sources From The Wiki Record

- AWS SDK for Rust: https://aws.amazon.com/sdk-for-rust/
- Tokio: https://tokio.rs/
- axum: https://docs.rs/axum/latest/axum/
- Tower: https://docs.rs/tower/
- SQLx: https://github.com/launchbadge/sqlx
- Diesel: https://docs.diesel.rs/main/diesel/
- SeaQuery: https://docs.rs/sea-query/latest/sea_query/
- Kubernetes Lease: https://kubernetes.io/docs/concepts/architecture/leases/
- Serde: https://serde.rs/
- Testcontainers for Rust: https://rust.testcontainers.org/
