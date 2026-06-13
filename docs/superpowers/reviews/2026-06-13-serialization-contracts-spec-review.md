# Serialization Contracts Spec Review

Date: 2026-06-13
Gate: Step 2-R
Spec: `docs/superpowers/specs/2026-06-13-serialization-contracts-design.md`
Issue: #109

## Scope

Reviewed the issue #109 contract-only spec for `bluetape-rs-serialization`.
The review applied `bluetape-rs-patterns` and the
`bluetape4k-full-feature` Step 2-R reference. The gate used six independent
read-only native review lanes plus current-session integration.

## Initial Findings

| Lane | P0 | P1 | P2 | P3 | Verdict |
|---|---:|---:|---:|---:|---|
| Performance | 0 | 0 | 2 | 1 | PASS |
| Stability | 0 | 0 | 2 | 1 | PASS |
| Security | 0 | 0 | 1 | 1 | PASS |
| Operator/Ops | 0 | 1 | 2 | 1 | BLOCKED |
| Developer/API | 0 | 0 | 3 | 2 | PASS |
| User/Caller | 0 | 1 | 3 | 1 | BLOCKED |
| Main integration | 0 | 2 | 6 | 2 | BLOCKED |

Blocking findings:

- Operator/Ops P1: cache namespace/version rollout semantics were too
  deferred for a metadata contract.
- User/Caller P1: `Serializer::metadata(payload_size)` allowed caller-supplied
  metadata size to diverge from encoded bytes.

## Spec Changes

- Replaced caller-supplied payload-size metadata with `SerializedPayload`, whose
  constructor derives or validates `metadata.payload_size == bytes.len()`.
- Added deterministic `PayloadMetadataPolicy` matching rules.
- Added exact max lengths and allowed bytes for format id, content type, and
  adapter id.
- Added a default max payload size of `16 * 1024 * 1024` bytes with inclusive
  boundary behavior.
- Added adapter source-error redaction requirements and `Display`/`Debug`/
  `source()` leakage tests.
- Added cache rollout and operator guidance for namespace/key-prefix
  versioning, hard-reject mismatch behavior, evict/rebuild/migrate/alert
  actions, rollback behavior, and payload-free observability fields.
- Added README/Rustdoc/README.ko acceptance criteria for compile-checked
  examples, contract-only scope, adapter deferral, no dynamic registry, and
  unsafe legacy migration warnings.

## Rerun Findings

| Lane | P0 | P1 | P2 | P3 | Verdict |
|---|---:|---:|---:|---:|---|
| Operator/Ops rerun | 0 | 0 | 0 | 1 | PASS |
| User/Caller rerun | 0 | 0 | 0 | 0 | PASS |
| Main integration rerun | 0 | 0 | 0 | 1 | PASS |

Remaining non-blocking item:

- P3: release evidence remains generic at the spec stage. This is acceptable
  because Step 7-P, Step 7-R, Step 8, and Step 9 own PR, CI, and final release
  evidence.

## Convergence Verdict

Step 2-R passes with `P0=0` and `P1=0`.

### Step 2-R Checklist Completion Report

| Item | Status | Notes |
|---|---|---|
| Six perspective lanes complete | Done | performance, stability, security, operator, developer/API, user/caller |
| Main integration complete | Done | Current session integrated findings and normalized severity |
| P0 findings fixed and rerun | N/A | No P0 findings |
| P1 findings fixed and rerun | Done | Operator/Ops and User/Caller reruns passed |
| P2/P3 disposition recorded | Done | P2 items incorporated into spec; one P3 deferred to PR/CI evidence gates |
| Convergence reached | Done | Final P0=0 P1=0 |
