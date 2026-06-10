# Codec Crate Bootstrap Lessons

Date: 2026-06-10
Issue: #53

## What Happened

The `0.3.0` codec line needed a crate boundary before the first encoder APIs.
The safe slice was workspace registration, an additive root facade feature,
crate README/Rustdoc, and a smoke test only.

## What Surprised Us

`cargo check --locked` failed before `Cargo.lock` was updated because the new
workspace package had to be recorded. The repair was to run the same check
without `--locked` in offline mode first, then rerun the locked validation.

Rustdoc also rejected a comment-only Rust code block under `-D warnings`; crate
bootstrap docs should mark non-compiling examples as `text` instead of relying
on comment-only Rust blocks.

## Next Time

- For new Rust workspace crates, expect one lockfile refresh before locked
  validation can pass.
- Keep bootstrap issues free of behavior implementation so follow-up issues can
  review API and error contracts independently.
- Mark placeholder manifest snippets as `toml` or `text`; do not use empty or
  comment-only Rust code blocks in crate Rustdoc.
