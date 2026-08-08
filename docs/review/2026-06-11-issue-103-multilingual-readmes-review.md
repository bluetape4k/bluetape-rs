# 이슈 #103 다국어 README 검토

## 범위

- 아직 README가 없는 모든 workspace 모듈에 `README.md` / `README.ko.md`
  쌍을 추가한다.
- 기존 crate README 파일에 공통 `English | 한국어` 탐색 형식을 추가한다.
- 다이어그램 자산은 단일 소스로 유지하며 로컬라이즈된 다이어그램 파일은
  추가하지 않는다.
- `bluetape-rs-workshop`은 제외한다.

## 근거

| 확인 항목 | 결과 |
|---|---|
| `git diff --check` | Pass |
| workspace member README matrix 검사 | Pass |
| README 언어 탐색 검사 | Pass |
| 다이어그램 diff 검사 | Pass: `docs/images/readme-diagrams` 아래 파일은 변경하지 않음 |
| workshop 제외 검사 | Pass: `workshop` 경로는 변경하지 않음 |
| `cargo metadata --no-deps --format-version 1` | Pass |
| `cargo fmt --all --check` | Pass |
| `cargo test --workspace` | Pass |

## DoD 상태

| 항목 | 상태 |
|---|---|
| 이슈 전용 worktree 사용 | 완료 |
| 모든 workspace 모듈에 English README 존재 | 완료 |
| 모든 workspace 모듈에 Korean README 존재 | 완료 |
| README 탐색에 `English | 한국어` 형식 사용 | 완료 |
| 다이어그램 로컬라이제이션 제외 | 완료 |
| workshop 변경 없음 | 완료 |
| docs-only 검증 완료 | 완료 |
