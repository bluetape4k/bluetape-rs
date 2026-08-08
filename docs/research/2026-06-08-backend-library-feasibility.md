# 백엔드 라이브러리 실현 가능성

날짜: 2026-06-08
상태: 저장소 로컬 요약
출처: [`bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md`](../../../bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md)

## 결정

`bluetape-rs`는 Rust 네이티브 백엔드 라이브러리 계열로 실현 가능하지만,
Kotlin/JVM `bluetape4k` API나 `bluetape-go`의 Go 패키지 형태를 복사해서는 안
된다.

이 저장소는 Rust 고유의 가치에 집중해야 한다.

- SQL, null 가능성, 트랜잭션 범위 및 방언 기능에 대한 컴파일 타임 계약
- 명시적인 `Result` 및 `Option` 기반 실패·부재 처리
- `Send` 및 `Sync`를 고려한 동시성 경계
- 낮은 런타임 오버헤드와 작은 네이티브 바이너리
- 인프라 연동 크레이트를 위한 Testcontainers 기반 테스트

## 초기 제품 포지션

| 기존 계열 | 포지션 |
|---|---|
| `bluetape4k` | Kotlin/Spring/JVM 생태계 통합과 DSL 생산성. |
| `bluetape-go` | 단순하고 명시적이며 운영에 친화적인 백엔드 프리미티브. |
| `bluetape-rs` | 강한 타입 수준 계약, async 네이티브 인프라 프리미티브 및 작은 네이티브 배포물. |

## 권장 크레이트 계열

| 영역 | 작업 이름 | 범위 |
|---|---|---|
| Core | `bluetape-rs-core` | 타입 지정 검증 오류, 검증 도우미, 문자열 도우미 및 소규모 수치 확인. |
| Logging | `bluetape-rs-logging` | Tracing 설정 도우미, 구조화 필드, 상관관계 ID, 테스트 캡처 및 비식별화 규칙. |
| Testing | `bluetape-rs-test` | 비동기 assertion, 결정론적 fixture, 임시 리소스 및 향후 Testcontainers 경계. |
| Collections | `bluetape-rs-collections` | 집중형 iterator, slice, map, grouping, chunking 및 오류 인지 변환 도우미. |
| Codec | `bluetape-rs-codec` | Base 인코더, hex, URL-safe 코덱 및 소규모 바이너리/텍스트 코덱 도우미. |
| Compression | `bluetape-rs-compression` | 옵트인 압축 도우미와 레지스트리 방식 코덱 선택. |
| Serialization | `bluetape-rs-serde` | 안전한 serializer/deserializer 인터페이스와 serde 호환 형식용 테스트 도우미. |
| Testcontainers | `bluetape-rs-testcontainers` | 명시적 feature 뒤에 PostgreSQL, Redis, MySQL, NATS, Kafka 및 emulator fixture 도우미를 제공. |
| SQL | `bluetape-rs-sql` | SQL AST, 방언 렌더링, bind 수집 및 타입 지정 쿼리 구성. |
| SQLx | `bluetape-rs-sqlx` | SQLx executor, pool, transaction, migration 및 repository 어댑터. |
| Resilience | `bluetape-rs-resilience` | 재시도, timeout, circuit breaker, bulkhead, backoff 및 서비스 정책. |
| Leader | `bluetape-rs-leader` | SQL 및 resilience 기반 이후 Redis, RDB, etcd 및 Kubernetes Lease 리더 선출. |
| AWS | `bluetape-rs-aws` | 공식 AWS SDK for Rust를 감싸는 얇은 도우미. |
| Audit | `bluetape-rs-audit` | Snapshot, diff, outbox 및 이벤트 스트림 프리미티브. |
| Graph | `bluetape-rs-graph` | Rust 드라이버가 충분히 성숙한 경우의 그래프 모델, bulk I/O 및 백엔드 어댑터. |
| Text | `bluetape-rs-text` | Aho-Corasick 검색, 금칙어 마스킹, tokenizer 래퍼 및 언어 감지. |
| Workshop | `bluetape-rs-workshop` | 실행 가능한 axum, Tokio, SQLx, Redis, AWS, graph 및 text 예제. |

