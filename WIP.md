# bluetape-rs WIP

Status: bootstrap
Started: 2026-06-08

## Objective

Create a Rust-native backend library line for the bluetape ecosystem. The first
usable features should prove Rust-specific value instead of copying the Kotlin
or Go APIs.

## Initial Priorities

1. Define crate layout and contribution rules.
2. Prototype `bt-rs-sql` with SQL AST, dialect rendering, and bind collection.
3. Add `bt-rs-sqlx` adapter for PostgreSQL-backed smoke tests.
4. Prototype Redis and SQL leader election with owner tokens and fencing tokens.
5. Add a small axum/Tokio workshop example after the SQL and leader prototypes.

## Deferred

- Full ORM or DAO lifecycle.
- Spring Boot style auto-configuration.
- Broad AWS/graph/text/audit coverage before the SQL and leader foundations are
  proven.
- Public release automation.

## Evidence

- Repo-local feasibility research: `docs/research/2026-06-08-backend-library-feasibility.md`
- Source wiki record: `../bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md`
