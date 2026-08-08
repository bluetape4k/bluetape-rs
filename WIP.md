# WIP

스냅샷: 2026-06-13 KST
범위: `0.4.0` compression helper 및 `0.5.x` serialization/benchmark 계획.

## 현재 대상 릴리스

`v0.4.0` - backend service용 compression helper.

이 릴리스는 Rust 네이티브 compression train을 시작하고 shared fixture를
사용해 `bluetape-go` 및 `bluetape4k-io`와의 benchmark parity를 추적합니다.

## 현재 상태

- 이 repository는 Rust 2024 workspace이며, root facade crate
  `bluetape-rs`와 core, logging, test support, collections, async helper,
  codec helper, compression helper용 집중형 crate로 구성됩니다.
- 현재 release-prep train은 `0.4.0`입니다. release readiness issue와
  epic이 닫힐 때까지 `0.4.0` 작업을 GitHub milestone `0.4.0`에서
  추적합니다.
- feasibility research는
  `docs/research/2026-06-08-backend-library-feasibility.md`에 있습니다.
- implementation, documentation, benchmark child issue를 merge한 뒤에도
  GitHub milestone `0.4.0`에서 compression epic #73과 release readiness
  issue #80을 추적합니다.
- benchmark 비교 데이터는 `docs/benchmark`에 보존합니다.
- 다음 serialization train은 `0.5.x` milestone으로 나뉘며, `0.5.0`의
  cache-first binary payload 지원부터 시작합니다.
- GitHub milestone `0.5.5`에서는 `bluetape-rs`, `bluetape-go`,
  `bluetape4k-projects`의 same-environment, same-scenario SerDe benchmark를
  추적하고, 이어서 측정된 성능 튜닝을 진행합니다.

## `0.1.0` 범위

1. Rust 2024용 workspace layout, crate naming, feature-flag policy, release
   hygiene, CI command를 확립합니다.
2. typed validation error, validation helper, string helper, small numeric
   check를 위한 `bluetape-rs-core`를 추가합니다.
3. 낮은 설정 부담의 `tracing` setup, structured field, request/task
   correlation, bounded correlation ID, scoped test capture를 위한
   `bluetape-rs-logging`을 추가합니다.
4. async test helper, `MultithreadingTester`, `SuspendedJobTester`,
   assertion helper, temporary-resource cleanup, 향후 Testcontainers 지원
   경계를 위한 `bluetape-rs-test`를 추가합니다.
5. `README.md`, `README.ko.md`, WIP status가 동일한 package 범위와
   caveat를 설명하도록 public docs와 example을 갱신합니다.

## 릴리스 체크리스트

브랜치 정책:

- `develop`을 default branch이자 active development center로 사용합니다.
- `main`을 최신 안정 릴리스 소스 브랜치로 사용합니다.
- stable version tag와 GitHub Release를 만들기 전에 검증한 `develop` tree를
  `main`으로 promote합니다.
- operational release flow는 `docs/release/release-guide.md`에 유지합니다.

1. workspace crate가 additive feature flag를 사용해 Rust 2024에서 compile됩니다.
2. public API에 English Rustdoc과 success, failure, boundary, 적용 가능한
   feature-flag behavior test가 있습니다.
3. `bluetape-rs-logging`은 library code에서 global subscriber를 강제하지
   않고 `tracing`과 통합됩니다.
4. `bluetape-rs-test`는 ownership과 cleanup을 명시하지 않는 한 global
   mutable test state를 사용하지 않습니다.
