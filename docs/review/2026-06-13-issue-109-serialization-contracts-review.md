# 이슈 #109 구현 검토 - 직렬화 계약

날짜: 2026-06-13
범위: `origin/develop` 대비 이슈 #109 구현 diff
게이트: Step 6-R 구현 diff 검토

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
- 구현 커밋에서 변경한 루트 로드맵 및 패키지 문서
- 승인된 계약 범위에 대한 Step 2-R 및 Step 3-R 검토 산출물

## 검증 근거

- `cargo fmt --all --check`
- `git diff --check`
- `cargo tree -p bluetape-rs-serialization --locked --edges normal,features`
- `rg -n "serde_json|bincode|prost|apache-avro|fory|redis|testcontainers|sqlx|bluetape-rs-compression" crates/serialization`
- `cargo test -p bluetape-rs-serialization --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`

## 검토 레인

| 레인 | 초기 결과 | 최종 결과 | 근거 |
|---|---:|---:|---|
| 코드 검토자 | `P0=0 P1=0` | `P0=0 P1=0` | 메타데이터 비공개 여부와 안전하지 않은 레거시 거부 범위를 재확인했다. |
| 검증자 | `P0=0 P1=0` | `P0=0 P1=0` | 검증 명령이 통과했으며 `from_parts`의 P3 Rustdoc 누락을 수정했다. |
| 보안 | `P0=0 P1=0` | `P0=0 P1=0` | `SerializedPayload`의 사용자 지정 `Debug`가 원시 페이로드 바이트를 제외하며, 비식별화한 어댑터 오류가 소스를 노출하지 않는다. |
| 성능/런타임 | `P0=0 P1=0` | `P0=0 P1=0` | 계약 계층에는 구체적인 어댑터 핫 패스나 광범위한 의존성 유입이 없으며, 후속 벤치마크는 어댑터 마일스톤으로 남겼다. |
| 아키텍처/API | `P0=0 P1=1` | `P0=0 P1=0` | `PayloadMetadata`가 더 이상 공개 필드를 노출하지 않으며, 메타데이터 생성을 생성자와 타입 지정 접근자로 제한했다. |
| 라이브러리 사용자 | `P0=0 P1=0` | `P0=0 P1=0` | README 예제가 일반 Rust 블록을 사용하고, 설정이 타입이 지정된 content-type/version setter를 지원하며, 0 페이로드 한도를 거부한다. |

## 통합 발견 사항 및 수정

| 우선순위 | 영역 | 해결 |
|---|---|---|
| P1 | 공개 메타데이터 형태 | `PayloadMetadata` 필드를 비공개로 바꾸고 `new` 및 접근자를 추가했으며, 테스트에서 구조체 리터럴을 사용하지 않도록 수정했다. |
| P2 | 페이로드 바이트 진단 | `SerializedPayload`의 파생 `Debug`를 메타데이터와 `bytes_len`만 출력하는 사용자 지정 구현으로 교체했다. |
| P2 | 설정 사용성 | 이미 검증된 타입 값을 사용하는 `SerializationConfig::with_content_type` 및 `SerializationConfig::with_version`을 추가했다. |
| P2 | 메타데이터 정책 생성 | `PayloadMetadataPolicy::from_parts`가 `Result`를 반환하도록 바꾸고 0 `max_payload_size`를 거부했다. |
| P3 | 안전하지 않은 레거시 범위 | 안전한 trust-profile setter가 `UnsafeLegacyCompatibility`를 거부하는 명시적인 테스트를 추가했다. |
| P3 | 공개 Rustdoc | 공개 `from_parts` `Result` API에 `# Errors` 문서를 추가했다. |
| P3 | README 예제 | 두 로케일 파일의 일반 README 코드 fence에서 Rustdoc 전용 숨김 `#` 표식을 제거했다. |

## 보류한 후속 확인

- 구체적인 바이너리 어댑터 작업에서는 실제 어댑터 파서 오류와 잘못된 입력
  사례를 대상으로 비식별화 테스트를 다시 실행해야 한다.
- 어댑터 마일스톤에는 벤치마크 근거를 추가해야 한다. 이 계약 이슈는
  벤치마크 가능한 경계만 정의하며 런타임 성능을 주장하지 않는다.
- Protobuf, Avro, Fory 및 언어 간 호환성 확인은 이후 `0.5.x` 마일스톤으로
  보류했다.

## 게이트 판정

차단 요소를 수정하고 영향을 받은 레인을 다시 검토한 뒤 Step 6-R을 통과했다.

P0=0 P1=0
