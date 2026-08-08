# 교훈: Serialization contract

날짜: 2026-06-13
범위: 이슈 #109, serialization format/error/metadata/config contract

## 변경 사항

- 이슈에서 JSON, Protobuf, Avro, Fory 또는 구체적인 binary adapter를 선택하지 않고 호출자가 제공하는 `serde` type을 중심으로 Rust-native serialization trait를 정의했습니다.
- Typed format, content type, adapter id, payload version, trust profile, metadata, metadata policy, config, error contract를 추가했습니다.
- Payload bytes와 metadata는 함께 유지하되, 기본 진단에는 payload를 포함하지 않았습니다.
- Adapter dependency 경계를 보존했습니다. Contract crate는 `serde`와 `thiserror`를 사용하지만 구체적인 adapter crate는 사용하지 않습니다.

## 교훈

- 공개 metadata struct는 첫 contract 릴리스에서 모든 field를 노출하면 안 됩니다. private field와 typed accessor를 사용하면 caller를 struct literal에 고정하지 않고 metadata를 확장할 여지가 남습니다.
- Cache payload를 담을 수 있는 payload container에는 `Debug`를 derive하면 안 됩니다. Custom debug 출력에는 raw bytes 대신 길이와 metadata를 노출해야 합니다.
- Trust-profile vocabulary만으로는 충분하지 않습니다. Safe setter는 unsafe legacy mode를 거부해야 하며, migration-only opt-in은 의도가 드러나는 이름으로 지정해야 합니다.
- README code fence와 Rustdoc example은 요구 사항이 다릅니다. 숨김 Rustdoc marker는 Rustdoc에서 유효하지만 일반 README example은 일반적인 코드처럼 읽혀야 합니다.

## 누락을 발견한 검사

- Step 6-R architecture review에서 PR 전에 public-field metadata 형태를 발견했습니다.
- Step 6-R security review에서 raw payload debug 위험을 발견했습니다.
- Step 6-R library-user review에서 README snippet 품질과 config setter ergonomics 문제를 발견했습니다.
- Step 6-R verifier review에서 공개 `Result` API에 필요한 `# Errors` Rustdoc section이 빠진 것을 발견했습니다.

## 앞으로의 규칙

향후 serialization adapter는 각각 별도 milestone 뒤에 두고, 동일한 scenario
matrix를 요구합니다. Matrix에는 metadata mismatch, version mismatch,
trust-profile mismatch, oversized payload, malformed bytes, safe 및 redacted
adapter failure, README parity, 성능을 주장할 때의 benchmark 근거를
포함합니다.