5. `cargo fmt --all`, `cargo test --workspace`,
   `cargo test --workspace --all-features`,
   `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
   `git diff --check`로 validation을 통과합니다.
6. public behavior, package scope, roadmap 변경에 대한 README parity를
   유지합니다.

## 마일스톤 로드맵

이 로드맵은 package contract를 Rust 네이티브로 유지하면서
`bluetape-go`의 큰 구성을 따릅니다. GitHub milestone `0.4.0`은
활성 compression milestone이며, 다음 serialization train은 `0.5.x`로
나누어 cache payload foundation을 JSON, Protobuf, Avro, Fory 및 cross-repo
benchmark 후속 작업보다 먼저 마련합니다.

| Milestone | 주제 | 비고 |
|---|---|---|
| `0.1.0` | Core helper, logging, test support | Rust workspace hygiene, `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-test`, docs parity. |
| `0.1.1` | 회고 기반 품질 마무리 | `0.1.0` 이후 발견한 누락 spec, review evidence, CI metadata, docs gap 보완. |
| `0.2.0` | Collections 및 async/concurrency helper | 집중형 iterator/map helper, Tokio task helper, bounded concurrency, cancellation/deadline helper. |
| `0.3.0` | Codec helper | Base encoder, hex, URL-safe codec, small binary/text codec helper. |
| `0.4.0` | Compression helper | Opt-in compression helper, safe default, streaming boundary, registry-style codec selection. |
| `0.5.0` | Core + binary serialization | Cache-first serializer/deserializer trait, binary payload contract, typed error, format metadata, trust profile, first binary adapter. |
| `0.5.1` | JSON serialization | `serde_json` bytes-first adapter, UTF-8 string helper, explicit typed decode, malformed-input test. |
| `0.5.2` | Protobuf serialization | Typed Protobuf adapter, opt-in `Any`/type URL 처리, static target decode, compatibility fixture. |
| `0.5.3` | Avro serialization | Schema-first Avro adapter, writer/reader schema 처리, codec interaction, schema evolution test. |
| `0.5.4` | Fory cross-language 평가 | Rust, Go, Kotlin, Java, Python 간 Apache Fory compatibility matrix; default 채택 전 benchmark 및 safety evidence. |
| `0.5.5` | Cross-repo SerDe benchmark 및 성능 튜닝 | `bluetape-rs`, `bluetape-go`, `bluetape4k-projects`의 same-environment benchmark scenario, 비교 가능한 report, 측정된 안전한 최적화. |
| `0.6.0` | Testcontainers fixture | 명시적 feature 뒤의 PostgreSQL, Redis, MySQL, NATS, Kafka, emulator fixture boundary. |
| `0.7.0` | Relational SQL DSL 및 repository helper | Inspectable SQL AST, dialect rendering, bind separation, SQLx adapter; ORM은 주장하지 않음. |
| `0.8.0` | Resilience primitive | Retry, timeout, circuit breaker, bulkhead, backoff, policy composition, observability hook. |
| `0.9.0` | Leader election | Fencing token을 사용하는 Redis, RDB, etcd, Kubernetes Lease leader election; fixture support와 SQL foundation에 의존. |
| `0.10.0` | Cache 및 coordination | Local TTL cache, same-key load collapse, Redis lock, rate limiting, coordination example. |
| `0.11.0` | Portable utility package | ID, JWT, measured value, money, probabilistic structure, utility stabilization. |
| `0.12.0` | Research 및 crypto/encryption gate | 더 큰 domain을 위한 Tink/encryption feasibility 및 research gate. |
| `0.13.0` | AWS helper package 및 example | 공식 AWS SDK for Rust를 감싸는 thin helper와 emulator 기반 example. |
| `0.14.0` | Text package | Aho-Corasick search, blockword masking, tokenizer feasibility, language detection. |
| `0.15.0` | Audit 및 event package | Audit workload에서 영감을 얻은 snapshot, diff, outbox, event-stream primitive. |
| `0.16.0` | Graph package 및 example | Driver maturity review 후 graph abstraction, graph I/O, 선택한 backend adapter. |
| `0.17.0` | Rule engine research 및 구현 | Research가 범위를 정당화할 때 rule model, evaluation contract, integration example. |

## 작업 큐

### `0.1.0` - 기반

- workspace structure와 crate naming policy를 만듭니다.
- MSRV, Rust 2024, feature-flag, release hygiene rule을 정의합니다.
- typed validation error, validation helper, string helper, small numeric check를
  위한 `bluetape-rs-core`를 추가합니다.
- `tracing` setup helper, structured field, bounded correlation ID, scoped test
  capture를 위한 `bluetape-rs-logging`을 추가합니다.
- eventual/consistent assertion, `MultithreadingTester`,
  `SuspendedJobTester`, async test helper, temporary resource cleanup을 위한
  `bluetape-rs-test`를 추가합니다.
- `README.md`, `README.ko.md`, WIP를 동기화합니다.

#### `bluetape-rs-core` 범위

사용한 근거:

- `bluetape-go/core`는 의도적으로 범위가 좁습니다. validation helper,
  pointer-like optional value helper, zero/default helper, string helper, small
  numeric check를 제공합니다.
- `bluetape4k-core`는 validation, codec, collections, concurrency, ranges,
  Java time, Apache wrapper, functional helper, runtime utility까지 훨씬
  넓습니다. 이 category를 `0.1.0`에 그대로 복사하지 않습니다.

`0.1.0`에 포함:

- typed validation error: field/name, 필요한 경우 invalid value,
  `std::error::Error`를 지원하는 stable error message.
- validation helper: non-empty string, non-blank string, inclusive range,
  half-open range, positive number, non-negative number.
- Option/result helper는 `0.1.0`에 의도적으로 포함하지 않습니다. 현재
  범위에서는 standard combinator가 더 명확합니다.
- string helper: `has_text`, empty/blank fallback helper, UTF-8 byte-boundary
  truncation, prefixed hex format check.
- numeric helper: invalid-range error를 반환하는 checked clamp와 small hex
  digit predicate.
- success, failure, boundary, Unicode, invalid-range case를 위한 public
  Rustdoc example과 test.

`0.1.0`에서 연기:

- Codec/base encoder는 `0.3.0`으로 연기합니다.
- general-purpose collections와 async/concurrency helper는 `0.2.0`으로
  연기합니다.
- Compression은 `0.4.0`으로 연기합니다.
- Serialization은 `0.5.0`으로 연기합니다.
- Testcontainers는 `0.6.0`으로 연기합니다.
- ID generation, time DSL, money, measurement, probabilistic structure 및
  기타 portable utility는 `0.11.0`으로 연기합니다.
- Kotlin contract, Apache wrapper API, Java reflection helper, virtual
  thread, Reactor, Java Time DSL과 같은 JVM/Kotlin 전용 개념은 제외합니다.

### `0.2.0` - Collections 및 Async/Concurrency Helper

- `std`, `itertools` 또는 기존 crate가 명확한 해답을 제공하지 않는
  경우에만 집중형 collection helper를 추가합니다. iterator, map, slice,
  page value helper를 포함한 `bluetape-rs-collections`로 구현했습니다.
- Tokio 우선 task group과 bounded concurrency helper를 추가합니다. 처음에는
  `try_map_bounded`, `map_bounded_collect`를 포함한
  `bluetape-rs-async`로 구현했습니다.
- cancellation, timeout, shutdown, deadline helper를 추가합니다.
  `with_timeout`, `with_deadline`, `run_until_cancelled`,
  `with_timeout_or_cancel`, `CancellationSource`, `CancellationToken`,
  shutdown signal pair로 구현했습니다.
- `bluetape-rs-test`를 사용해 결정적 async test pattern을 추가합니다.
  paused time, bounded stress, eventual/consistent assertion을 포함한
  integration coverage를 구현했습니다.
- test에서 task leak이나 unbounded resource growth가 없음을 증명합니다.
  시작한 sibling future에 대해 abort/drain 및 drop-counter check를
  구현했습니다.
- `0.2.0`에서 연기한 항목: codec, compression, serialization,
  Testcontainers, SQL, resilience, leader election, cache/coordination, AWS,
  text, audit, graph, rule-engine 작업.

### `0.3.0` - Codec Helper

- strict hex, Base64 standard, Base64 URL-safe, Bitcoin Base58, byte-oriented
  Base62, UTF-8 text boundary helper를 포함한 `bluetape-rs-codec`로
  구현했습니다.
- codec API에서 allocation과 typed decode error contract를 명시했습니다.
- crate-boundary example과 test를 추가했습니다. usage pattern이 안정적인
  measurement surface를 정당화할 때까지 benchmark는 연기합니다.

### `0.4.0` - Compression Helper

- additive feature flag 뒤에서 gzip, zlib, deflate, zstd, lz4, snappy 압축기를
  제공하는 초기 `bluetape-rs-compression` crate를 구현했습니다.
- registry-style algorithm selection을 사용하는 typed configuration과 error
  contract를 추가했습니다.
- config-aware decompression safety limit, 64 MiB 기본 decode safety limit,
  stream copy helper, `Read`/`Write` boundary용 직접 stream
  reader/writer constructor를 추가했습니다.
- JSON, text, binary, random payload에서 `bluetape-rs`,
  `bluetape-go`, `bluetape4k-io`를 비교하는 동일 조건 benchmark runner와
  report를 추가했습니다.

### `0.5.0` - Core + Binary Serialization

- Rust 네이티브 serializer/deserializer contract를 위한 serialization crate
  boundary를 추가합니다.
- Issue #108은 `crates/serialization`을 package
  `bluetape-rs-serialization`, library `bluetape_rs_serialization`, opt-in
  root facade feature `serialization`으로 추가합니다. 이는 crate/facade/docs
  bootstrap만 수행하며 아직 serializer trait, concrete adapter, runtime
  binary encoding을 노출하지 않습니다.
- Issue #109는 typed `Serializer`/`Deserializer`와
  `BinarySerializer` trait, `SerializedPayload`, `PayloadMetadataPolicy`,
  typed error, safe config default, format id, content type, version, adapter
  id, trust profile vocabulary로 contract layer를 추가합니다. 여전히
  concrete adapter나 runtime binary encoding은 구현하지 않습니다.
- 명시적 format selection과 magic global default가 없는 첫 binary adapter를
  선택하고 구현합니다.
- compression composition을 명시적으로 유지하고 기존 `0.4.0` compression
  crate와 호환되게 합니다.
- round-trip, invalid input, empty payload, version mismatch, format mismatch,
  cache payload boundary case를 집중형 test로 다룹니다.
- JSON, Protobuf, Avro, Fory production adapter는 이후 `0.5.x`
  milestone으로 연기합니다.

### `0.5.1` - JSON Serialization

- UTF-8 string helper를 포함한 `serde_json` 기반 bytes-first adapter를
  추가합니다.
- `serde::Deserialize`를 통해 typed decode를 명시적으로 유지합니다.
- malformed JSON, target type mismatch, UTF-8 boundary, compact output case를
  집중형 test로 다룹니다.
- usage evidence가 정당화할 때까지 JSONB 또는 binary JSON 채택을 연기합니다.

### `0.5.2` - Protobuf Serialization

- `prost`를 중심으로 typed Protobuf serialization을 추가합니다.
- 기본적으로 caller가 target type을 제공하도록 요구합니다. serialized
  bytes가 임의의 Rust type을 선택해서는 안 됩니다.
- dynamic type selection은 trust-boundary decision이므로 `Any`와 type URL
  지원을 opt-in으로 취급합니다.
- corrupted message, wrong target type, compatibility behavior를 위한
  fixture를 추가합니다.
- gRPC transport concern은 범위에서 제외합니다.

### `0.5.3` - Avro Serialization

- `apache-avro`를 중심으로 schema-first Avro serialization을 추가합니다.
- writer schema와 reader schema를 제공하는 방식을 정의합니다.
- v1-to-v2 및 v2-to-v1 schema evolution fixture를 추가합니다.
- 선택한 Rust Avro backend가 직접 지원하는 경우 Avro codec/compression
  interaction을 다룹니다.
- schema registry 지원은 범위에서 제외합니다.

### `0.5.4` - Fory Cross-Language 평가

- Rust, Go, Kotlin, Java, Python payload interoperability를 위해 Apache Fory를
  평가합니다.
- compatibility matrix를 만들고 `0.5.0` binary adapter와 payload
  size/throughput을 비교합니다.
- adapter를 production-ready로 노출하기 전에 trust, compatibility mode,
  schema consistency, upgrade constraint를 문서화합니다.
- benchmark, compatibility, safety evidence가 더 강한 default를 정당화할
  때까지 Fory를 opt-in으로 유지합니다.

### `0.5.5` - Cross-Repo SerDe Benchmark 및 성능 튜닝

- `bluetape-rs`, `bluetape-go`, `bluetape4k-projects`를 위한 shared SerDe
  benchmark fixture와 scenario matrix를 정의합니다.
- 동일한 machine/run condition에서 Rust, Go, Kotlin/JVM benchmark를 실행하고
  repository commit SHA, toolchain/runtime version, benchmark command, raw
  output path, timestamp, run setting을 기록합니다.
- payload size, encode throughput, decode throughput, 가능한 경우 latency,
  allocation/GC note, 관련된 compression interaction을 비교합니다.
- cache-internal, human-readable, schema-first, cross-language use case용
  recommendation matrix를 게시합니다.
- correctness, trust boundary, compatibility, docs를 보존하는 집중적이고
  안전한 performance improvement만 benchmark result로 생성하거나
  실행합니다.
- 단일 benchmark run만으로 global default serializer를 선언하거나 caveat
  없이 서로 다른 scenario를 비교하지 않습니다.

### `0.6.0` - Testcontainers Fixture

- 먼저 PostgreSQL과 Redis용 reusable fixture helper를 추가합니다.
- fixture contract가 안정된 뒤에만 MySQL, NATS, Kafka, local emulator
  fixture를 추가합니다.
- 모든 container-backed behavior를 명시적 feature 뒤에 둡니다.
- success, failure, cancellation path에서 cleanup behavior를 증명합니다.

### `0.7.0` - Relational SQL

- inspectable SQL AST와 dialect renderer로 시작합니다.
- SQL text와 bind value를 분리하여 보존합니다.
- rendering behavior를 test할 수 있게 된 뒤 SQLx execution adapter를
  추가합니다.
- database support를 주장하기 전에 PostgreSQL Testcontainers를 사용합니다.
- lifecycle, relation loading, migration, transaction semantic을 명시적으로
  설계하고 test하기 전에는 ORM support를 주장하지 않습니다.

### `0.8.0` - Resilience

- retry, timeout, circuit breaker, bulkhead, backoff policy를 추가합니다.
- policy는 first-party이며 composable하게 유지하고, 외부 resilience
  framework 전체를 감싸지 않습니다.
- cancellation/deadline test와 observability hook를 추가합니다.
- core policy contract가 안정된 뒤에만 HTTP/service example을 추가합니다.

### `0.9.0` - Leader Election

- leader election을 초기 helper가 아닌 large multi-backend track으로
  취급합니다.
- Redis Testcontainers fixture를 사용할 수 있게 된 뒤에만 Redis leader
  election을 추가합니다.
- relational SQL과 PostgreSQL Testcontainers foundation을 사용할 수 있게 된
  뒤에만 RDB-backed leader election을 추가합니다.
- etcd 및 Kubernetes Lease backend는 milestone 범위에 넣기 전에 별도로
  평가합니다.
- owner token, fencing token, renewal, resign, lookup, shutdown semantic을
  정의합니다.
- support를 주장하기 전에 실제 backend lifecycle test를 추가합니다.

### `0.10.0` - Cache 및 Coordination

- local TTL cache interface와 same-key load collapse를 추가합니다.
- Redis lock과 rate-limit helper는 명시적 feature 뒤에만 추가합니다.
- cross-process cache invalidation을 local cache semantic과 분리합니다.
- Redis-backed behavior를 위한 Testcontainers-backed smoke test를 추가합니다.

### `0.11.0` - Portable Utility

- Rust 네이티브 crate가 강한 building block을 제공하는 경우 ID generation
  helper를 추가합니다.
- 명시적 algorithm과 key selection을 사용하는 JWT helper를 추가합니다.
- dependency와 API 비용이 수용 가능한 경우 measured value, money,
  probabilistic structure를 추가합니다.
- usage evidence가 명확하지 않으면 provider-backed adapter를 연기합니다.

### `0.12.0` - Research 및 Encryption Gate

- Tink/encryption support, Rust crate maturity, key-management boundary를
  평가합니다.
- implementation milestone이 SQL, AWS, text, audit, graph, rule-engine
  track을 사용하기 전에 research를 수집합니다.
- substantial implementation 전에 spec/plan을 만듭니다.

### `0.13.0` - AWS

- 공식 AWS SDK for Rust를 감싸는 thin helper를 우선합니다.
- S3, SQS, DynamoDB 또는 동등한 service용 local emulator example을
  추가합니다.
- credential, retry, region behavior를 명시적으로 유지합니다.

### `0.14.0` - Text

- 먼저 Aho-Corasick search와 blockword masking을 추가합니다.
- tokenizer와 language-detection crate를 채택하기 전에 research합니다.
- large model/runtime dependency는 default feature에서 제외합니다.

### `0.15.0` - Audit 및 Events

- snapshot, diff, outbox, event-stream primitive를 추가합니다.
- JaVers에 의존하지 않고 audit workload에서 영감을 얻은 설계를
  유지합니다.
- adapter 전에 ordering, idempotency, serialization contract를 정의합니다.

### `0.16.0` - Graph

- 구현 전에 Rust graph driver maturity를 다시 평가합니다.
- backend 비용이 명확해진 뒤에만 graph abstraction과 graph I/O를
  추가합니다.
- graph example은 좁은 범위와 backend별 특성을 유지합니다.

### `0.17.0` - Rule Engine

- 먼저 rule model, expression language, evaluation safety를 research합니다.
- model이 작고 test 가능하며 backend service에 유용한 경우에만 구현을
  추가합니다.
- dynamic execution과 untrusted input boundary를 명시적으로 유지합니다.

## 보류

- Full ORM 또는 DAO lifecycle.
- Spring Boot 스타일 auto-configuration.
- 기계적인 Kotlin/JVM 또는 Go API parity.
- general helper, logging, test-support, codec/compression/serialization,
  Testcontainers foundation을 검증하기 전의 SQL, SQLx, ORM, repository
  abstraction.
- relational SQL, resilience, Redis/PostgreSQL Testcontainers foundation을
  사용할 수 있게 되기 전의 leader election.
- core foundation이 안정되기 전의 광범위한 AWS, graph, text, audit coverage.
- `0.1.0`에 필요한 최소 범위를 넘어서는 public release automation.

## 결정 기록

- package scope, roadmap, install guidance, development command가 바뀌면
  `README.md`와 `README.ko.md`를 동기화합니다.
- `develop`을 integration branch이자 default branch로 사용합니다.
- `main`을 latest stable release branch로 사용하고 stable release를
  tag하기 전에 `develop`을 `main`으로 promote합니다.
- Cargo package name은 `bluetape-rs-*`를 사용하고 library target에는
  Rust import form `bluetape_rs_*`를 허용합니다.
- public API는 `Result`, `Option`, ownership-aware builder, narrow trait,
  additive feature flag, explicit error enum을 사용하는 Rust 네이티브
  형태로 유지합니다.
- 넓은 utility bag보다 backend service 가치가 명확한 작은 crate를
  우선합니다.
- infrastructure-facing package의 default async runtime으로 `tokio`를
  사용합니다.
- `unsafe`를 피합니다. 필요해지면 safe boundary 주변에 격리하고 invariant를
  문서화하며 test를 추가합니다.
- PostgreSQL, Redis, Kafka, AWS emulator, graph database 또는 기타
  infrastructure support를 주장하기 전에 Testcontainers-backed test를
  사용합니다.
- 초기 milestone은 helper function, logging, test support를 database
  abstraction보다 먼저 두어 단순하고 널리 유용하게 유지합니다.
- `0.1.0`을 작게 유지하도록 codec, compression, serialization,
  Testcontainers, leader election을 별도 milestone으로 나눕니다.
- serialization은 `0.5.x`로 나눕니다. `0.5.0`에서 cache-first binary
  payload support로 시작하고, 이후 별도 milestone에서 JSON, Protobuf, Avro,
  Fory validation을 추가한 다음 `0.5.5`에서 same-environment cross-repo
  benchmark와 측정된 성능 튜닝을 진행합니다.
- repository/database ergonomics를 higher-level runtime policy보다 먼저
  검증해야 하므로 relational SQL을 resilience보다 먼저 구현합니다.
- backend matrix가 더 크므로 relational SQL과 resilience 이후에 leader
  election을 구현합니다. 대상은 Redis, RDB, etcd, Kubernetes Lease 및
  기타 coordination system일 수 있습니다.
- SQL을 시작할 때 inspectable AST + dialect renderer + SQLx adapter로
  취급합니다. lifecycle, relation loading, migration, transaction
  semantic을 명시적으로 설계하고 test하기 전에는 ORM support를 주장하지
  않습니다.

## 근거

- Repository-local feasibility research:
  `docs/research/2026-06-08-backend-library-feasibility.md`
- 소스 위키 기록:
  `../bluetape4k-wiki/research/2026-06-08-bluetape-rs-backend-library-feasibility.md`
