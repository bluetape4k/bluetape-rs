# 직렬화 계약 설계

날짜: 2026-06-13
상태: Step 2-R 검토 완료, P0=0 P1=0
범위: 이슈 #109, 마일스톤 `0.5.0`, `crates/serialization`

## 문제

이슈 #108은 `bluetape-rs-serialization` 크레이트 경계와 선택적 루트 파사드를
생성했지만, 크레이트는 아직 직렬화 계약을 노출하지 않는다. 이슈 #109는
이후 `0.5.0` 작업이 기반으로 삼을 공개 용어를 정의해야 한다. 여기에는
명시적인 형식 식별자, 타입이 지정된 오류, 페이로드 메타데이터, 신뢰
프로파일, 안전한 구성 기본값이 포함된다.

이 이슈는 첫 어댑터 구현이 아니다. 바이너리, JSON, Protobuf, Avro, Fory
어댑터는 이후 마일스톤 이슈에 남긴다. #109의 결과물은 공개 API 형태를 다시
변경하지 않고 해당 어댑터 이슈를 구현할 수 있게 하는 작은 Rust 네이티브
계약 계층이다.

## 현재 근거

- `crates/serialization/src/lib.rs`에는 현재 크레이트 Rustdoc과 메타데이터
  스모크 테스트만 있다. serializer 트레이트, 페이로드 envelope, 어댑터는
  이후 검토 이슈라고 명시한다.
- `WIP.md`는 첫 바이너리 어댑터 전에 타입이 지정된 serializer/deserializer
  계약, 타입이 지정된 오류, 형식 ID, 콘텐츠 타입, 버전, 신뢰 프로파일
  용어를 나열한다.
- `docs/superpowers/specs/2026-06-13-serde-0-5x-design.md`는 `0.5.0`이
  캐시 우선 및 바이너리 우선이고 JSON, Protobuf, Avro, Fory, 저장소 간
  벤치마크는 별도의 `0.5.x` 마일스톤임을 명시한다.
- 공식 Serde 문서는 데이터 모델 트레이트인 `serde::Serialize` 및
  `serde::Deserialize`를 구체적인 데이터 형식 크레이트와 분리한다. 또한
  일반적인 형식 크레이트 구조를 별도의 `ser`, `de`, `error` 모듈로
  보여 준다.
- Kotlin/JVM `bluetape4k-projects`에는 신뢰 프로파일에 대한 유용한 도메인
  근거가 있지만, 이전 보안 검토는 동적 클래스 로딩, 대체 객체 역직렬화,
  모두 허용하는 기본값이 실제 역직렬화 위험임을 보여 준다.
- `bluetape-go`에는 `Serializer[T]`, `NamedSerializer[T]`, 버전이 지정된
  envelope가 있는 작은 `serialization` 패키지가 있다. 도메인 근거로는
  유용하지만 이 Rust 크레이트는 Go 메서드 이름이나 인터페이스 형태를
  기계적으로 이식해서는 안 된다.
- 자매 Rust 크레이트는 `lib.rs`를 간결하게 유지하고 구현을 집중된
  모듈로 나눈다. `crates/compression`은 `mod config`, `mod error`,
  `mod registry`, `mod stream`, `mod traits`를 사용하므로 #109도
  `lib.rs`를 확장하는 대신 이 형태를 따른다.

## 목표

- 숨겨진 전역 기본값 없이 명시적이고 타입이 지정된 형식 식별자를 정의한다.
- `serde::Serialize` 및 `serde::de::DeserializeOwned`와 호환되는 타입 지정
  serializer/deserializer 계약을 정의한다.
- 캐시 및 인프라 페이로드용 바이너리 페이로드 메타데이터를 정의한다.
  콘텐츠 타입, 페이로드 버전, 형식 ID, 신뢰 프로파일, 어댑터 ID, 페이로드
  크기를 포함한다.
- 인코드, 디코드, 구성 검증, 형식 불일치, 콘텐츠 타입 불일치, 버전 불일치,
  신뢰 프로파일 불일치, 잘못된 입력, 초과 페이로드, 어댑터 실패를 위한
  타입 지정 오류 열거형을 정의한다.
- bluetape 생태계에 맞는 신뢰 프로파일 용어를 정의한다. 신뢰된 내부,
  허용 목록 타입, 정적 타입, 안전하지 않은 레거시 호환성을 포함한다.
- 문서화된 근거와 함께 안전한 구성 기본값을 정의한다.
- 바이트/문자열 경계를 명시적으로 유지한다.
- 압축, 코덱, 직렬화 관심사를 분리한다.

## 비목표

