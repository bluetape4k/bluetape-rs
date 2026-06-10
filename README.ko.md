# bluetape-rs

[English](README.md) | [한국어](README.ko.md)

bluetape 생태계를 위한 Rust 백엔드 primitive 라이브러리입니다.

![bluetape-rs workspace overview](docs/images/readme-diagrams/bluetape-rs-workspace-overview.png)

`bluetape-rs`는 새 WIP repository입니다. Kotlin/JVM 기반 `bluetape4k`의
포팅도 아니고, `bluetape-go`의 재작성도 아닙니다. 목표는 컴파일타임 계약, 작은
native binary, 명시적 오류 처리, 재현 가능한 integration test가 중요한 backend
서비스를 위해 Rust다운 building block을 제공하는 것입니다.

## 현재 상태

현재 package scope는 `0.2.0` collections 및 async/concurrency release line입니다.

첫 작업 범위는 작게 유지합니다.

- workspace layout과 release policy 정의
- typed validation error, string, 작은 numeric check를 위한 일반 helper 함수 추가
- library code에서 global subscriber를 강제하지 않는 logging/tracing 지원과 scoped
  test capture helper 추가
- async assertion, `MultithreadingTester`, `SuspendedJobTester`, temporary resource
  cleanup을 위한 재사용 test helper 추가
- `0.2.0` line을 위한 focused collection helper와 Tokio-first bounded task helper 추가
- Kotlin extension API나 Go package shape를 복사하지 않는 Rust-native API 유지

## 계획 패키지군

| 영역 | 작업명 | 목적 |
|---|---|---|
| Core | `bluetape-rs-core` | Typed validation error, validation helper, string helper, 작은 numeric check. |
| Logging | `bluetape-rs-logging` | Tracing setup helper, structured field, bounded correlation ID, scoped test capture. |
| Testing | `bluetape-rs-test` | Async assertion, `MultithreadingTester`, `SuspendedJobTester`, temporary resource, 향후 Testcontainers 경계. |
| Collections | `bluetape-rs-collections` | Iterator, slice, map, grouping, chunking, error-aware transform helper. |
| Async | `bluetape-rs-async` | Tokio-first bounded task execution, timeout/deadline, cancellation, shutdown helper. |
| Encoding | `bluetape-rs-codec` | Base encoder, hex, URL-safe codec, 작은 binary/text codec helper. |
| Compression | `bluetape-rs-compression` | Opt-in compression helper와 registry-style codec selection. |
| Serialization | `bluetape-rs-serde` | serde-compatible format을 위한 안전한 serializer/deserializer interface와 test helper. |
| Testcontainers | `bluetape-rs-testcontainers` | PostgreSQL, Redis, MySQL, NATS, Kafka, emulator fixture helper를 명시적 feature 뒤에 둡니다. |
| Leader | `bluetape-rs-leader` | Redis, SQL, etcd, Kubernetes Lease 기반 leader election. |
| SQL | `bluetape-rs-sql` | SQL AST, dialect rendering, bind collection, typed query construction. |
| SQLx | `bluetape-rs-sqlx` | SQLx executor, pool, transaction, migration, repository adapter. |
| Resilience | `bluetape-rs-resilience` | Retry, timeout, circuit breaker, bulkhead, backoff, service policy. |
| AWS | `bluetape-rs-aws` | 공식 AWS SDK for Rust 위의 얇은 helper. |
| Audit | `bluetape-rs-audit` | Audit workload를 위한 snapshot, diff, outbox, event-stream primitive. |
| Graph | `bluetape-rs-graph` | Graph model, bulk I/O, Rust driver가 성숙한 backend adapter. |
| Text | `bluetape-rs-text` | Aho-Corasick search, blockword masking, tokenizer wrapper, language detection. |
| Workshop | `bluetape-rs-workshop` | axum, Tokio, SQLx, Redis, AWS, graph, text runnable example. |

## 설계 포지션

Rust는 기존 라이브러리와 다른 가치를 제공해야 합니다.

- SQL, nullability, transaction scope, dialect capability를 더 강하게 타입으로 표현
- `Result`/`Option` 기반의 명시적 실패/없음 처리
- `Send`/`Sync`를 고려한 동시성 경계
- 낮은 runtime overhead와 작은 배포 binary
- infrastructure package는 Testcontainers 기반 실제 smoke test로 검증

첫 release는 의도적으로 지루하고 넓게 재사용 가능한 helper, logging, test
support에 집중합니다. Codec, compression, serialization, Testcontainers, leader
election은 별도 milestone으로 나눕니다. Relational SQL은 Testcontainers 뒤,
resilience 앞에 구현합니다. Leader election은 Redis, RDB, etcd, Kubernetes Lease
등 backend matrix가 큰 작업이므로 SQL과 resilience 뒤에 배치합니다. SQL을
시작할 때도 초기 형태는 full ORM이 아니라 inspect 가능한 SQL AST와 SQLx
execution adapter가 되어야 합니다.

## 사용 방법

Root facade는 기본으로 `core`만 활성화합니다. `logging`, `test` facade
module은 feature를 명시적으로 켜거나, 필요한 focused crate에 직접 의존합니다.

```toml
[dependencies]
bluetape-rs = { version = "0.1.1", features = ["logging"] }

[dev-dependencies]
bluetape-rs = { version = "0.1.1", features = ["test"] }
```

```rust
use bluetape_rs::{core, logging};
```

Focused crate는 import name에서 hyphen 대신 underscore를 사용합니다.

```toml
[dependencies]
bluetape-rs-core = "0.1.1"
bluetape-rs-logging = "0.1.1"

[dev-dependencies]
bluetape-rs-test = "0.1.1"
```

```rust
use bluetape_rs_core::require_not_blank;
use bluetape_rs_logging::CorrelationId;
use bluetape_rs_test::TempDir;
```

Tokio task helper를 사용할 때:

```toml
[dependencies]
bluetape-rs-async = "0.2.0"
```

```rust
use bluetape_rs_async::{try_map_bounded, with_timeout};
```

## 개발

```bash
cargo fmt --all
cargo test --workspace
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## 프로젝트 관리

- [Current WIP](WIP.md)
- [Research index](docs/research/README.md)
- [Backend library feasibility](docs/research/2026-06-08-backend-library-feasibility.md)

## 프로젝트 원칙

- Rust에 자연스러운 API를 우선합니다.
- Kotlin/JVM 또는 Go API를 기계적으로 옮기지 않습니다.
- catch-all utility package보다 목적이 분명한 작은 crate를 선호합니다.
- 인프라 지원을 주장하기 전에 실제 container-backed smoke test를 추가합니다.
- 공개 문서는 영어를 기본으로 하고 Korean README parity를 유지합니다.
