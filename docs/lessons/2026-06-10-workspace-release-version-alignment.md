# Workspace 릴리스 버전 정렬에서 얻은 교훈

## 교훈

Milestone 릴리스는 단일 crate 릴리스와 같지 않습니다. bluetape-rs의
stable milestone publish가 완료되었다고 판단하기 전에는 root crate와 현재
workspace member를 모두 검증해야 하며, tag, GitHub Release, crates.io
publish도 확인해야 합니다.

## 계기

`v0.3.0`은 `0.3.0` milestone으로 릴리스되었지만
`bluetape-rs-codec@0.3.0`만 publish되었습니다. 다른 workspace crate의
manifest version은 여전히 `0.1.1` 또는 `0.2.0`이었고 crates.io에도
없었습니다.

## 규칙

- Milestone 릴리스를 publish하기 전에 `cargo metadata`를 실행하고, publish 가능한 모든 workspace package가 의도한 릴리스 버전을 갖는지 확인합니다.
- `cargo info`가 local path package metadata를 crates.io 상태처럼 보고하지 않도록 repository 외부에서 registry 검사를 실행합니다.
- `cargo publish --workspace --dry-run --locked`는 preflight로 취급하되 실제 crate는 dependency 순서로 publish하고 `/tmp`에서 각각 확인합니다.
- 현재 milestone의 root crate, member crate, GitHub Release, tag, registry visibility가 모두 일치하기 전에는 다음 milestone을 시작하지 않습니다.
