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
| Core | `bt-rs-core` | Errors, validation, IDs, time, configuration, and small typed helpers. |
| SQL | `bt-rs-sql` | SQL AST, dialect rendering, bind collection, typed query construction. |
| SQLx | `bt-rs-sqlx` | SQLx executor, pool, transaction, migration, and repository adapters. |
| Resilience | `bt-rs-resilience` | Retry, timeout, circuit breaker, bulkhead, backoff, and service policies. |
| Leader | `bt-rs-leader` | Redis, SQL, etcd, and Kubernetes Lease leader election. |
| AWS | `bt-rs-aws` | Thin helpers around the official AWS SDK for Rust. |
| Audit | `bt-rs-audit` | Snapshot, diff, outbox, and event-stream primitives. |
| Graph | `bt-rs-graph` | Graph model, bulk I/O, and backend adapters where Rust drivers are mature enough. |
| Text | `bt-rs-text` | Aho-Corasick search, blockword masking, tokenizer wrappers, and language detection. |
| Workshop | `bt-rs-workshop` | Runnable axum, Tokio, SQLx, Redis, AWS, graph, and text examples. |

## SQL Direction

The SQL DSL should be the first representative feature.

Recommended first shape:

1. `bt-rs-sql-ast`: `Select`, `Insert`, `Update`, `Delete`, `Expr`, `Condition`,
   `Value`, `Bind`, and `Dialect`.
2. `bt-rs-sql-render`: Postgres/MySQL/SQLite renderers, bind placeholder policy,
   and identifier quoting.
3. `bt-rs-sql-schema`: optional table/column derive macro or declarative macro.
4. `bt-rs-sqlx`: SQLx `Executor`, `Pool`, and `Transaction` adapter.
5. `bt-rs-repository`: repository traits, pagination, optimistic locking, and
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

1. Create crate layout and contribution rules.
2. Prototype SQL AST rendering and bind collection.
3. Add SQLx PostgreSQL execution and Testcontainers smoke tests.
4. Prototype Redis and SQL leader election with fencing tokens.
5. Add a small axum/Tokio workshop example after the SQL and leader prototypes.

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