- #109에서는 바이너리 어댑터를 구현하지 않는다.
- JSON, Protobuf, Avro, Fory, 스키마 레지스트리, 다언어 운영 어댑터를
  구현하지 않는다.
- 압축 조합을 구현하지 않는다.
- 동적 레지스트리, 숨겨진 기본 serializer, 환경 변수로 선택하는 어댑터,
  페이로드가 선택하는 Rust 타입을 사용하지 않는다.
- Testcontainers, Redis, 데이터베이스, SQLx, resilience, 벤치마크 harness를
  사용하지 않는다.
- Kotlin/JVM 또는 Go API 동등성을 약속하지 않는다.

## 제안 설계

승인된 B 접근법, 즉 Rust 네이티브 타입 지정 계약 모듈을 사용한다.

`lib.rs`는 크레이트 수준의 인덱스 및 export 표면으로 유지한다.

- `mod config;`
- `mod error;`
- `mod format;`
- `mod metadata;`
- `mod traits;`
- `mod trust;`

공개 export는 이러한 집중 모듈에서 제공한다. 긴 설명은 `lib.rs`가 아니라
각 타입의 RustDoc과 README/사양 문서에 둔다.

### 형식 용어

`SerializationFormat`은 전역 레지스트리가 아니라 검증된 작은 값 타입이다.
`binary`, `json`, `protobuf`, `avro`와 같은 안정적인 형식 ID나 나중에
추가되는 어댑터 전용 ID를 나타낸다.

규칙:

- 형식 ID는 ASCII 소문자 토큰이다.
- 허용 문자는 `a-z`, `0-9`, `-`, `_`, `.`, `/`이다.
- 비어 있거나 공백인 ID, 제어 문자가 포함된 ID, 대문자 ID, 지나치게 긴
  ID는 거부한다.
- 타입은 소유 문자열을 저장하므로 이후 어댑터가 열거형을 변경하지 않고
  안정적인 사용자 지정 ID를 정의할 수 있다.

이렇게 하면 모든 어댑터마다 semver에 드러나는 변경이 필요한 닫힌 열거형을
피하면서도 임의의 미검증 메타데이터를 막을 수 있다.

### 신뢰 프로파일

`SerializationTrustProfile`은 닫힌 열거형이다.

- `TrustedInternal`: 하나의 신뢰된 배포 내부에 있는 비공개 캐시 또는 큐.
- `AllowListedTypes`: 타입 메타데이터를 포함할 수 있지만 명시적인 허용
  목록으로 제한하는 형식.
- `StaticallyTyped`: 페이로드가 런타임 타입을 선택하지 않고 호출자가 Rust
  대상 타입을 제공하는 형식.
- `UnsafeLegacyCompatibility`: 모두 허용하거나 레거시 호환 경로를 위한
  마이그레이션 전용 경계.

#109 기본 구성은 `StaticallyTyped`를 사용한다. Rust 호출자는
`DeserializeOwned`를 통해 대상 타입을 제공해야 하며 `0.5.0`에서는 동적
타입 로딩을 활성화해서는 안 되기 때문이다.

### 메타데이터

`PayloadMetadata`는 페이로드 바이트를 노출하지 않고 페이로드를 설명한다.

- `format: SerializationFormat`
- `content_type: ContentType`
- `version: PayloadVersion`
- `trust_profile: SerializationTrustProfile`
- `adapter_id: AdapterId`
- `payload_size: usize`

`ContentType`, `PayloadVersion`, `AdapterId`는 검증된 값 타입이다. 모든
메타데이터 토큰 한도는 공개 계약의 일부이므로 구현과 테스트가 서로 다른
경계를 임의로 만들지 않는다.

- 형식 ID 길이는 1..=64바이트이고 소문자 ASCII만 허용하며 허용 바이트는
  `a-z`, `0-9`, `-`, `_`, `.`, `/`이다.
- 콘텐츠 타입 길이는 1..=127바이트이고 소문자 ASCII만 허용한다. `/`는
  정확히 하나여야 하며 파라미터, 공백, 제어 문자는 허용하지 않고
  `a-z`, `0-9`, `!`, `#`, `$`, `&`, `^`, `_`, `.`, `+`, `-`와 같은 표시 가능한
  미디어 타입 토큰 바이트만 허용한다.
- 페이로드 버전은 양의 `u16`이다.
- 어댑터 ID 길이는 1..=64바이트이고 소문자 ASCII만 허용하며 허용 바이트는
  `a-z`, `0-9`, `-`, `_`, `.`이다.
- 페이로드 크기는 메타데이터일 뿐이며 페이로드 바이트를 로깅하거나
  저장하지 않는다.

