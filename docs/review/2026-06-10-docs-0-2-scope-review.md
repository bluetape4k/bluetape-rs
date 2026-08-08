# 0.2.0 문서 범위 검토

## 범위

- 이슈: #24
- 마일스톤: 0.2.0
- 변경 표면: `README.md`, `README.ko.md`, `WIP.md`, `.github/workflows/ci.yml`

## 7-Tier 검토

| 단계 | 결과 | 근거 |
| --- | --- | --- |
| README 동등성 | Pass | 영어와 한국어 README가 같은 0.2.0 크레이트 범위, 예제, 보류 트랙을 설명한다. |
| WIP 동등성 | Pass | WIP가 완료된 0.2.0 하위 이슈와 실제 구현된 헬퍼 표면을 나열한다. |
| 예제 | Pass | README 조각이 소스에서 확인한 export API를 사용하고 내장 크레이트 예제의 워크스페이스 doctest가 통과한다. |
| 범위 제외 명확성 | Pass | 코덱, 압축, 직렬화, Testcontainers, SQL, resilience, 리더 선출은 보류 상태다. |
| 런타임 주의 사항 | Pass | 비동기 헬퍼 예제가 Tokio를 명시적으로 유지하고 README가 집중 크레이트 사용을 가리킨다. |
| 커버리지 보고 | Pass | CI가 이제 `cargo llvm-cov`를 실행하고 GitHub step summary에 커버리지를 기록하며 `coverage-rust`를 업로드한다. |
| CI 트리거 범위 | Pass | `pull_request.paths-ignore`가 Markdown/문서만 변경한 경우를 건너뛰고 workflow/소스 변경은 계속 CI를 실행한다. |
| 범위 통제 | Pass | 문서 및 CI 보고만 변경하며 Rust 소스 동작은 변경하지 않는다. |
| 위험 | 낮음 | workflow가 독립적인 커버리지 작업 하나를 추가하지만 기존 check/test/clippy/rustdoc 작업은 변경하지 않는다. |

## 발견 사항

- P0: 0
- P1: 0
- P2: 0
- P3: 0

## 검증

- 통과: README API의 source alignment 검사: `Page::with_meta`, `iter::chunks`,
  `try_map_bounded` 및 `with_timeout`
- 통과: README image reference 검사
- 통과: `cargo llvm-cov --workspace --all-features --locked --lcov --output-path coverage/lcov.info`를
  사용한 로컬 커버리지 보고서
  - 워크스페이스 라인 커버리지: 87.00% (1379/1585)
  - `bluetape-rs-collections`: 97.21% (349/359)
  - `bluetape-rs-async`: 88.15% (424/481)
- 통과: `actionlint .github/workflows/ci.yml`
- 통과: CI trigger 검토에서 Markdown/docs-only pull request를 `ci.yml`이
  무시함을 확인
- 통과: `cargo fmt --all --check`
- 통과: `cargo test --workspace --all-features --locked`
- 통과: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- 통과: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
- 통과: `git diff --check`
