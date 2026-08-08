# Compression benchmark 재현성

벤치마크 결과가 `bluetape-rs`, `bluetape-go`, `bluetape4k-io`를 가로지를
때에는 `/tmp` 상태나 기억에 의존하지 않고 비교를 재구성할 수 있을 만큼
로컬 근거를 보고서에 남겨야 합니다.

## 교훈

- fixture hash를 추적되는 manifest에 보관합니다.
- 보고서 옆에 sibling repository revision을 기록합니다.
- chart를 그리기 전에 단위와 provenance를 포함해 CSV schema를 정규화합니다.
- `MiB/s` 또는 `MB/s`를 일관되게 사용하며, Rust의 수작업 계산을 Go benchmark label과 변환 없이 섞지 않습니다.
- 짧은 로컬 실행은 production ranking이 아니라 동일 조건 snapshot으로 표시합니다.
- 생성된 SVG asset에 로컬 filesystem path를 넣지 않습니다.
- 아직 release되지 않은 crate install snippet은 release가 존재할 때까지 path/Git 예시로 유지합니다.
- 필요한 benchmark 행이 없으면 생성 보고서를 즉시 실패하게 합니다. `n/a` placeholder는 불완전한 근거를 숨길 수 있습니다.
- source revision provenance는 이전 구현 commit이 아니라 실제 review baseline에 연결합니다.
- 공개 재현 명령에는 cwd, sibling checkout 가정, cross-repo harness를 추적하지 않을 때의 정확한 BLOCKED 사유를 명시합니다.

## 이슈 #83에 적용한 항목

- `docs/benchmark/compression-fixtures-manifest.csv`
- `docs/benchmark/compression-same-condition-metadata.md`
- `benchmark/compression-benchmark/scripts/normalize_csv.py`
- `benchmark/compression-benchmark/scripts/render_report.py`
- `docs/review/2026-06-11-issue-83-compression-benchmark-review.md`

## 이슈 #81/#82에 적용한 항목

- `benchmark/compression-benchmark/src/main.rs`
- `benchmark/compression-benchmark/go/cmd/generate-payloads`
- `benchmark/compression-benchmark/scripts/render_report.py`
- `docs/benchmark/compression-same-condition-benchmark.md`
- `docs/benchmark/compression-same-condition-metadata.md`
- `docs/review/2026-06-11-issue-81-82-benchmark-evidence-review.md`
