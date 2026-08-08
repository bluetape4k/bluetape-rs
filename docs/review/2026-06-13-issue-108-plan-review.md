# 이슈 #108 계획 검토 - 직렬화 크레이트 부트스트랩

날짜: 2026-06-13
범위: `docs/superpowers/plans/2026-06-13-serialization-crate-bootstrap-plan.md`
참조 사양: `docs/superpowers/specs/2026-06-13-serde-0-5x-design.md`
게이트: Step 3-R 계획 검토

## 검토 범위

- 이슈 #108: `Add the serialization workspace crate`
- 마일스톤: `0.5.0`
- 워크트리: `.worktrees/issue-108-serialization-crate`
- 이전 게이트: `P0=0 P1=0`으로 Step 2-R 사양 검토 통과

## 검토 레인

| 레인 | 초기 결과 | 최종 결과 | 근거 |
|---|---:|---:|---|
| 성능 | `P0=0 P1=0` | `P0=0 P1=0` | 기본 빌드 feature-tree 확인, `0.5.5` 벤치마크 문구, 리소스 한도 후속 항목을 추가했다. |
| 안정성 | `P0=0 P1=2` | `P0=0 P1=0` | 크레이트 뼈대를 워크스페이스 등록보다 먼저 만들도록 작업 순서를 바꾸고 불일치/no-fallback 문서를 추가했다. |
| 보안 | `P0=0 P1=0` | `P0=0 P1=0` | 안전하지 않은 역직렬화와 숨겨진 기본 serializer를 비목표로 추가하고 전체 보류 실패 매트릭스를 추가했다. |
| 운영자/Ops | `P0=0 P1=1` | `P0=0 P1=0` | 위험한 전체 members 조각을 한 줄 삽입으로 교체하고 lock 갱신, 메타데이터/릴리스 경계, 검토/lesson/PR DoD 산출물을 추가했다. |
| 개발자/API | `P0=0 P1=4` | `P0=0 P1=0` | 작업 순서, 워크스페이스 멤버 보존, `Cargo.lock` 처리, Markdown fence, 버전 정책, 기본 feature 검증을 수정했다. |
| 사용자/호출자 | `P0=0 P1=2` | `P0=0 P1=0` | 공개 불일치/no-fallback 문서, 직접 크레이트 사용, 마이그레이션 메모, 엄격한 비목표, 루트 README 지침, 한국어 동등성 문구를 추가했다. |

## 통합 발견 사항

| 우선순위 | 영역 | 해결 |
|---|---|---|
| P1 | 실행 순서 | 이제 계획이 워크스페이스에 등록하기 전에 `crates/serialization`을 생성한다. |
| P1 | 워크스페이스 멤버십 | 이제 계획이 `crates/serialization`만 삽입하고 기존 멤버를 모두 보존하도록 지시한다. |
| P1 | Lockfile 흐름 | 이제 계획이 잠금 검증 전에 `Cargo.lock`을 한 번 갱신한다. |
| P1 | 문서 의미론 | README/Rustdoc 작업에 지원하지 않는 버전, 잘못된 형식/신뢰 프로파일, `None` fallback 금지, 대체 어댑터 fallback 금지, 호출자가 소유하는 캐시 마이그레이션/재구축 정책을 포함했다. |
| P1 | 기본 파사드 증명 | 이제 계획이 기본 feature를 끈 빌드와 serialization 활성화 빌드 외에 일반 기본 빌드와 feature tree도 검증한다. |

## 보류한 P2 항목

- 이후 어댑터/릴리스 준비 상태 이슈는 인코딩 크기(encoding size),
  압축 해제 크기(decompressed-size), 압축 비율(compression ratio),
  컬렉션/깊이 경계(collection/depth bounds), 손상된 바이트(corrupt bytes),
  잘린 바이트(truncated bytes), 후행 바이트(trailing bytes), 빈 바이트(empty bytes),
  알 수 없는 format id(unknown format id), content-type 불일치(content-type mismatch),
  지원하지 않는 버전(unsupported version), 잘못된 대상 타입(wrong target type),
  trust-profile 불일치(trust-profile mismatch), 과도하게 큰 페이로드(oversized payload),
  압축된 잘못된 페이로드(compressed-invalid payload), 어댑터 실패 사례(adapter
  failure cases) 및 실제 API가 존재할 때 실행 가능한 문서/예제를 확인해야 한다.
- 릴리스 준비 상태 주장은 릴리스 가이드를 따르고 이후 릴리스 준비 상태
  이슈에 게시 dry-run 근거를 포함해야 한다.

## 게이트 판정

계획을 개정한 뒤 Step 3-R을 통과했다.

P0=0 P1=0
