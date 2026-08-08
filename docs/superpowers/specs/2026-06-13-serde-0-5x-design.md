# SerDe 0.5.x 설계

날짜: 2026-06-13
상태: 승인된 설계
범위: `0.5.x`의 Rust-native serializer/deserializer milestone

## 결정

`0.5.x` serialization 라인은 cache-first binary payload 지원으로 시작합니다.
첫 milestone은 schema, JSON, cross-language format을 추가하기 전에 내부
cache storage를 위한 가장 작은 유용한 binary serializer 기반을 제공해야
합니다. Cross-repo benchmark와 측정 기반 performance tuning은 core adapter가
생긴 뒤 전용 `0.5.5` milestone으로 보류합니다.

이 설계는 Kotlin/JVM API를 기계적으로 포팅하지 않습니다. Typed error,
`Result`, `Option`, 명시적인 target type, 작은 trait, additive feature flag,
기본적으로 dynamic type loading을 사용하지 않는 Rust-native contract를
사용합니다.

## 참고 근거

Kotlin `bluetape4k-projects` serialization module은 유용한 domain 경계를
제공하지만 직접적인 API 형태를 제공하지는 않습니다.

- `io/io`는 `BinarySerializer` 경계, trust profile, compression composition, Kryo/Fory/JDK 구현을 정의합니다.
- `io/json`은 bytes-first JSON serializer contract와 string helper를 정의합니다.
- `io/jackson3`과 `io/fastjson2`는 구체적인 JSON 구현을 제공하며, Fastjson2 module은 JSONB byte helper도 노출합니다.
- `io/protobuf`는 type URL allowlist와 mixed store용 fallback serializer를 사용해 Protobuf `Any` packing을 수행합니다.
- `io/avro`는 schema-first generic record, specific record, reflection, codec 선택, schema evolution test를 분리합니다.
- Apache Fory는 cross-language payload에 유용하지만 production default가 되기 전에 별도의 호환성 및 안전성 검증이 필요합니다.

## Milestone 분할

### `0.5.0` - Core + Binary SerDe

목표: 내부 cache payload 기반을 제공합니다.

범위:

- `crates/serialization` 아래에 serialization crate 경계를 추가합니다.
- Serialization 및 deserialization의 core trait를 정의합니다.
- Cache storage 및 restoration을 위한 binary payload contract를 정의합니다.
- Typed error, format id, content type, version, trust profile vocabulary를 정의합니다.
- 첫 binary adapter를 선택하고 구현합니다.
- 기존 `0.4.0` compression crate와 명시적이고 호환 가능한 compression composition을 추가합니다.
- Round-trip, invalid input, empty payload, version mismatch, format mismatch test를 추가합니다.

이슈 #108 초기 구성 범위:

- Package name `bluetape-rs-serialization`, library name `bluetape_rs_serialization`으로 `crates/serialization`을 생성합니다.
- `[workspace].members` 및 `[workspace.dependencies]`에 crate를 등록합니다.
- Root optional dependency와 root facade feature를 추가합니다: `serialization = ["dep:bluetape-rs-serialization"]`.
- `#[cfg(feature = "serialization")]` 뒤에서만 `bluetape-rs`에서 crate를 re-export합니다.
- Root default feature set을 변경하지 않습니다.
- Serialization crate default feature set을 최소화하고 JSON, Protobuf, Avro, Fory, Testcontainers, SQL, resilience dependency를 포함하지 않습니다.
- Crate bootstrap, facade wiring, README/Rustdoc 경계 문구, roadmap parity에서 멈춥니다. 첫 binary adapter는 여전히 `0.5.0` milestone의 일부지만 이슈 범위가 확장되지 않는 한 #108의 일부가 아닙니다.

Feature 정책:

- Format integration은 이후 milestone에서 추가할 때 additive opt-in feature여야 합니다.
- 향후 `json`, `protobuf`, `avro`, `fory` feature를 root crate default feature set으로 활성화하지 않습니다.
- Hidden global registry, env-selected adapter, default serializer는 `0.5.0`에서 허용하지 않습니다.

Cache payload contract:

