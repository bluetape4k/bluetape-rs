# 릴리스 가이드

이 저장소는 `bluetape-go`와 같은 브랜치 역할 분리를 사용한다.

- `develop`은 기본 브랜치이자 활발한 개발의 중심이다.
- `main`은 최신 안정 릴리스의 소스 브랜치다.
- 안정 릴리스는 검증된 `develop` 트리를 `main`으로 승격한 뒤 `main`에서
  서명된 버전 태그와 GitHub Release를 생성한다.

## 브랜치 계약

| 브랜치 | 역할 | 허용된 변경 |
|---|---|---|
| `develop` | 통합 및 개발 브랜치 | 일반 feature, fix, docs, CI 및 릴리스 준비 PR. |
| `main` | 안정 릴리스 브랜치 | 릴리스 승격 커밋만 허용하며 최신 게시 안정 태그와 일치해야 한다. |

일반 개발에 `main`을 사용하지 않는다. 일반 작업은 `develop`을 대상으로 연다.

## 안정 릴리스 흐름

1. `develop`에서 대상 GitHub 마일스톤을 완료한다.
2. 릴리스 범위에 의도하지 않은 열린 PR이 없는지 확인한다.
3. `develop`을 대상으로 하는 릴리스 준비 브랜치에서 `CHANGELOG.md`, 패키지
   버전, README 파일 및 릴리스 근거를 갱신한다.
4. 전체 로컬 품질 기준을 실행한다.
   - `cargo fmt --all --check`
   - `git diff --check`
   - `cargo test --workspace --all-features --locked`
   - `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
   - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
   - 크레이트를 게시하는 릴리스에서는 `cargo publish --workspace --dry-run --locked`
5. CI와 검토가 통과한 뒤 릴리스 준비 PR을 `develop`에 병합한다.
6. 릴리스 PR 또는 명시적으로 검토한 브랜치 갱신을 통해 검증된 `develop`
   트리를 `main`으로 승격한다.
7. `main`에 서명된 주석 태그를 만든다.
8. 해당 태그에서 GitHub Release를 생성한다.
9. 로컬 `develop`과 `main`을 동기화하고 오래된 worktree/브랜치를 정리한 뒤
   최종 릴리스 근거를 기록한다.

## 현재 안정 기준선

현재 `main`은 `v0.3.1` 릴리스 커밋을 가리킨다.

- `v0.3.1^{}`: `6cef44048b97d2d933fcf42e34fd6f3267b4ff30`
- `origin/main`: `6cef44048b97d2d933fcf42e34fd6f3267b4ff30`

GitHub의 기본 브랜치는 계속 `develop`이다.

## 현재 개발 기준선

`0.4.0` 압축 릴리스 준비 상태 검토 시점에 `develop`은 안정 브랜치인
`main`보다 앞서 있다.

- `origin/develop`: `71b3a564025b66d3228df5950e23c08019a6f543`
- `origin/main`: `6cef44048b97d2d933fcf42e34fd6f3267b4ff30`

릴리스를 승격하기 전에 이 참조를 다시 확인한다. 안정 릴리스 태그는 검증된
`develop` 트리를 승격한 뒤에도 `main`에만 둔다.

## 보호 규칙

- feature 작업을 `main`에 직접 push하지 않는다.
- 안정 릴리스에서 `main`이 아닌 브랜치로 태그를 만들지 않는다.
- `main`, 버전 태그, GitHub Release 및 로컬 동기화 근거를 모두 확인하기 전에는
  마일스톤을 릴리스 완료로 닫지 않는다.
- 브랜치 보호 또는 저장소 ruleset 변경은 운영 변경이므로 별도의 검토 작업으로
  처리한다.
