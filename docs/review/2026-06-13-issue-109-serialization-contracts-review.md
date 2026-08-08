# 이슈 #109 구현 검토 - Serialization contract

날짜: 2026-06-13
범위: `origin/develop` 대비 이슈 #109 구현 diff
Gate: Step 6-R implemented diff review

## 검토 범위

- `crates/serialization/src/config.rs`
- `crates/serialization/src/error.rs`
- `crates/serialization/src/format.rs`
- `crates/serialization/src/metadata.rs`
- `crates/serialization/src/traits.rs`
- `crates/serialization/src/trust.rs`
- `crates/serialization/tests/contracts.rs`
- `crates/serialization/README.md`
- `crates/serialization/README.ko.md`
- 구현 commit에서 변경한 root roadmap 및 package docs
- 승인된 contract 범위를 위한 Step 2-R 및 Step 3-R review artifact

## 검증 근거

- `cargo fmt --all --check`
- `git diff --check`
- `cargo tree -p bluetape-rs-serialization --locked --edges normal,features`
- `rg -n "serde_json|bincode|prost|apache-avro|fory|redis|testcontainers|sqlx|bluetape-rs-compression" crates/serialization`
- `cargo test -p bluetape-rs-serialization --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`

## 검토 lane

| Lane | 최초 결과 | 최종 결과 | 근거 |
|---|---:|---:|---|
| Code reviewer | `P0=0 P1=0` | `P0=0 P1=0` | Metadata privacy와 unsafe legacy rejection coverage를 재확인했습니다. |
| Verifier | `P0=0 P1=0` | `P0=0 P1=0` | Validation 명령이 통과했고 `from_parts` Rustdoc P3 gap을 수정했습니다. |
| Security | `P0=0 P1=0` | `P0=0 P1=0` | `SerializedPayload` custom `Debug`가 raw payload byte를 제외하고 redacted adapter error가 source를 노출하지 않음을 확인했습니다. |
| Performance/runtime | `P0=0 P1=0` | `P0=0 P1=0` | Contract layer에 concrete adapter hot path와 broad dependency pull-in이 없으며 후속 benchmark는 adapter milestone로 남겼습니다. |
| Architecture/API | `P0=0 P1=1` | `P0=0 P1=0` | `PayloadMetadata`가 더 이상 public field를 노출하지 않고 constructor와 typed accessor로 metadata를 생성합니다. |
| Library user | `P0=0 P1=0` | `P0=0 P1=0` | README snippet이 일반 Rust block을 사용하고 config가 typed content-type/version setter를 지원하며 zero payload limit을 거부합니다. |

## 통합 발견 사항 및 수정

| 우선순위 | 영역 | 해결 |
|---|---|---|
| P1 | 공개 metadata 형태 | `PayloadMetadata` field를 private으로 만들고 `new`, accessor를 추가했으며 struct literal을 사용하던 test를 갱신했습니다. |
| P2 | Payload byte 진단 | `SerializedPayload`의 derived `Debug`를 metadata와 `bytes_len`만 출력하는 custom 구현으로 바꿨습니다. |
| P2 | Config ergonomics | 이미 검증된 typed value를 사용하는 `SerializationConfig::with_content_type`와 `SerializationConfig::with_version`을 추가했습니다. |
| P2 | Metadata policy 구성 | `PayloadMetadataPolicy::from_parts`가 `Result`를 반환하고 zero `max_payload_size`를 거부하도록 변경했습니다. |
| P3 | Unsafe legacy coverage | Safe trust-profile setter가 `UnsafeLegacyCompatibility`를 거부하는 명시적 test coverage를 추가했습니다. |
| P3 | 공개 Rustdoc | 공개 `from_parts` `Result` API에 `# Errors` 문서를 추가했습니다. |
| P3 | README 예시 | 두 locale의 일반 README code fence에서 Rustdoc-hidden `#` marker를 제거했습니다. |

## 보류 후속 검사

- Concrete binary adapter 작업은 실제 adapter parser error와 malformed input case에 대해 redaction test를 다시 실행해야 합니다.
- Adapter milestone은 benchmark 근거를 추가해야 합니다. 이 contract 이슈는 benchmark 가능한 경계만 정의하며 runtime performance를 주장하지 않습니다.
- Protobuf, Avro, Fory, cross-language compatibility 검사는 이후 `0.5.x` milestone으로 보류합니다.

## Gate 판정

Blocker 수정 및 영향 lane 재검토 후 Step 6-R 통과

P0=0 P1=0
