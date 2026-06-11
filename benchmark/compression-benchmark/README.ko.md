# compression-benchmark

[English](README.md) | [한국어](README.ko.md)

`bluetape-rs-compression` crate를 위한 동일 조건 compression benchmark runner입니다.

이 runner는 bluetape compression 비교 작업에서 만든 shared payload matrix를 읽고,
Go 및 Kotlin 결과와 병합할 수 있는 CSV를 생성합니다. Payload kind, payload size,
compressor name, direction, throughput, compression ratio를 공통 schema로
유지합니다.

## 범위

- payload kind: `json`, `text`, `binary`, `random`
- payload size: `small`, `medium`, `large`
- compressor: `bluetape_rs_compression::algorithms()`가 반환하는 모든 algorithm
- direction: `compress`, `decompress`
- output: 동일 조건 report와 chart 생성을 위한 CSV row

## 사용 예

```bash
cargo run -p compression-benchmark -- \
  --payload-dir /tmp/bluetape-compression-bench/payloads \
  --output docs/benchmark/compression-same-condition-rust.csv
```

Payload directory에는 전체 `{kind}-{size}.bin` matrix가 있어야 합니다. 예:
`json-small.bin`, `binary-medium.bin`, `random-large.bin`.

## Output Column

- `ecosystem`
- `compressor`
- `direction`
- `payload_kind`
- `payload_size`
- `original_bytes`
- `compressed_bytes`
- `ratio`
- `iterations`
- `total_ns`
- `ns_op`
- `mib_s`
- `timing_provenance`