- Cache envelope는 진단에 payload byte를 요구하지 않고 format id, content type, payload version, trust profile, adapter id, payload size를 기록합니다.
- Decode failure는 typed입니다. 최소한 invalid payload, unsupported version, format mismatch, content-type mismatch, trust profile mismatch, oversized payload, adapter failure를 고려해야 합니다.
- Corrupt, unknown-version, wrong-format, wrong-trust-profile, truncated, trailing-byte payload는 조용히 decode하거나 `None`으로 fallback하거나 다른 adapter를 시도하면 안 됩니다. Eviction, cache flush, namespace migration, rebuild는 호출자 정책입니다.
- Old-reader/new-writer와 rollback 동작을 명시해야 합니다. Unknown version은 typed metadata diagnostic과 함께 실패하며 cache namespace/version migration은 best-effort fallback 뒤에 숨기지 않고 문서화합니다.
- 첫 binary adapter는 caller-supplied target type만 사용해야 합니다. Payload가 Rust type을 동적으로 선택하지 않습니다.
- Unsafe deserialization, dynamic registry, unbounded collection/depth decode, unbounded decompressed size를 허용하지 않습니다.
- Implementation plan은 `0.5.5` 이전에 cross-repo benchmark 우월성을 주장하지 않고 buffer ownership, copy boundary, compact header/envelope 기대 사항, small/medium/large payload runtime check를 정의해야 합니다.

범위 외:

- JSON, Protobuf, Avro, Fory production adapter
- Schema registry 또는 schema evolution 지원
- Primary cache payload format으로서의 JSON
- Testcontainers 또는 external service integration
- SQL, SQLx, database adapter, ORM integration
- Resilience, retry, circuit-breaker, fallback policy API
- Hidden global registry, hidden default serializer, env-selected adapter
- Dynamic type loading

### `0.5.1` - JSON SerDe

목표: Binary 기반 이후 portable하고 사람이 읽을 수 있는 SerDe adapter를
추가합니다.

범위:

- `serde_json`을 기본 JSON backend로 사용합니다.
- Bytes-first API와 UTF-8 string helper를 제공합니다.
- `serde::Deserialize`를 통해 typed decode를 명시적으로 유지합니다.
- Malformed JSON, target type mismatch, UTF-8 boundary, 지원하는 경우 pretty/compact output에 대한 test를 추가합니다.

범위 외:

- Jackson/Fastjson 형태의 module parity
- JSONB 또는 binary JSON을 default로 채택

### `0.5.2` - Protobuf SerDe

목표: Payload가 선택한 Rust type을 기본적으로 허용하지 않고 typed
Protobuf serialization을 추가합니다.

범위:

- Typed message에는 Rust Protobuf ecosystem, 아마도 `prost`를 사용합니다.
- 호출자가 target type을 제공하는 typed encode/decode API를 제공합니다.
- `Any`와 type URL 지원은 dynamic type selection이 security boundary이므로 opt-in으로 취급합니다.
- Corrupt message와 wrong target type에 대한 compatibility fixture 및 failure test를 추가합니다.

범위 외:

- gRPC transport concern
- 일반적인 mixed-object fallback serialization

### `0.5.3` - Avro SerDe

목표: 명시적인 schema evolution test와 함께 schema-first serialization을
추가합니다.

범위:

- Rust Avro ecosystem, 아마도 `apache-avro`를 사용합니다.
- 먼저 schema-bound record를 지원합니다.
- Writer schema와 reader schema를 제공하는 방법을 정의합니다.
- v1-to-v2 및 v2-to-v1에 해당하는 schema evolution fixture를 추가합니다.
- Avro가 직접 지원하는 경우 codec/compression interaction test를 추가합니다.

범위 외:

- 완전한 schema registry
- Reflection 형태의 JVM parity

### `0.5.4` - Apache Fory Cross-Language

목표: Cross-language binary payload에 Fory를 평가하고 선택적으로
추가합니다.

범위:

- Compatibility matrix로 Rust, Go, Kotlin, Java, Python interoperability 주장을 검증합니다.
- `0.5.0` binary adapter와 payload size 및 throughput을 측정합니다.
- Trust, compatibility mode, schema consistency, upgrade constraint를 문서화합니다.
- Production safety가 입증될 때까지 모든 adapter를 opt-in으로 유지합니다.

범위 외:

- Benchmark 및 compatibility 근거 없이 Fory를 기본 binary serializer로 만들기

### `0.5.5` - Cross-Repo Benchmark 및 Performance Tuning

목표: 동일한 environment와 scenario에서 `bluetape-rs`, `bluetape-go`,
`bluetape4k-projects` SerDe 동작을 비교한 뒤 측정된 bottleneck만 조정합니다.

범위:

- Rust, Go, Kotlin/JVM 사이에 공유 payload fixture와 scenario matrix를 정의합니다.
- 동일한 machine/run 조건에서 동일한 scenario cell의 benchmark를 실행합니다.
- Repository commit SHA, toolchain/runtime version, benchmark command, raw output path, timestamp, warmup/iteration 설정, environment를 기록합니다.
- 가능하면 payload size, encode time, decode time, throughput, allocation/GC 메모, compression interaction을 비교합니다.
- Cache-internal, human-readable, schema-first, cross-language 사용 사례를 위한 recommendation matrix를 publish합니다.
- Before/after 측정이 변경을 정당화할 때만 집중 performance tuning 후속 작업을 실행합니다.

