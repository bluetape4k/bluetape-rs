# 릴리스 브랜치 정책 검토

날짜: 2026-06-10
이슈: #60
브랜치: `docs/issue-60-release-branch-policy`

## 범위

Repository branch policy를 문서화하고 검증합니다.

- `develop`은 기본 브랜치이자 active development center로 유지합니다.
- `main`은 존재하며 최신 stable release source를 나타냅니다.
- Stable release 흐름은 tag를 만들기 전에 검증된 `develop`을 `main`으로 promote합니다.
- README와 README.ko가 release guide를 연결합니다.

## 근거

- `gh repo view --json defaultBranchRef`: `develop`
- `git rev-parse 'v0.2.0^{}' origin/main origin/develop`: 세 ref 모두 `fae6977bc9a01d8c665a9959bd7808139791dd46`
- `gh api repos/bluetape4k/bluetape-rs/branches/main`: branch가 `fae6977bc9a01d8c665a9959bd7808139791dd46`에 존재
- `.github/workflows/ci.yml`: CI가 이미 `develop`과 `main`에서 trigger됨

## 검토

| 검사 | 판정 | 근거 |
|---|---|---|
| Branch contract | PASS | `docs/release/release-guide.md`가 `develop`을 development/default branch, `main`을 stable release source로 명시합니다. |
| Release 흐름 | PASS | Guide가 milestone 종료, `develop` release-prep, `main` promote, signed tag 및 GitHub Release 순서를 정의합니다. |
| 현재 기준점 | PASS | Guide가 일치하는 SHA와 함께 `v0.2.0` release commit에서 `main`을 생성했다고 기록합니다. |
| README parity | PASS | `README.md`와 `README.ko.md`가 모두 `docs/release/release-guide.md`를 연결합니다. |
| 범위 통제 | PASS | Branch protection/ruleset과 향후 stable release 작업은 별도 검토 task로 명시적으로 보류했습니다. |

## P0/P1 Gate

P0=0 P1=0

이 문서 변경에는 P2/P3 후속 작업이 필요하지 않습니다.

## 검증

- `git diff --check`: PASS
- `rg -n "develop|main|Release guide|release-guide|v0\\.2\\.0|fae6977" README.md README.ko.md WIP.md docs/release/release-guide.md`: PASS
