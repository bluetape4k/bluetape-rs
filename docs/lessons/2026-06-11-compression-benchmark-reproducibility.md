# 압축 벤치마크 재현성

벤치마크 결과가 `bluetape-rs`, `bluetape-go`, `bluetape4k-io` 여러 저장소에
걸쳐 비교될 때는 `/tmp` 상태나 기억에 의존하지 않고 비교를 재구성할 수
있을 만큼 충분한 로컬 근거를 보고서에 남겨야 한다.

## 교훈

- 픽스처 해시는 추적되는 매니페스트에 보관한다.
- 보고서 옆에 자매 저장소의 리비전을 기록한다.
- 차트를 만들기 전에 단위와 출처를 포함한 CSV 스키마를 정규화한다.
- `MiB/s` 또는 `MB/s` 중 하나를 일관되게 사용한다. 변환하지 않은 Rust의
  수동 계산값과 Go 벤치마크 라벨을 섞지 않는다.
- 짧은 로컬 실행은 운영 환경 순위가 아니라 동일 조건 스냅숏으로 표시한다.
- 생성된 SVG 에셋에 로컬 파일 시스템 경로를 넣지 않는다.
- 릴리스가 존재하기 전까지는 아직 릴리스되지 않은 크레이트의 설치 조각을
  path/Git 예제로 유지한다.
- 필수 벤치마크 행이 없으면 생성 보고서가 즉시 실패하게 한다. `n/a`
  자리표시자는 불완전한 근거를 숨길 수 있다.
- 소스 리비전 출처는 이전 구현 커밋이 아니라 실제 검토 기준점에 연결한다.
- 공개 재현 명령에는 cwd, 자매 체크아웃 전제 조건, 그리고 저장소 간
  하네스가 추적되지 않을 때의 정확한 BLOCKED 사유를 명시한다.

## 이슈 #83에 적용

- `docs/benchmark/compression-fixtures-manifest.csv`
- `docs/benchmark/compression-same-condition-metadata.md`
- `benchmark/compression-benchmark/scripts/normalize_csv.py`
- `benchmark/compression-benchmark/scripts/render_report.py`
- `docs/review/2026-06-11-issue-83-compression-benchmark-review.md`

## 이슈 #81/#82에 적용

- `benchmark/compression-benchmark/src/main.rs`
- `benchmark/compression-benchmark/go/cmd/generate-payloads`
- `benchmark/compression-benchmark/scripts/render_report.py`
- `docs/benchmark/compression-same-condition-benchmark.md`
- `docs/benchmark/compression-same-condition-metadata.md`
- `docs/review/2026-06-11-issue-81-82-benchmark-evidence-review.md`
