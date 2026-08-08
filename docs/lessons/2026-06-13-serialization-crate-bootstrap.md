# 교훈: Serialization crate 초기 구성

날짜: 2026-06-13
범위: 이슈 #108, `bluetape-rs-serialization` 초기 구성

## 변경 사항

- `crates/serialization` workspace crate를 package `bluetape-rs-serialization` 및 library `bluetape_rs_serialization`으로 추가했습니다.
- 기본 root 동작은 변경하지 않고 opt-in root `serialization` facade feature를 추가했습니다.
- 이슈 #108은 crate/facade/docs 초기 구성으로만 유지했습니다. Serializer trait, adapter, runtime binary encoding은 이후 `0.5.0` 작업입니다.

## 교훈

- 새 crate의 `lib.rs`는 먼저 sibling crate style을 따라야 합니다. 긴 roadmap, 이슈 이력, 사용 가이드, non-goal 설명은 crate root가 아니라 README/spec/plan artifact에 둡니다.
- 아직 release되지 않은 crate의 공개 README snippet은 새 crate나 feature가 없는 이미 publish된 version을 caller에게 가리키면 안 됩니다. Release 준비에서 version을 갱신하기 전에는 git/path 또는 명시적인 post-release 예시를 사용합니다.
- Bootstrap 이슈에서도 WIP/roadmap 추적성이 중요합니다. Milestone 범위를 설명하는 곳에 package, library, root feature, non-goal을 명시합니다.
- Step 6-R 근거 파일은 사후 장식이 아니라 gate의 일부입니다. Gate가 닫혔다고 주장하기 전에 구현 review artifact를 추가합니다.

## 누락을 발견한 검사

- Step 6-R user/caller review에서 publish되지 않은 `0.4.0` Cargo snippet을 발견했습니다.
- Step 6-R operator review에서 추적되는 review 및 lesson artifact가 빠진 것을 발견했습니다.
- Step 6-R stability review에서 `WIP.md`의 crate/facade 추적성이 약한 것을 발견했습니다.

## 앞으로의 규칙

향후 `bluetape-rs` crate bootstrap마다 최소 두 개의 sibling crate root를
비교한 뒤 `lib.rs`를 작성하고, 공개 Cargo snippet이 release 상태를
반영하는지 확인하며, PR 생성 전에 Step 6-R review artifact를 만듭니다.
