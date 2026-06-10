# Workspace Release Version Alignment Lessons

## Lesson

A milestone release is not the same as a single crate release. For bluetape-rs,
stable milestone publishing must verify the root crate and every current
workspace member before tag, GitHub Release, and crates.io publication are
called complete.

## Trigger

`v0.3.0` was released as the `0.3.0` milestone, but only
`bluetape-rs-codec@0.3.0` was published. Other workspace crates still had
`0.1.1` or `0.2.0` manifest versions and were absent from crates.io.

## Rule

- Before publishing a milestone release, run `cargo metadata` and confirm every
  publishable workspace package has the intended release version.
- Run registry checks from outside the repository so `cargo info` cannot report
  local path package metadata as if it were crates.io state.
- Treat `cargo publish --workspace --dry-run --locked` as a preflight, but
  publish the actual crates in dependency order and verify each one from `/tmp`.
- Do not start the next milestone until the current milestone's root crate,
  member crates, GitHub Release, tag, and registry visibility all match.
