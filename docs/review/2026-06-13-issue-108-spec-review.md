# 이슈 #108 Spec 검토 - SerDe 0.5.x 설계

날짜: 2026-06-13
범위: `docs/superpowers/specs/2026-06-13-serde-0-5x-design.md`
Gate: Step 2-R spec review

## 검토 범위

- 이슈 #108: `Add the serialization workspace crate`
- Milestone: `0.5.0`
- Worktree: `.worktrees/issue-108-serialization-crate`
- 기준: 승인된 `0.5.x` SerDe design과 이슈 #108 acceptance criteria

## 검토 lane

| Lane | 최초 결과 | 최종 결과 | 근거 |
|---|---:|---:|---|
| Performance | `P0=0 P1=0` | `P0=0 P1=0` | Feature-default, allocation, runtime-check 명확화를 요청했고 blocker는 없었습니다. |
| Stability | `P0=0 P1=2` | `P0=0 P1=0` | Envelope/error contract, no-silent-fallback rule, feature matrix, resource-bound 요구 사항을 추가했습니다. |
| Security | `P0=0 P1=2` | `P0=0 P1=0` | Malicious payload 방어, unsafe/dynamic deserialization 금지, hidden global/default adapter 정책 금지를 추가했습니다. |
| Operator/Ops | `P0=0 P1=3` | `P0=0 P1=0` | Crate/workspace/root-facade checklist, rollback/version 정책, 안전한 metadata 진단, docs parity를 추가했습니다. |
| Developer/API | `P0=0 P1=2` | `P0=0 P1=0` | Package/lib name, workspace dependency contract, root feature/re-export 형태, additive feature 정책을 추가했습니다. |
| User/Caller | `P0=0 P1=2` | `P0=0 P1=0` | 이슈 #108 bootstrap slice, 완전한 non-goal list, README/Rustdoc/example acceptance, migration note를 추가했습니다. |

## 통합 발견 사항

| 우선순위 | 영역 | 해결 |
|---|---|---|
| P1 | Bootstrap 범위 | Spec이 `0.5.0` milestone과 좁은 이슈 #108 crate/facade/docs bootstrap slice를 분리합니다. |
| P1 | Feature/default 경계 | Root default를 변경하지 않고 opt-in root `serialization` feature를 사용하며 default format/infra dependency를 두지 않도록 했습니다. |
| P1 | Payload 복구 | Typed failure, silent fallback 금지, 명시적 unknown-version 동작, caller-owned eviction/rebuild 정책을 요구합니다. |
| P1 | Security 경계 | Payload-selected Rust type, unsafe deserialization, dynamic registry, hidden global serializer, env-selected adapter를 금지합니다. |
| P1 | Operational 진단 | Payload byte를 log하지 않고 안전한 expected/observed metadata 진단을 요구합니다. |

## Step 3을 위한 보류 P2

- Implementation plan은 release-readiness 주장 전에 구체적인 encoded-size, decompressed-size, ratio, depth, collection-bound 검사 방법을 정의해야 합니다.
- Implementation plan은 direct crate usage, root facade feature-gated usage, default facade 부재에 대한 구체적인 doc/example 검증을 포함해야 합니다.

## Gate 판정

Spec 수정 후 Step 2-R 통과

P0=0 P1=0
