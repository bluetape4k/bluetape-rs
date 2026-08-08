# 백엔드 라이브러리 실행 가능성

날짜: 2026-06-08
상태: repository-local summary
출처: [`bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md`](../../../bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md)

## 결정

`bluetape-rs`는 Rust 네이티브 백엔드 라이브러리군으로 실행 가능하지만
Kotlin/JVM `bluetape4k` API나 `bluetape-go`의 Go package shape를
복사해서는 안 됩니다.

repository는 Rust 고유의 가치에 집중해야 합니다.

- SQL, nullability, transaction scope, dialect capability를 위한 compile-time
  contract
- 명시적인 `Result` 및 `Option` 기반 failure와 absence 처리
- `Send` 및 `Sync`를 고려한 concurrency boundary
- 낮은 runtime overhead와 작은 native binary
- infrastructure-facing crate를 위한 Testcontainers-backed test

## 초기 제품 위치

| 기존 생태계 | 위치 |
|---|---|
| `bluetape4k` | Kotlin/Spring/JVM ecosystem integration 및 DSL productivity. |
| `bluetape-go` | 단순하고 명시적이며 operations에 적합한 backend primitive. |
| `bluetape-rs` | 강한 type-level contract, async-native infrastructure primitive, 작은 native deployable. |

## 권장 Crate Family

| 영역 | 작업 이름 | 범위 |
|---|---|---|
| Core | `bluetape-rs-core` | typed validation error, validation helper, string helper, small numeric check |
| Logging | `bluetape-rs-logging` | tracing setup helper, structured field, correlation ID, test capture, redaction convention |
| Testing | `bluetape-rs-test` | async assertion, deterministic fixture, temporary resource, 향후 Testcontainers boundary |
| Collections | `bluetape-rs-collections` | 집중형 iterator, slice, map, grouping, chunking, error-aware transform helper |
| Codec | `bluetape-rs-codec` | base encoder, hex, URL-safe codec, small binary/text codec helper |
| Compression | `bluetape-rs-compression` | opt-in compression helper와 registry-style codec selection |
| Serialization | `bluetape-rs-serde` | serde 호환 format을 위한 안전한 serializer/deserializer interface 및 test helper |
| Testcontainers | `bluetape-rs-testcontainers` | 명시적 feature 뒤의 PostgreSQL, Redis, MySQL, NATS, Kafka, emulator fixture helper |
| SQL | `bluetape-rs-sql` | SQL AST, dialect rendering, bind collection, typed query construction |
| SQLx | `bluetape-rs-sqlx` | SQLx executor, pool, transaction, migration, repository adapter |
| Resilience | `bluetape-rs-resilience` | retry, timeout, circuit breaker, bulkhead, backoff, service policy |
| Leader | `bluetape-rs-leader` | SQL 및 resilience foundation 이후의 Redis, RDB, etcd, Kubernetes Lease leader election |
| AWS | `bluetape-rs-aws` | 공식 AWS SDK for Rust를 감싸는 thin helper |
| Audit | `bluetape-rs-audit` | snapshot, diff, outbox, event-stream primitive |
| Graph | `bluetape-rs-graph` | Rust driver가 충분히 성숙한 경우의 graph model, bulk I/O, backend adapter |
| Text | `bluetape-rs-text` | Aho-Corasick search, blockword masking, tokenizer wrapper, language detection |
| Workshop | `bluetape-rs-workshop` | 실행 가능한 axum, Tokio, SQLx, Redis, AWS, graph, text example |

## SQL 방향

SQL DSL은 general helper, Testcontainers, testing foundation이 안정된 뒤
resilience보다 먼저 구현해야 합니다. `0.1.0`에는 포함하지 않습니다.

권장하는 첫 형태:

1. `bluetape-rs-sql-ast`: `Select`, `Insert`, `Update`, `Delete`, `Expr`,
   `Condition`, `Value`, `Bind`, and `Dialect`.
2. `bluetape-rs-sql-render`: Postgres/MySQL/SQLite renderers, bind placeholder policy,
   and identifier quoting.
3. `bluetape-rs-sql-schema`: optional table/column derive macro or declarative macro.
4. `bluetape-rs-sqlx`: SQLx `Executor`, `Pool`, and `Transaction` adapter.
5. `bluetape-rs-repository`: repository traits, pagination, optimistic locking, and
   transaction context.

MVP는 inspectable SQL AST와 SQLx execution adapter여야 합니다. lifecycle,
relation loading, migration, transaction semantic을 명시적으로 설계하고
test하기 전에는 full ORM support를 주장하지 않습니다.

## Capability 비고

| 영역 | 실행 가능성 | 권장 접근 |
|---|---:|---|
| SQL DSL/repository | 높음 | SQL AST + SQLx adapter + optional typed schema macro |
| Leader election | 높음 | 먼저 Redis owner-token lease, 이후 SQL row lease, etcd, Kubernetes Lease |
| AWS helper | 높음 | tracing 및 LocalStack example을 포함한 공식 AWS SDK for Rust thin wrapper |
| Audit/diff | 중간 | Serde snapshot, JSON Patch 또는 semantic diff, outbox/event stream primitive |
| Graph | 중간 | driver maturity가 허용되는 경우에만 core graph model과 backend adapter |
| Text | 중간-높음 | Aho-Corasick부터 시작한 뒤 tokenizer/language-detection quality gate 적용 |
| Workshop | 높음 | Spring/Ktor auto-configuration 대신 axum/Tokio/SQLx/Testcontainers example |

## 첫 Milestone

1. `0.1.0`: async assertion, `MultithreadingTester`,
   `SuspendedJobTester`, temporary resource를 포함한 workspace layout과
   `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-test`를
   만듭니다.
2. `0.2.0`: 집중형 collections 및 async/concurrency helper를 추가합니다.
3. `0.3.0`부터 `0.5.0`까지: codec, compression, serialization을
   분리합니다.
4. `0.6.0`: 명시적 Testcontainers fixture를 추가합니다.
5. `0.7.0`: resilience보다 먼저 relational SQL을 구현합니다.
6. `0.8.0`: resilience를 구현합니다.
7. `0.9.0`: Redis, RDB, etcd, Kubernetes Lease로 인해 multi-backend
   track이 커지므로 SQL과 resilience 이후에 leader election을 구현합니다.

## 위험

- macro-heavy API는 좋지 않은 compiler error UX를 만들 수 있습니다.
- dynamic query ergonomics와 compile-time proof는 서로 trade-off 관계입니다.
- graph database driver maturity는 JVM ecosystem보다 약합니다.
- Korean/Japanese tokenizer quality는 crate availability로 추정하지 말고
  local corpus로 측정해야 합니다.
- AWS SDK feature management와 compile time을 적극적으로 통제해야 합니다.

## Wiki Record의 외부 출처

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
