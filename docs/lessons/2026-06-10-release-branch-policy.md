# 릴리스 브랜치 정책에서 얻은 교훈

날짜: 2026-06-10
이슈: #60

## 발생한 일

`develop`은 이미 GitHub 기본 브랜치였지만 `main`은 존재하지 않았습니다.
bluetape-go의 릴리스 형태에 맞추기 위해 `main`을 최신 stable release
commit, 즉 peel된 `v0.2.0` tag commit에서 생성했습니다.

## 예상 밖의 발견

`git rev-parse v0.2.0`은 릴리스 commit SHA가 아니라 annotated tag object
SHA를 반환합니다. 브랜치 정렬 근거에는
`git rev-parse 'v0.2.0^{}'`를 사용해야 합니다.

## 다음에 적용할 규칙

- annotated release tag를 확인할 때 tag object와 peel된 commit을 모두 검증합니다.
- `develop`을 기본 브랜치로 유지하고 `main`은 stable release source에만 사용합니다.
- `main`, tag, GitHub Release 및 local sync 근거를 모두 기록하기 전에는 milestone이 릴리스 완료라고 부르지 않습니다.
