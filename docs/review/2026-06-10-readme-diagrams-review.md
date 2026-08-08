# README 다이어그램 검토

## 범위

- 이슈: #43
- 마일스톤: 0.2.0
- 변경 표면: 루트 README, 로컬라이즈 README, 크레이트 README 파일, README
  다이어그램 자산 및 CI pull request trigger.

## 7-Tier 검토

| 단계 | 결과 | 근거 |
| --- | --- | --- |
| 워크플로 범위 | Pass | README 및 다이어그램 작업을 #42 비동기 코드 분할과 분리했다. |
| 소스 진실성 | Pass | 다이어그램을 생성하기 전에 Cargo 워크스페이스와 크레이트 `lib.rs` 파일을 읽었다. |
| Graphviz 배치 | Pass | 모든 다이어그램에 `docs/images/readme-diagrams/` 아래 `.dot`, `.plain`, `.graphviz.svg`, `.graphviz.png` 근거가 있다. |
| 렌더링된 에셋 | Pass | 모든 README PNG에 장식된 SVG 소스와 Graphviz 렌더링 PNG 근거 파일이 대응한다. |
| 데코레이터 기준 | Pass | 최종 SVG/PNG 에셋은 간결한 bluetape4k 스타일 외곽 프레임, 제목/부제 헤더, 내부 본문 패널, 그림자가 있는 파스텔 카드, footer callout을 사용한다. |
| README 삽입 | Pass | README 파일은 `.png` 에셋만 삽입한다. |
| CI 트리거 | Pass | `pull_request.paths-ignore`를 제거해 문서만 변경한 경우에도 PR CI를 실행하며 bluetape-go PR 트리거 형태와 일치한다. |
| 시각 검토 | Pass | 간결한 장식 contact sheet와 워크스페이스/core PNG를 검사했으며 비어 있거나 잘리거나 여백이 불균형하거나 주요 콘텐츠가 겹치지 않았다. |
| 위험 | 낮음 | Rust 소스를 변경하지 않은 문서 전용 변경이다. |

## 발견 사항

- P0: 0
- P1: 0
- P2: 0
- P3: 0

## 검증

- 통과: 모든 커밋된 `.dot` 파일에 `dot -Tplain`을 실행했고 stderr가 비어 있다.
- 통과: README image reference가 커밋된 `.png` 파일을 가리킨다.
- 통과: 모든 `.png`에 대응하는 `.svg`가 있다.
- 통과: 커밋된 SVG 파일에 `Inter`, `Arial`, `Helvetica` 또는 오래된 arrowhead
  marker size가 없다.
- 통과: 간결한 최종 SVG가 Graphviz 기반 node/route topology를 유지하면서
  README 가독성을 위해 최종 connector path를 단순화한다.
- 통과: `cargo fmt --all --check`
- 통과: `cargo test --workspace --all-features --locked`
- 통과: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- 통과: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- 통과: `actionlint .github/workflows/ci.yml`
- 통과: `git diff --check`