## SQL 방향

SQL DSL은 일반 도우미, Testcontainers 및 테스트 기반이 안정화된 뒤,
resilience보다 먼저 구현해야 한다. `0.1.0`에 포함해서는 안 된다.

권장하는 첫 형태:

1. `bluetape-rs-sql-ast`: `Select`, `Insert`, `Update`, `Delete`, `Expr`,
   `Condition`, `Value`, `Bind` 및 `Dialect`.
2. `bluetape-rs-sql-render`: Postgres/MySQL/SQLite renderer, bind placeholder 정책
   및 identifier quoting.
3. `bluetape-rs-sql-schema`: 선택적인 table/column derive macro 또는 declarative macro.
4. `bluetape-rs-sqlx`: SQLx `Executor`, `Pool` 및 `Transaction` adapter.
5. `bluetape-rs-repository`: repository trait, pagination, optimistic locking 및
   트랜잭션 컨텍스트.

MVP는 검사 가능한 SQL AST와 SQLx 실행 어댑터여야 한다. lifecycle, relation
loading, migrations 및 transaction semantics를 명시적으로 설계하고 테스트하기
전에는 완전한 ORM 지원을 주장해서는 안 된다.

## 기능 메모

| 영역 | 실현 가능성 | 권장 접근 |
|---|---:|---|
| SQL DSL/repository | 높음 | SQL AST + SQLx adapter + 선택적 타입 지정 schema macro. |
| Leader election | 높음 | Redis owner-token lease를 먼저 구현한 뒤 SQL row lease, etcd 및 Kubernetes Lease로 확장. |
| AWS helpers | 높음 | tracing 및 LocalStack 예제를 포함한 공식 AWS SDK for Rust 래퍼. |
| Audit/diff | 중간 | Serde snapshot, JSON Patch 또는 semantic diff, outbox/event stream 프리미티브. |
| Graph | 중간 | 드라이버 성숙도가 허용되는 경우에만 핵심 그래프 모델과 백엔드 어댑터를 제공. |
| Text | 중간 이상 | Aho-Corasick부터 시작한 뒤 tokenizer/language-detection 품질 게이트를 적용. |
| Workshop | 높음 | Spring/Ktor 자동 구성보다 axum/Tokio/SQLx/Testcontainers 예제를 제공. |

## 첫 마일스톤

1. `0.1.0`: workspace 레이아웃과 `bluetape-rs-core`,
   `bluetape-rs-logging`, `bluetape-rs-test`를 생성하고 async assertion,
   `MultithreadingTester`, `SuspendedJobTester` 및 임시 리소스를 제공한다.
2. `0.2.0`: 집중형 collections 및 async/concurrency 도우미를 추가한다.
3. `0.3.0`부터 `0.5.0`까지: codec, compression 및 serialization을 분리한다.
4. `0.6.0`: 명시적인 Testcontainers fixture를 추가한다.
5. `0.7.0`: resilience보다 먼저 관계형 SQL을 구현한다.
6. `0.8.0`: resilience를 구현한다.
7. `0.9.0`: Redis, RDB, etcd 및 Kubernetes Lease로 인해 여러 백엔드를 다루는
   큰 계열이 되므로 SQL과 resilience 이후에 leader election을 구현한다.

## 위험

- Macro 중심 API는 좋지 않은 compiler error UX를 만들 수 있다.
- 동적 쿼리 사용성과 컴파일 타임 증명 사이에는 상충 관계가 있다.
- 그래프 데이터베이스 드라이버의 성숙도는 JVM 생태계보다 낮다.
- 한국어/일본어 tokenizer 품질은 crate 이용 가능성으로 추정하지 말고 로컬
  corpus를 기준으로 측정해야 한다.
- AWS SDK feature 관리와 컴파일 시간은 지속적으로 통제해야 한다.

## Wiki 기록의 외부 출처

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
