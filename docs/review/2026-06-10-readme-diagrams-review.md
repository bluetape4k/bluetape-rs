# README 다이어그램 검토

## 범위

- 이슈: #43
- Milestone: 0.2.0
- 변경 대상: root README, localized README, crate README file, README diagram asset, CI pull request trigger

## 7-Tier 검토

| Tier | 결과 | 근거 |
| --- | --- | --- |
| Workflow 범위 | 통과 | README 및 diagram 작업을 #42 async code split과 분리했습니다. |
| Source truth | 통과 | 다이어그램을 생성하기 전에 Cargo workspace와 crate `lib.rs`를 읽었습니다. |
| Graphviz layout | 통과 | 모든 다이어그램에 `docs/images/readme-diagrams/` 아래 `.dot`, `.plain`, `.graphviz.svg`, `.graphviz.png` 근거가 있습니다. |
| Rendered asset | 통과 | 모든 README PNG에 decorated SVG source와 Graphviz-rendered PNG 근거가 있습니다. |
| Decorator 기준 | 통과 | 최종 SVG/PNG asset이 compact bluetape4k-style outer frame, title/subtitle header, inner body panel, shadow가 있는 pastel card, footer callout을 사용합니다. |
| README embed | 통과 | README file은 `.png` asset만 embed합니다. |
| CI trigger | 통과 | `pull_request.paths-ignore`를 제거해 docs-only 변경에도 PR CI가 실행되며 bluetape-go PR trigger 형태와 일치합니다. |
| Visual 검토 | 통과 | Compact decorated contact sheet와 workspace/core PNG를 검사했으며 빈 영역, clipping, 불균형 margin, primary content overlap을 찾지 못했습니다. |
| 위험 | 낮음 | Rust source 변경이 없는 문서 전용 변경입니다. |

## 발견 사항

- P0: 0
- P1: 0
- P2: 0
- P3: 0

## 검증

- 통과: 모든 commit된 `.dot` file에 `dot -Tplain` 실행, stderr 비어 있음
- 통과: README image reference가 commit된 `.png` file을 가리킴
- 통과: 모든 `.png`에 대응하는 `.svg`가 있음
- 통과: Commit된 SVG file에 `Inter`, `Arial`, `Helvetica` 또는 오래된 arrowhead marker size가 없음
- 통과: Compact final SVG가 Graphviz에서 유래한 node/route topology를 유지하면서 README 가독성을 위해 최종 connector path를 단순화함
- 통과: `cargo fmt --all --check`
- 통과: `cargo test --workspace --all-features --locked`
- 통과: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- 통과: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- 통과: `actionlint .github/workflows/ci.yml`
- 통과: `git diff --check`
