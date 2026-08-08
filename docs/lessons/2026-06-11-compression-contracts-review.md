# Compression contract 검토에서 얻은 교훈

## 배경

이슈 #75는 0.4.0 compression 라인의 첫 공개 compression config, error,
stream contract를 추가했습니다.

## 교훈

- Safety-limit contract는 allocation을 고려해야 합니다. lz4와 snappy one-shot decoder는 전체 output buffer를 할당하는 API를 호출하기 전에 선언된 decompressed size를 확인해야 합니다.
- 공개 trait 확장은 가능하면 기존 implementor의 source 호환성을 보존해야 합니다. required method를 추가하기 전에 default method를 추가하거나 extension trait를 분리합니다.
- 빈 default feature set에는 테스트뿐 아니라 lint gate도 필요합니다. `default = []`는 `-D warnings`를 사용하는 no-default clippy를 통과해야 합니다.
- Streaming contract에는 실패 경로 테스트가 필요합니다. 실패하는 `Read`와 `Write` 구현을 주입해 성공한 `Vec` round-trip뿐 아니라 typed error variant와 `source()` 순회도 증명합니다.
- Rustdoc은 wire-format 주의 사항을 API 발견 지점에 담아야 합니다. registry enum variant가 one-shot과 stream helper를 모두 노출하면 README 경고만으로는 충분하지 않습니다.

## 후속 방어 규칙

향후 codec/compression API에서는 공개 trait의 source-compatibility test와
feature-matrix clippy를 로컬 Step 6-R 근거에 포함한 뒤 PR을 생성합니다.
