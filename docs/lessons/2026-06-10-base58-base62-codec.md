# Base58 및 Base62 코덱에서 얻은 교훈

날짜: 2026-06-10
이슈: #63

## 발생한 일

Base58과 Base62를 Kotlin/JVM 객체 및 확장 API 형태로 기계적으로 포팅하지
않고 바이트 지향 코덱 기본 기능으로 추가했습니다. Base58은 Bitcoin 알파벳을
사용하고, Base62는 bluetape 숫자 우선 알파벳을 사용합니다.

## 예상 밖의 발견

bluetape4k Kotlin 라인에는 Base62 개념이 두 가지 있습니다. 정수/UUID 지향
`Base62`와 KSUID 로컬 바이트 지향 `BytesBase62`입니다. Rust 이슈에서는
바이트 슬라이스를 입력하고 출력하도록 요구했으므로, 코덱 crate는 우선
바이트 지향 기본 기능을 노출하고 정수/UUID 렌더링은 이후 ID 전용 경계로
남겨 두어야 합니다.

일반적인 Base58 예시 `2NEpo7TZRRrLZSi2U`는 `Hello, World!`가 아니라
`Hello World!`에 해당합니다. 예시를 문서화할 때 알려진 벡터의 텍스트를
정확하게 유지해야 합니다.

## 다음에 적용할 규칙

- 모든 base 계열 코덱의 공개 문서에 알파벳과 선행 0 처리 정책을 명시합니다.
- ID 전용 crate 또는 모듈이 해당 API 형태를 소유하기 전까지 UUID 및 정수 렌더링을 `bluetape-rs-codec`에서 제외합니다.
- 여러 코덱에서 공유 알파벳 또는 변환 helper를 재사용하는 무상태 코덱 경로에는 thread stress test를 추가합니다.
