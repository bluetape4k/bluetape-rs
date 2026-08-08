# 0.2.0 문서 범위 검토

## 범위

- 이슈: #24
- Milestone: 0.2.0
- 변경 대상: `README.md`, `README.ko.md`, `WIP.md`, `.github/workflows/ci.yml`

## 7-Tier 검토

| Tier | 결과 | 근거 |
| --- | --- | --- |
| README parity | 통과 | 영문 및 한국어 README가 동일한 0.2.0 crate 범위, 예시, 보류 track을 설명합니다. |
| WIP parity | 통과 | WIP가 완료된 0.2.0 하위 이슈와 실제 구현 helper surface를 나열합니다. |
| 예시 | 통과 | README snippet이 source에서 확인한 export API를 사용하고, 삽입된 crate 예시의 workspace doctest가 통과합니다. |
| 범위 외부 명확성 | 통과 | Codec, compression, serialization, Testcontainers, SQL, resilience, leader election은 보류 상태입니다. |
| Runtime 주의 사항 | 통과 | Async helper 예시가 Tokio를 명시하고 README가 목적별 crate 사용을 가리킵니다. |
| Coverage 보고 | 통과 | CI가 `cargo llvm-cov`를 실행하고 GitHub step summary에 coverage를 기록하며 `coverage-rust`를 upload합니다. |
| CI trigger 범위 | 통과 | `pull_request.paths-ignore`가 Markdown/docs-only 변경을 건너뛰지만 workflow/source 변경은 CI를 실행합니다. |
| 범위 통제 | 통과 | 문서 및 CI reporting만 변경했으며 Rust source 동작은 변경하지 않았습니다. |
| 위험 | 낮음 | Workflow에 독립적인 coverage job 하나를 추가했지만 기존 check/test/clippy/rustdoc job은 바꾸지 않습니다. |

## 발견 사항

- P0: 0
- P1: 0
- P2: 0
- P3: 0

## 검증

- 통과: README API source alignment check: `Page::with_meta`, `iter::chunks`, `try_map_bounded`, `with_timeout`
- 통과: README image reference check
- 통과: `cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`를 사용한 로컬 coverage 보고
  - Workspace line coverage: 87.00% (1379/1585)
  - `bluetape-rs-collections`: 97.21% (349/359)
  - `bluetape-rs-async`: 88.15% (424/481)
- 통과: `actionlint .github/workflows/ci.yml`
- 통과: CI trigger 검토에서 Markdown/docs-only pull request가 `ci.yml`에서 무시됨을 확인
- 통과: `cargo fmt --all --check`
- 통과: `cargo test --workspace --all-features --locked`
- 통과: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- 통과: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- 통과: `git diff --check`