`SerializedPayload`는 인코딩된 바이트와 메타데이터를 함께 소유한다.
생성자는 `metadata.payload_size == bytes.len()`을 계산하거나 검증하므로
호출자가 실제 인코딩된 페이로드와 다른 메타데이터를 게시할 수 없다.

### 트레이트

공개 트레이트는 형식에 종속되지 않으며 바이트 우선이다.

```rust
pub trait Serializer<T>
where
    T: serde::Serialize,
{
    fn serialize(&self, value: &T) -> Result<SerializedPayload, SerializationError>;
}

pub trait Deserializer<T>
where
    T: serde::de::DeserializeOwned,
{
    fn deserialize(&self, payload: &SerializedPayload) -> Result<T, SerializationError>;
    fn expected_metadata(&self) -> PayloadMetadataPolicy;
}

pub trait BinarySerializer<T>: Serializer<T> + Deserializer<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
}
```

구체적인 계획에서 메서드 이름은 다듬을 수 있지만 다음 제약은 유지해야 한다.

- 원시 바이트 입력은 `SerializedPayload::try_from_parts` 또는 디코드 전에
  메타데이터를 검증하는 동등한 생성자로 사용할 수 있어야 한다.
- 인코드 출력은 `Vec<u8>` 바이트를 포함하는 소유된
  `SerializedPayload`여야 한다.
- 인코드 입력 값은 빌릴 수 있어야 한다.
- 디코드 대상 타입은 Rust 제네릭을 통해 호출자가 제공한다.
- 메타데이터 페이로드 크기는 `bytes.len()`에서 계산하거나 그 값과 비교해
  검증한다.
- #109 API에는 `Any`, `TypeId`, 동적 타입 레지스트리, 페이로드가 선택하는
  타입이 나타나지 않는다.

`PayloadMetadataPolicy`는 어댑터가 존재하기 전에 deserializer가 기대하는
내용을 나타낸다. 결정적인 일치 규칙을 정의해야 한다.

- `format`과 `content_type`은 정확히 일치해야 한다.
- 이후 어댑터가 자체 이슈에서 더 좁은 호환성 규칙을 명시적으로 설계하지
  않는 한 `trust_profile`도 정확히 일치해야 한다.
- `max_supported_version`은 포함 범위다. 관측된 `0`은 잘못된 값이며 최댓값을
  넘는 관측 값은 지원하지 않는 버전 오류를 반환한다.
- 어댑터 ID 일치는 선택적인 정책 메타데이터이며 동적 레지스트리 조회가
  아니다.
- 불일치 결과는 형식, 콘텐츠 타입, 신뢰 프로파일, 버전, 잘못된 메타데이터,
  초과 페이로드에 해당하는 타입 지정 오류 변형으로 매핑한다.

### 오류

타입이 지정된 `SerializationError` 열거형을 사용한다. 변형은 안전한 진단
정보를 보존해야 한다.

- 인코드 또는 디코드 방향.
- 해당하는 경우 예상 및 관측 형식 ID.
- 해당하는 경우 예상 및 관측 콘텐츠 타입.
- 해당하는 경우 예상 및 관측 페이로드 버전.
- 해당하는 경우 예상 및 관측 신뢰 프로파일.
- 해당하는 경우 어댑터 ID.
- 해당하는 경우 페이로드 크기와 한도.
- 페이로드 바이트를 포함하지 않는 잘못된 입력 사유.
- 이후 어댑터가 제공할 수 있는 경우 어댑터 실패의 소스 오류.

공개 오류는 `std::error::Error`, `Display`, `Debug`를 구현해야 하며 소스를
첨부하는 경우 `Send + Sync`여야 한다. 어댑터 실패 소스는 표시해도 안전한
경우 `Box<dyn std::error::Error + Send + Sync + 'static>`에 해당하는 저장
형태를 사용해야 한다. 업스트림 오류에 원시 페이로드 바이트나 조각이
포함될 수 있다면, 어댑터는 `SerializationError`에 첨부하기 전에 삭제된
소스로 감싸야 한다. `Display`, `Debug`, `source()` 순회 테스트는 식별 가능한
페이로드 표식이 노출되지 않음을 증명해야 한다. 워크스페이스가 이미
`crates/compression`에 `thiserror`를 사용하므로 이를 사용해도 된다.

### 구성 기본값

`SerializationConfig`은 이후 어댑터를 위한 안전한 기본값을 정의한다.

- 기본 신뢰 프로파일: `StaticallyTyped`.
- 기본 콘텐츠 타입: `application/octet-stream`.
- 기본 페이로드 버전: `1`.
- 기본 최대 페이로드 크기: `16 * 1024 * 1024`바이트.
  `bytes.len() > max_payload_size`인 페이로드는 디코드 전에 실패하고,
  정확히 같은 크기의 페이로드는 허용한다.
