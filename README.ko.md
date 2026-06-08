# bluetape-rs

[English](README.md) | [한국어](README.ko.md)

bluetape 생태계를 위한 Rust 백엔드 primitive 라이브러리입니다.

`bluetape-rs`는 새 WIP repository입니다. Kotlin/JVM 기반 `bluetape4k`의
포팅도 아니고, `bluetape-go`의 재작성도 아닙니다. 목표는 컴파일타임 계약, 작은
native binary, 명시적 오류 처리, 재현 가능한 integration test가 중요한 backend
서비스를 위해 Rust다운 building block을 제공하는 것입니다.

## 현재 상태

이 repository는 bootstrap 상태입니다.

첫 작업 범위는 작게 유지합니다.

- crate layout과 release policy 정의
- SQLx 기반 SQL DSL/repository layer prototype
- fencing token을 가진 Redis/SQL leader election prototype
- Testcontainers 기반 PostgreSQL/Redis smoke test 추가
- Kotlin extension API나 Go package shape를 복사하지 않는 Rust-native API 유지

## 계획 패키지군

| 영역 | 작업명 | 목적 |
|---|---|---|
| Core | `bt-rs-core` | Error, validation, ID, time, configuration, 작은 typed helper. |
| SQL | `bt-rs-sql` | SQL AST, dialect rendering, bind collection, typed query construction. |
| SQLx | `bt-rs-sqlx` | SQLx executor, pool, transaction, migration, repository adapter. |
| Resilience | `bt-rs-resilience` | Retry, timeout, circuit breaker, bulkhead, backoff, service policy. |
| Leader | `bt-rs-leader` | Redis, SQL, etcd, Kubernetes Lease 기반 leader election. |
| AWS | `bt-rs-aws` | 공식 AWS SDK for Rust 위의 얇은 helper. |
| Audit | `bt-rs-audit` | Audit workload를 위한 snapshot, diff, outbox, event-stream primitive. |
| Graph | `bt-rs-graph` | Graph model, bulk I/O, Rust driver가 성숙한 backend adapter. |
| Text | `bt-rs-text` | Aho-Corasick search, blockword masking, tokenizer wrapper, language detection. |
| Workshop | `bt-rs-workshop` | axum, Tokio, SQLx, Redis, AWS, graph, text runnable example. |

## 설계 포지션

Rust는 기존 라이브러리와 다른 가치를 제공해야 합니다.

- SQL, nullability, transaction scope, dialect capability를 더 강하게 타입으로 표현
- `Result`/`Option` 기반의 명시적 실패/없음 처리
- `Send`/`Sync`를 고려한 동시성 경계
- 낮은 runtime overhead와 작은 배포 binary
- infrastructure package는 Testcontainers 기반 실제 smoke test로 검증

첫 대표 기능은 SQL DSL이 적합합니다. 초기 형태는 full ORM이 아니라, inspect
가능한 SQL AST와 SQLx execution adapter가 되어야 합니다.

## 개발

```bash
cargo fmt --all
cargo test --workspace
```

## 프로젝트 관리

- [Current WIP](WIP.md)
- [Research note](../bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md)

## 프로젝트 원칙

- Rust에 자연스러운 API를 우선합니다.
- Kotlin/JVM 또는 Go API를 기계적으로 옮기지 않습니다.
- catch-all utility package보다 목적이 분명한 작은 crate를 선호합니다.
- 인프라 지원을 주장하기 전에 실제 container-backed smoke test를 추가합니다.
- 공개 문서는 영어를 기본으로 하고 Korean README parity를 유지합니다.
