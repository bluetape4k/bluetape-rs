# Serialization contract 계획 검토

날짜: 2026-06-13
범위: 이슈 #109, Step 3-R plan review
Plan: `docs/superpowers/plans/2026-06-13-serialization-contracts-plan.md`
Spec: `docs/superpowers/specs/2026-06-13-serialization-contracts-design.md`

## 검토 lane

Step 3-R은 여섯 개의 독립적인 read-only 관점 lane과 main-session 통합으로
실행했습니다.

- Performance/runtime stability
- Stability/failure-path
- Security
- Operator/Ops
- Developer/API
- User/caller

검토 lane은 파일을 편집하거나 무거운 cargo 명령을 실행하지 않았습니다.

## 최초 발견 사항

| 우선순위 | 영역 | 발견 사항 | 필요한 plan 수정 |
|---|---|---|---|
| P1 | Metadata policy | `adapter_id`를 policy에 저장했지만 검증하지 않았습니다. | Strict `Some(expected)` adapter-id validation, `None` wildcard 동작, typed error mapping, test를 추가합니다. |
| P1 | Error API | Adapter source 처리가 안전한 source 진단을 버리거나 raw-source bypass 위험을 허용했습니다. | Safe-source 및 redacted-source constructor를 추가하고 source를 wrap하며 raw source alias를 private으로 유지하고 `Display`/`Debug`/`source()` 동작을 테스트합니다. |
| P1 | Error API | 공개 mismatch error가 encode/decode 방향을 보존하지 않았습니다. | `SerializationOperation`, operation-bearing constructor, test를 추가합니다. |
| P1 | Config/API | `UnsafeLegacyCompatibility`가 vocabulary이지만 config에서 도달할 수 없었습니다. | Safe default를 유지하고 이름이 명시된 migration-only opt-in method를 추가합니다. |
| P1 | Operator | Payload-free diagnostic/runbook field가 실행 가능한 plan gate가 아니었습니다. | README/README.ko diagnostic field 요구 사항과 Step 6-R 검증을 추가합니다. |
| P2 | Test matrix | Metadata policy test가 format mismatch만 다뤘습니다. | Content type, trust profile, adapter id, version, exact-limit, oversized-payload test를 추가합니다. |
| P2 | API semantics | `SerializedPayload::new`가 처음에는 일반적인 metadata mismatch에 decode operation을 사용했습니다. | Byte/metadata size mismatch에는 operation-neutral `InvalidMetadata`를 사용합니다. |
| P2 | 문서 | Doctest 검증이 `cargo doc`에 암시되어 있었습니다. | 명시적인 `cargo test --doc` gate와 compile-fail Rustdoc coverage를 추가합니다. |
| P2 | Workflow | PR readiness gate가 plan에 나열되지 않았습니다. | Step 6-R, PR `## DoD Status`, Step 7-R, `P0=0 P1=0` 요구 사항을 추가합니다. |

## 적용한 수정

- `SerializationOperation`과 operation-bearing error constructor를 추가했습니다.
- `AdapterIdMismatch`와 strict/wildcard adapter-id policy test를 추가했습니다.
- Safe-source와 redacted-source adapter failure 경로를 추가하고 raw source alias는 private으로 유지했습니다.
- Content type, trust profile, adapter id, unsupported version, exact payload limit, oversized payload에 대한 mismatch 및 boundary test를 확장했습니다.
- `/json`, `application/` 같은 빈 content-type media type segment를 거부했습니다.
- Safe default를 유지하면서 명시적인 unsafe legacy migration opt-in을 추가했습니다.
- 계획한 `SerializedPayload` type에서 `Clone`을 제거하고 Step 6-R allocation/copy review 근거를 추가했습니다.
- Doctest, compile-fail Rustdoc, README/README.ko example, payload-free diagnostic, Step 6-R/Step 7-R gate를 추가했습니다.

## 재실행 판정

| Lane | P0 | P1 | 판정 |
|---|---:|---:|---|
| Performance/runtime stability | 0 | 0 | PASS |
| Stability/failure-path | 0 | 0 | PASS |
| Security | 0 | 0 | PASS |
| Operator/Ops | 0 | 0 | PASS |
| Developer/API | 0 | 0 | PASS |
| User/caller | 0 | 0 | PASS |
| Main-session integration | 0 | 0 | PASS |

Step 3-R은 `P0=0 P1=0`으로 종료되었습니다.

## 이어서 확인할 항목

- Step 6-R은 구현에 공개 raw `Box<dyn Error>` 또는 raw `AdapterSource` attachment 경로가 없는지 확인해야 합니다.
- Step 6-R은 strict 및 wildcard adapter-id policy 동작을 확인해야 합니다.
- Step 6-R은 operation semantics, source redaction, payload-free diagnostic, doctest, README parity, dependency exclusion을 확인해야 합니다.
- Step 7-R은 PR 생성 후 CI/merge-ready 주장을 하기 전에 실행해야 합니다.
