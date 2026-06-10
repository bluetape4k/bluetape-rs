# Release Branch Policy Lessons

Date: 2026-06-10
Issue: #60

## What Happened

`develop` was already the GitHub default branch, but `main` did not exist. To
match the bluetape-go release shape, `main` was created at the latest stable
release commit, which is the peeled `v0.2.0` tag commit.

## What Surprised Us

`git rev-parse v0.2.0` returns the annotated tag object SHA, not the release
commit SHA. For branch alignment evidence, use `git rev-parse 'v0.2.0^{}'`.

## Next Time

- Verify both the tag object and peeled commit when checking annotated release
  tags.
- Keep `develop` as default and use `main` only for stable release source.
- Do not call a milestone release-complete until `main`, tag, GitHub Release,
  and local sync evidence are all recorded.
