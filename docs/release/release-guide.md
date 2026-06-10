# Release Guide

This repository uses the same branch role split as `bluetape-go`:

- `develop` is the default branch and the center of active development.
- `main` is the latest stable release source branch.
- Stable releases promote the verified `develop` tree to `main`, then create
  the signed version tag and GitHub Release from `main`.

## Branch Contract

| Branch | Role | Allowed changes |
|---|---|---|
| `develop` | Integration and development branch | Normal feature, fix, docs, CI, and release-prep PRs. |
| `main` | Stable release branch | Release promotion commits only; it should match the latest published stable tag. |

Do not use `main` for ordinary development. Open normal work against `develop`.

## Stable Release Flow

1. Finish the target GitHub milestone on `develop`.
2. Confirm there are no unintended open PRs for the release scope.
3. Update `CHANGELOG.md`, package versions, README files, and release evidence
   on a release-prep branch targeting `develop`.
4. Run the full local quality bar:
   - `cargo fmt --all --check`
   - `git diff --check`
   - `cargo test --workspace --all-features --locked`
   - `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
   - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`
   - `cargo publish --workspace --dry-run --locked` when the release publishes crates
5. Merge the release-prep PR into `develop` after CI and review pass.
6. Promote the verified `develop` tree to `main` using a release PR or an
   explicitly reviewed branch update.
7. Create the signed annotated tag on `main`.
8. Create the GitHub Release from that tag.
9. Sync local `develop` and `main`, prune stale worktrees/branches, and record
   final release evidence.

## Current Stable Baseline

`main` was created from the `v0.2.0` release commit:

- `v0.2.0^{}`: `fae6977bc9a01d8c665a9959bd7808139791dd46`
- `origin/main`: `fae6977bc9a01d8c665a9959bd7808139791dd46`

GitHub's default branch remains `develop`.

## Current Development Baseline

As of the `0.3.0` codec release-readiness pass, `develop` has advanced beyond
the stable `main` branch:

- `origin/develop`: `639b030559b20960e0f3d79a8e3b1cf5060994dd`
- `origin/main`: `fae6977bc9a01d8c665a9959bd7808139791dd46`

Recheck these references before release promotion. Stable release tags still
belong on `main` after the verified `develop` tree is promoted.

## Guardrails

- Never push feature work directly to `main`.
- Do not tag from a branch other than `main` for stable releases.
- Do not close a milestone as release-complete until `main`, the version tag,
  GitHub Release, and local sync evidence have all been checked.
- Branch protection or repository ruleset changes are operational changes and
  should be handled in a separate reviewed task.