- 구체적인 어댑터에는 어댑터 ID가 필요하다.
- fallback serializer를 사용하지 않는다.
- 숨겨진 압축을 사용하지 않는다.

구성 검증은 타입이 지정된 오류를 반환하고, 0인 최대 크기, 0인 페이로드
버전, 안전하지 않은 레거시 호환성 기본값, 공백 어댑터 ID, 잘못된
메타데이터 토큰을 거부한다.

### 의존성 정책

이슈 #109에서는 다음을 추가할 수 있다.

- 공개 트레이트 경계를 위한 워크스페이스 의존성 `serde`. `std`를 지원하고
  이 크레이트에서 `derive` feature를 요구하지 않는다.
- 타입이 지정된 오류를 위한 `crates/serialization`의
  `thiserror.workspace = true`.

`bincode`, `serde_json`, `prost`, `apache-avro`, `fory`, Redis, Testcontainers,
압축 어댑터와 같은 어댑터 의존성은 추가하지 않는다.

## 채택하지 않은 접근법

### Go `Serializer[T]` 직접 이식

Go의 `Marshal`, `Unmarshal` 이름과 `NamedSerializer` 인터페이스는 작고
`bluetape-go`에서 검증되었지만, 이를 직접 이식하면 Rust의 `serde` 트레이트
생태계와 소유권 관례를 무시하게 된다. Rust API는 빌린 인코드 입력,
명시적인 `DeserializeOwned` 디코드 대상, Rust 오류 타입을 사용해야 한다.

### 모든 형식을 위한 닫힌 열거형

닫힌 `enum SerializationFormat { Binary, Json, Protobuf, Avro, Fory }` 열거형은
단순해 보이지만 이후의 모든 어댑터나 사용자 정의 형식마다 공개 열거형을
변경해야 한다. 검증된 문자열 newtype은 확장성을 유지하면서 안정적인
검증을 제공한다.

### 지금 동적 레지스트리 도입

동적 레지스트리는 이후 어댑터 조회를 편리하게 만들지만 숨겨진 기본값과
페이로드가 선택하는 동작도 만든다. #109에서는 의도적으로 어댑터 선택을
호출자가 소유하고 명시적으로 수행하게 한다.

## 캐시 롤아웃 및 운영 지침

이슈 #109에서는 캐시 저장, 캐시 키, 캐시 제거를 구현하지 않는다. 그러나
메타데이터 계약은 이후 캐시 사용자가 형식 및 버전 변경을 안전하게 운영하는
방법을 설명해야 한다.

- `PayloadVersion`은 직렬화 페이로드 계약을 설명하며 캐시 네임스페이스
  자체를 설명하지 않는다.
- 형식 ID, 콘텐츠 타입, 신뢰 프로파일 또는 호환되지 않는 페이로드 버전을
  변경할 때 호출자는 캐시 네임스페이스나 키 접두사의 버전을 변경해야 한다.
- 불일치는 안전한 메타데이터 진단과 함께 즉시 거부한다. 계약은 조용히
  디코드하거나 다른 어댑터로 fallback하거나 `None`을 반환해서는 안 된다.
- 권장 운영자 조치는 명시적이다. 항목을 제거하고, 기준 소스에서 재구축하고,
  네임스페이스를 마이그레이션하거나, 계획된 롤아웃 외부에서 불일치가
  발생하면 알림을 보낸다.
- 롤백 동작도 명시적이다. 더 오래된 reader가 지원하지 않는 최신 버전을
  만나면 관측된 버전과 지원 가능한 최대 버전 메타데이터를 포함한 타입이
  지정된 지원하지 않는 버전 오류를 반환한다.
- 관측 필드는 카디널리티가 낮고 페이로드를 포함하지 않아야 한다. 오류
  종류, 방향, 형식 ID, 콘텐츠 타입, 버전 관계, 신뢰 프로파일, 어댑터 ID,
  페이로드 크기 구간, 구성된 크기 한도를 사용한다. 원시 페이로드 바이트와
  제한 없는 페이로드 조각은 로그, 메트릭, trace 필드가 될 수 없다.
- `UnsafeLegacyCompatibility`는 완전히 신뢰된 배포에서만 사용하는 임시
  마이그레이션 경계로 문서화해야 하며, 일반적인 운영 기본값으로 사용하지
  않는다.

## 위험 및 실패 모드

