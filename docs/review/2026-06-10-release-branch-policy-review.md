# 릴리스 브랜치 정책 검토

날짜: 2026-06-10
이슈: #60
브랜치: `docs/issue-60-release-branch-policy`

## 범위

저장소 브랜치 정책을 문서화하고 검증한다.

- `develop`는 기본 브랜치이자 활성 개발 중심으로 유지된다.
- `main`은 존재하며 최신 안정 릴리스 소스를 나타낸다.
- 안정 릴리스 흐름은 tag 전에 검증된 `develop`을 `main`으로 승격한다.
- README 및 README.ko가 릴리스 가이드를 링크한다.

## 근거

- `gh repo view --json defaultBranchRef`: `develop`
- `git rev-parse 'v0.2.0^{}' origin/main origin/develop`:
  `fae6977bc9a01d8c665a9959bd7808139791dd46` for all three refs
- `gh api repos/bluetape4k/bluetape-rs/branches/main`: branch exists at
  `fae6977bc9a01d8c665a9959bd7808139791dd46`
- `.github/workflows/ci.yml`: CI already triggers on `develop` and `main`

## 검토

| 확인 항목 | 판정 | 근거 |
|---|---|---|
| 브랜치 계약 | PASS | `docs/release/release-guide.md`가 `develop`을 개발/기본 브랜치로, `main`을 안정 릴리스 소스로 명시한다. |
| 릴리스 흐름 | PASS | 가이드가 마일스톤 종료, `develop`에서 릴리스 준비, `main` 승격, 서명 태그와 GitHub Release 순서를 정의한다. |
| 현재 기준선 | PASS | 가이드가 일치하는 SHA와 함께 `v0.2.0` 릴리스 커밋에서 `main`을 만든 사실을 기록한다. |
| README 동등성 | PASS | `README.md`와 `README.ko.md` 모두 `docs/release/release-guide.md`를 링크한다. |
| 범위 통제 | PASS | 브랜치 보호/ruleset과 향후 안정 릴리스 작업은 별도 검토 작업으로 명시적으로 미뤘다. |

## P0/P1 게이트

P0=0 P1=0

이 문서 변경에는 P2/P3 후속 작업이 필요하지 않다.

## 검증

- `git diff --check`: PASS
- `rg -n "develop|main|Release guide|release-guide|v0\\.2\\.0|fae6977" README.md README.ko.md WIP.md docs/release/release-guide.md`: PASS