범위 외:

- 하나의 local benchmark 실행으로 global default serializer 선언
- 주의 사항 없이 서로 다른 scenario 비교
- Microbenchmark 이득을 위해 correctness, security, compatibility, API 명확성을 희생

## API 방향

공개 API는 payload format과 Rust type conversion을 분리해야 합니다.

- `Serializer<T>`와 `Deserializer<T>`는 typed contract입니다.
- `BinarySerializer<T>`는 cache 및 infrastructure payload를 위한 bytes 경계입니다.
- Format metadata는 cache invalidation, migration, rollback, debugging에 충분히 명시적이고 안정적이어야 합니다.
- `Option<T>`는 absent value를 나타내며 empty byte는 숨겨진 null convention이 아니라 payload 경계 결정으로 다룹니다.
- `SerializationTrustProfile`은 format이 trusted-internal, allowlisted, statically typed, unsafe legacy compatibility 중 무엇인지 설명합니다.
- Error value는 payload byte를 log하거나 반환하지 않고 expected/observed format id, content type, payload version, trust profile, adapter id, payload size와 같은 안전한 metadata를 노출합니다.

## GitHub 이슈 재조정

기존 `0.5.0` 이슈는 첫 milestone을 확장하지 않도록 범위를 좁혀야 합니다.

- Crate bootstrap, core contract, 첫 binary adapter는 `0.5.0`에 유지합니다.
- GitHub 범위를 명시적으로 확장하지 않는 한 이슈 #108은 crate bootstrap, root facade gating, documentation parity로 제한합니다.
- JSON adapter 작업은 `0.5.1`로 이동합니다.
- Schema-drift 검사를 Protobuf 및 Avro 후속 작업으로 분리합니다.
- Fory cross-language 작업은 `0.5.4`로 이동합니다.
- Cross-repo benchmark fixture, 동일 조건 runner, report publication, 측정 기반 performance tuning을 위한 `0.5.5` 이슈를 추가합니다.
- 문서 및 release readiness 이슈는 milestone별로 유지합니다.

## Acceptance criteria

- `WIP.md`가 `0.5.x` 분할을 문서화하고 이 설계와 일치합니다.
- 향후 implementation plan은 `0.5.0`을 cache-first 및 binary-first로 유지합니다.
- 이슈 #108 implementation plan은 `crates/serialization`, `bluetape-rs-serialization`, `bluetape_rs_serialization`, root `serialization` feature gating, 변경하지 않는 root default를 명시합니다.
- Feature verification은 root facade가 기본적으로 unavailable이고 `features = ["serialization"]`일 때만 available임을 증명합니다.
- Feature verification은 default build가 JSON, Protobuf, Avro, Fory, Testcontainers, SQL, resilience dependency를 가져오지 않음을 증명합니다.
- `crates/serialization/README.md`, crate Rustdoc, `README.md`, `README.ko.md`, `WIP.md`가 동일한 crate name, root facade feature name, `0.5.0` non-goal list를 사용합니다.
- 공개 문서는 JSON, Protobuf, Avro, Fory, Testcontainers, SQL, resilience API, hidden global, hidden default serializer, dynamic type loading, schema registry 지원이 `0.5.0` 범위 밖임을 명시합니다.
- 공개 문서는 direct crate 사용과 root facade feature-gated 사용을 보여주고 `Option<T>`, empty byte, version mismatch, format mismatch, trust-profile mismatch 동작을 설명합니다.
- `0.5.0` release readiness를 주장하기 전에 test 또는 verification task가 corrupt byte, truncated byte, trailing byte, empty byte, unknown format id, unsupported version, wrong target type, trust-profile mismatch, oversized payload, compressed-invalid payload를 다룹니다.
- 이슈 #108에서는 serialization이 opt-in이므로 기존 root crate 사용자에게 migration이 필요하지 않습니다.
- 전용 validation milestone 전에 어떤 milestone도 Protobuf, Avro, Fory production readiness를 주장하지 않습니다.
- Benchmark claim이 Rust, Go, Kotlin/JVM project line을 비교할 때는 동일 environment의 `0.5.5` benchmark track을 사용합니다.
- 공개 문서는 Rust-native positioning을 보존하고 Kotlin/JVM API parity를 약속하지 않습니다.
