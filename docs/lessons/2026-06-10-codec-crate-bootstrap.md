# 코덱 crate 초기 구성에서 얻은 교훈

날짜: 2026-06-10
이슈: #53

## 발생한 일

`0.3.0` 코덱 라인은 첫 encoder API보다 먼저 crate 경계가 필요했습니다.
안전한 범위는 workspace 등록, 추가적인 root facade feature, crate
README/Rustdoc, smoke test뿐이었습니다.

## 예상 밖의 발견

`Cargo.lock`을 갱신하기 전에는 새 workspace package를 기록해야 하므로
`cargo check --locked`가 실패했습니다. 먼저 offline 모드에서 동일한 검사를
`--locked` 없이 실행한 다음 locked 검사를 다시 실행하는 방식으로
복구했습니다.

또한 `-D warnings`에서 주석만 있는 Rust code block을 Rustdoc이 거부했습니다.
crate 초기 구성 문서에서는 주석만 있는 Rust block에 의존하지 말고,
컴파일되지 않는 예시는 `text`로 표시해야 합니다.

## 다음에 적용할 규칙

- 새 Rust workspace crate에서는 locked validation이 통과하기 전에 lockfile을 한 번 갱신해야 한다고 예상합니다.
- 초기 구성 이슈에서는 동작 구현을 제외해 후속 이슈가 API와 error contract를 독립적으로 검토할 수 있게 합니다.
- placeholder manifest snippet은 `toml` 또는 `text`로 표시하고, 비어 있거나 주석만 있는 Rust code block은 crate Rustdoc에서 사용하지 않습니다.
