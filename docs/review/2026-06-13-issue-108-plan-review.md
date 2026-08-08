# 이슈 #108 계획 검토 - Serialization crate 초기 구성

날짜: 2026-06-13
범위: `docs/superpowers/plans/2026-06-13-serialization-crate-bootstrap-plan.md`
참고 spec: `docs/superpowers/specs/2026-06-13-serde-0-5x-design.md`
Gate: Step 3-R plan review

## 검토 범위

- 이슈 #108: `Add the serialization workspace crate`
- Milestone: `0.5.0`
- Worktree: `.worktrees/issue-108-serialization-crate`
- 이전 gate: Step 2-R spec review가 `P0=0 P1=0`으로 통과

## 검토 lane

| Lane | 최초 결과 | 최종 결과 | 근거 |
|---|---:|---:|---|
| Performance | `P0=0 P1=0` | `P0=0 P1=0` | Default build feature-tree check, `0.5.5` benchmark 문구, resource-bound carryover를 추가했습니다. |
| Stability | `P0=0 P1=2` | `P0=0 P1=0` | Crate skeleton을 workspace 등록보다 먼저 만들도록 task 순서를 바꾸고 mismatch/no-fallback 문서를 추가했습니다. |
| Security | `P0=0 P1=0` | `P0=0 P1=0` | Unsafe deserialization 및 숨겨진 default serializer non-goal과 전체 deferred failure matrix를 추가했습니다. |
| Operator/Ops | `P0=0 P1=1` | `P0=0 P1=0` | 위험한 full members snippet을 한 줄 insertion으로 바꾸고 lock refresh, metadata/release 경계, review/lesson/PR DoD artifact를 추가했습니다. |
| Developer/API | `P0=0 P1=4` | `P0=0 P1=0` | Task 순서, workspace member 보존, `Cargo.lock` 처리, Markdown fence, version 정책, default feature 검증을 수정했습니다. |
| User/Caller | `P0=0 P1=2` | `P0=0 P1=0` | 공개 mismatch/no-fallback 문서, direct crate 사용, migration note, strict non-goal, root README 지침, 한국어 parity 문구를 추가했습니다. |

## 통합 발견 사항

| 우선순위 | 영역 | 해결 |
|---|---|---|
| P1 | 실행 순서 | Plan이 workspace에 등록하기 전에 `crates/serialization`을 생성하도록 수정했습니다. |
| P1 | Workspace membership | Plan이 `crates/serialization`만 삽입하고 기존 member를 모두 보존하도록 했습니다. |
| P1 | Lockfile 흐름 | Locked verification 전에 `Cargo.lock`을 한 번 갱신하도록 했습니다. |
| P1 | 문서 의미 | README/Rustdoc task에 unsupported version, wrong format, wrong trust profile, `None` fallback 없음, alternate adapter fallback 없음, caller-owned cache migration/rebuild 정책을 포함했습니다. |
| P1 | Default facade 근거 | No-default 및 serialization-enabled build 외에 일반 default build와 feature tree도 검증하도록 했습니다. |

## 보류 P2 항목

- 후속 adapter/release-readiness 이슈는 encoded-size, decompressed-size, compression ratio, collection/depth bound, corrupt byte, truncated byte, trailing byte, empty byte, unknown format id, content-type mismatch, unsupported version, wrong target type, trust-profile mismatch, oversized payload, compressed-invalid payload, adapter failure, 실제 API가 생겼을 때의 executable docs/example을 추적해야 합니다.
- Release-ready 주장은 release guide를 따르고 이후 release-readiness 이슈에 publish dry-run 근거를 포함해야 합니다.

## Gate 판정

Plan 수정 후 Step 3-R 통과

P0=0 P1=0
