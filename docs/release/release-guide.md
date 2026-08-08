# 릴리스 가이드

이 repository는 `bluetape-go`와 동일한 branch role split을 사용합니다.

- `develop`은 default branch이자 active development center입니다.
- `main`은 최신 안정 릴리스 소스 브랜치입니다.
- stable release는 검증한 `develop` tree를 `main`으로 promote한 뒤
  `main`에서 signed version tag와 GitHub Release를 생성합니다.

## 브랜치 계약

| 브랜치 | 역할 | 허용된 변경 |
|---|---|---|
| `develop` | Integration 및 development branch | 일반 feature, fix, docs, CI, release-prep PR. |
| `main` | Stable release branch | Release promotion commit만 허용하며 latest published stable tag와 일치해야 합니다. |

`main`을 일반 개발에 사용하지 않습니다. 일반 작업은 `develop`을
대상으로 엽니다.

## 안정 릴리스 절차

1. `develop`에서 target GitHub milestone을 완료합니다.
2. release 범위에 의도하지 않은 open PR이 없는지 확인합니다.
3. `develop`을 target으로 하는 release-prep branch에서 `CHANGELOG.md`,
   package version, README 파일, release evidence를 업데이트합니다.
4. 전체 local quality bar를 실행합니다.
   - `cargo fmt --all --check`
   - `git diff --check`
   - `cargo test --workspace --all-features --locked`
   - `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
   - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
   - crate를 publish하는 릴리스라면 `cargo publish --workspace --dry-run --locked`
5. CI와 review를 통과한 뒤 release-prep PR을 `develop`에 merge합니다.
6. release PR 또는 명시적으로 review한 branch update를 사용해 검증한
   `develop` tree를 `main`으로 promote합니다.
7. `main`에서 signed annotated tag를 생성합니다.
8. 해당 tag에서 GitHub Release를 생성합니다.
9. local `develop`과 `main`을 sync하고 stale worktree/branch를 prune한
   뒤 최종 release evidence를 기록합니다.

## 현재 안정 기준점

`main`은 현재 `v0.3.1` release commit을 가리킵니다.

- `v0.3.1^{}`: `6cef44048b97d2d933fcf42e34fd6f3267b4ff30`
- `origin/main`: `6cef44048b97d2d933fcf42e34fd6f3267b4ff30`

GitHub default branch는 계속 `develop`입니다.

## 현재 개발 기준점

`0.4.0` compression release-readiness pass 기준으로 `develop`은 stable
`main` branch보다 앞서 있습니다.

- `origin/develop`: `71b3a564025b66d3228df5950e23c08019a6f543`
- `origin/main`: `6cef44048b97d2d933fcf42e34fd6f3267b4ff30`

release promotion 전에 이 reference를 다시 확인합니다. stable release tag는
검증한 `develop` tree를 promote한 뒤에도 `main`에만 둡니다.

## 가드레일

- feature work를 `main`에 직접 push하지 않습니다.
- stable release에서는 `main` 이외의 branch에서 tag하지 않습니다.
- `main`, version tag, GitHub Release, local sync evidence를 모두 확인하기
  전에는 milestone을 release-complete로 닫지 않습니다.
- branch protection 또는 repository ruleset 변경은 operational change이므로
  별도의 review task로 처리합니다.