1. **지나치게 넓은 공개 API.** 지금 #109에서 어댑터별 동작을 정의하면
   #111 및 이후 어댑터 이슈가 잘못된 추상화를 물려받는다. 완화책:
   계약만 정의하고 어댑터는 구현하지 않는다.
2. **용어 드리프트로 인한 보안 회귀.** 신뢰 프로파일 이름이 Kotlin/JVM
   문서와 다르면 저장소 간 지침이 혼란스러워진다. 완화책: 네 가지
   bluetape 신뢰 프로파일을 Rust 네이티브 이름으로 사용한다.
3. **형식 ID의 Semver 함정.** 닫힌 열거형은 이후 형식 ID를 열거형 변형에
   넣도록 강제한다. 완화책: ID에 검증된 newtype을 사용한다.
4. **숨겨진 페이로드 누출.** 오류 메시지에 원시 바이트가 실수로 포함될 수
   있다. 완화책: 오류 변형에는 메타데이터와 사유 문자열만 저장하고,
   어댑터 소스 오류는 표시해도 안전하거나 삭제한 뒤에만 첨부한다.
5. **불명확한 캐시 마이그레이션 의미론.** 버전 불일치가 fallback 디코드로
   이어질 수 있다. 완화책: #109에서 불일치를 타입이 지정된 오류로 기록하고
   네임스페이스 마이그레이션, 제거, 재구축, 알림 경로를 호출자가 소유하는
   운영 정책으로 문서화한다.

## 인수 기준

- `crates/serialization/src/lib.rs`는 간결하게 유지되고 집중 모듈을 export한다.
- 공개 계약은 Rust 2024로 컴파일되고 `serde` 호환 경계를 사용한다.
- 공개 값 타입은 형식 ID, 콘텐츠 타입, 어댑터 ID, 페이로드 버전을 검증한다.
- 공개 오류는 안전한 메타데이터 맥락을 보존하고 페이로드 바이트를 노출하지
  않는다.
- 구성 기본값을 문서화하고 테스트한다.
- 신뢰 프로파일 용어는 신뢰된 내부, 허용 목록 타입, 정적 타입, 안전하지
  않은 레거시 호환성을 포함한다.
- `SerializedPayload` 또는 동등한 공개 생성 방식은 메타데이터
  `payload_size`가 인코딩된 바이트 길이와 달라지지 않게 한다.
- `PayloadMetadataPolicy`는 형식/콘텐츠/신뢰의 정확한 일치, 포괄적인
  최대 버전 동작, 선택적인 어댑터 ID 일치, 불일치 오류 매핑을 정의한다.
- 형식 ID, 콘텐츠 타입, 어댑터 ID의 최대 길이와 허용 바이트를 테스트한다.
- 테스트는 유효한 기본값, 잘못된 메타데이터 토큰, 버전 불일치, 형식
  불일치, 콘텐츠 타입 불일치, 신뢰 프로파일 불일치, 페이로드 크기 한도,
  구성 검증 실패, 메타데이터 바이트 길이 일관성, 안전한
  `Display`/`Debug`/`source()` 동작을 다룬다.
- Rustdoc에는 구성 생성, 어댑터 구현 형태, 인코드/디코드 사용, 메타데이터
  정책 검증, 타입 지정 오류 일치를 위한 컴파일 검증 예제가 포함된다.
- `README.md`와 `README.ko.md`는 동기화 상태를 유지하고 #109가 계약만
  추가하며 어댑터는 이후 이슈이고 동적 레지스트리나 페이로드 선택 타입이
  없으며 `UnsafeLegacyCompatibility`가 마이그레이션 전용 용어임을 명시한다.
- 운영 지침은 혼합 버전 배포, 캐시 네임스페이스/키 접두사 버전 관리,
  불일치 조치, 롤백 동작, 페이로드 없는 진단 필드를 다룬다.
- JSON, Protobuf, Avro, Fory, Testcontainers, Redis, SQLx, resilience,
  벤치마크 의존성을 추가하지 않는다.

## 검증 계획

- `cargo fmt --all --check`
- `cargo test -p bluetape-rs-serialization --all-features --locked`
- `cargo test --workspace --all-features --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `git diff --check`

## Step 2 DoD

| 항목 | 상태 |
|---|---|
| 어댑터 구현과 분리된 이슈 #109 범위 | Required |
| Rust 네이티브 모듈 분할 지정 | Required |
| Serde 호환성과 의존성 정책 지정 | Required |
| 신뢰 프로파일 용어 지정 | Required |
| 타입 지정 메타데이터/오류/구성 요구 사항 지정 | Required |
| 페이로드 크기 일관성과 캐시 롤아웃 지침 지정 | Required |
| 테스트 및 검증 명령 지정 | Required |
