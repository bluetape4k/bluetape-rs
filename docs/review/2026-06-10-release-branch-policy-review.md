# Release Branch Policy Review

Date: 2026-06-10
Issue: #60
Branch: `docs/issue-60-release-branch-policy`

## Scope

Document and verify the repository branch policy:

- `develop` remains the default branch and active development center.
- `main` exists and represents the latest stable release source.
- Stable release flow promotes verified `develop` to `main` before tagging.
- README and README.ko link to the release guide.

## Evidence

- `gh repo view --json defaultBranchRef`: `develop`
- `git rev-parse 'v0.2.0^{}' origin/main origin/develop`:
  `fae6977bc9a01d8c665a9959bd7808139791dd46` for all three refs
- `gh api repos/bluetape4k/bluetape-rs/branches/main`: branch exists at
  `fae6977bc9a01d8c665a9959bd7808139791dd46`
- `.github/workflows/ci.yml`: CI already triggers on `develop` and `main`

## Review

| Check | Verdict | Evidence |
|---|---|---|
| Branch contract | PASS | `docs/release/release-guide.md` names `develop` as development/default branch and `main` as stable release source. |
| Release flow | PASS | Guide defines milestone closure, release-prep on `develop`, promotion to `main`, then signed tag and GitHub Release. |
| Current baseline | PASS | Guide records `main` creation from `v0.2.0` release commit with matching SHAs. |
| README parity | PASS | `README.md` and `README.ko.md` both link to `docs/release/release-guide.md`. |
| Scope control | PASS | Branch protection/rulesets and future stable release actions are explicitly deferred to separate reviewed tasks. |

## P0/P1 Gate

P0=0 P1=0

No P2/P3 follow-up is required for this documentation change.

## Validation

- `git diff --check`: PASS
- `rg -n "develop|main|Release guide|release-guide|v0\\.2\\.0|fae6977" README.md README.ko.md WIP.md docs/release/release-guide.md`: PASS
