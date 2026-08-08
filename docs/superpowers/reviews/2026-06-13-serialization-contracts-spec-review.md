# Serialization contract spec 검토

날짜: 2026-06-13
Gate: Step 2-R
Spec: `docs/superpowers/specs/2026-06-13-serialization-contracts-design.md`
이슈: #109

## 범위

`bluetape-rs-serialization`의 이슈 #109 contract-only spec을 검토했습니다.
검토에는 `bluetape-rs-patterns`와 `bluetape4k-full-feature` Step 2-R reference를
적용했습니다. Gate는 여섯 개의 독립적인 read-only native review lane과
현재 session 통합으로 실행했습니다.

## 최초 발견 사항

| Lane | P0 | P1 | P2 | P3 | 판정 |
|---|---:|---:|---:|---:|---|
| Performance | 0 | 0 | 2 | 1 | PASS |
| Stability | 0 | 0 | 2 | 1 | PASS |
| Security | 0 | 0 | 1 | 1 | PASS |
| Operator/Ops | 0 | 1 | 2 | 1 | BLOCKED |
| Developer/API | 0 | 0 | 3 | 2 | PASS |
| User/Caller | 0 | 1 | 3 | 1 | BLOCKED |
| Main integration | 0 | 2 | 6 | 2 | BLOCKED |

Blocker 발견 사항:

- Operator/Ops P1: Metadata contract에 cache namespace/version rollout semantics가 너무 보류되어 있었습니다.
- User/Caller P1: `Serializer::metadata(payload_size)`가 호출자가 제공한 metadata size를 encoded byte와 다르게 만들 수 있었습니다.

## Spec 변경

- 호출자 제공 payload-size metadata를 `SerializedPayload`로 교체했습니다. Constructor가 `metadata.payload_size == bytes.len()`을 도출하거나 검증합니다.
- Deterministic `PayloadMetadataPolicy` matching rule을 추가했습니다.
- Format id, content type, adapter id에 exact max length와 허용 byte를 추가했습니다.
- 기본 max payload size를 `16 * 1024 * 1024` byte로 추가하고 inclusive boundary 동작을 정의했습니다.
- Adapter source-error redaction 요구 사항과 `Display`/`Debug`/`source()` leakage test를 추가했습니다.
- Namespace/key-prefix versioning, hard-reject mismatch 동작, evict/rebuild/migrate/alert action, rollback 동작, payload-free observability field에 대한 cache rollout 및 operator guidance를 추가했습니다.
- Compile-checked example, contract-only 범위, adapter 보류, dynamic registry 없음, unsafe legacy migration warning에 대한 README/Rustdoc/README.ko acceptance criteria를 추가했습니다.

## 재실행 발견 사항

| Lane | P0 | P1 | P2 | P3 | 판정 |
|---|---:|---:|---:|---:|---|
| Operator/Ops 재실행 | 0 | 0 | 0 | 1 | PASS |
| User/Caller 재실행 | 0 | 0 | 0 | 0 | PASS |
| Main integration 재실행 | 0 | 0 | 0 | 1 | PASS |

남은 비차단 항목:

- P3: Spec 단계의 release 근거가 여전히 일반적입니다. PR, CI, 최종 release 근거를 Step 7-P, Step 7-R, Step 8, Step 9가 담당하므로 허용됩니다.

## 수렴 판정

Step 2-R은 `P0=0`, `P1=0`으로 통과합니다.

### Step 2-R Checklist 완료 보고

| 항목 | 상태 | 메모 |
|---|---|---|
| 여섯 관점 lane 완료 | 완료 | performance, stability, security, operator, developer/API, user/caller |
| Main integration 완료 | 완료 | 현재 session에서 발견 사항을 통합하고 severity를 정규화 |
| P0 발견 수정 및 재실행 | N/A | P0 발견 사항 없음 |
| P1 발견 수정 및 재실행 | 완료 | Operator/Ops 및 User/Caller 재실행 통과 |
| P2/P3 처리 기록 | 완료 | P2 항목은 spec에 반영했고 P3 하나는 PR/CI evidence gate로 보류 |
| 수렴 도달 | 완료 | 최종 P0=0 P1=0 |
