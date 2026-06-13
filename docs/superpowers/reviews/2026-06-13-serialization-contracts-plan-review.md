# Serialization Contracts Plan Review

Date: 2026-06-13
Scope: issue #109, Step 3-R plan review
Plan: `docs/superpowers/plans/2026-06-13-serialization-contracts-plan.md`
Spec: `docs/superpowers/specs/2026-06-13-serialization-contracts-design.md`

## Review Lanes

Step 3-R ran six independent read-only perspective lanes plus main-session
integration:

- Performance/runtime stability
- Stability/failure-path
- Security
- Operator/Ops
- Developer/API
- User/caller

No review lane edited files or ran heavy cargo commands.

## Initial Findings

| Priority | Area | Finding | Required plan edit |
|---|---|---|---|
| P1 | Metadata policy | `adapter_id` was stored in policy but not validated. | Add strict `Some(expected)` adapter-id validation, `None` wildcard behavior, typed error mapping, and tests. |
| P1 | Error API | Adapter source handling discarded safe source diagnostics or allowed raw-source bypass risk. | Add safe-source and redacted-source constructors, wrap sources, keep raw source alias private, and test `Display`/`Debug`/`source()` behavior. |
| P1 | Error API | Public mismatch errors did not preserve encode/decode direction. | Add `SerializationOperation`, operation-bearing constructors, and tests. |
| P1 | Config/API | `UnsafeLegacyCompatibility` was vocabulary but unreachable from config. | Keep safe defaults and add an explicitly named migration-only opt-in method. |
| P1 | Operator | Payload-free diagnostic/runbook fields were not executable plan gates. | Add README/README.ko diagnostic field requirements and Step 6-R verification. |
| P2 | Test matrix | Metadata policy tests only covered format mismatch. | Add content type, trust profile, adapter id, version, exact-limit, and oversized-payload tests. |
| P2 | API semantics | `SerializedPayload::new` initially used a decode operation for a generic metadata mismatch. | Use operation-neutral `InvalidMetadata` for byte/metadata size mismatch. |
| P2 | Documentation | Doctest verification was implied by `cargo doc`. | Add explicit `cargo test --doc` gate and compile-fail Rustdoc coverage. |
| P2 | Workflow | PR readiness gates were not listed in the plan. | Add Step 6-R, PR `## DoD Status`, Step 7-R, and `P0=0 P1=0` requirements. |

## Revisions Applied

- Added `SerializationOperation` and operation-bearing error constructors.
- Added `AdapterIdMismatch` and strict/wildcard adapter-id policy tests.
- Added safe-source and redacted-source adapter failure paths with raw source
  alias kept private.
- Expanded mismatch and boundary tests for content type, trust profile, adapter
  id, unsupported version, exact payload limit, and oversized payload.
- Rejected empty content-type media type segments such as `/json` and
  `application/`.
- Added explicit unsafe legacy migration opt-in while preserving safe defaults.
- Removed `Clone` from the planned `SerializedPayload` type and added Step 6-R
  allocation/copy review evidence.
- Added doctest, compile-fail Rustdoc, README/README.ko example, payload-free
  diagnostics, and Step 6-R/Step 7-R gates.

## Rerun Verdict

| Lane | P0 | P1 | Verdict |
|---|---:|---:|---|
| Performance/runtime stability | 0 | 0 | PASS |
| Stability/failure-path | 0 | 0 | PASS |
| Security | 0 | 0 | PASS |
| Operator/Ops | 0 | 0 | PASS |
| Developer/API | 0 | 0 | PASS |
| User/caller | 0 | 0 | PASS |
| Main-session integration | 0 | 0 | PASS |

Step 3-R is closed with `P0=0 P1=0`.

## Carry-Forward Checks

- Step 6-R must verify the implementation still has no public raw
  `Box<dyn Error>` or raw `AdapterSource` attachment path.
- Step 6-R must verify strict and wildcard adapter-id policy behavior.
- Step 6-R must verify operation semantics, source redaction, payload-free
  diagnostics, doctests, README parity, and dependency exclusion.
- Step 7-R must run after PR creation before any CI/merge-ready claim.
