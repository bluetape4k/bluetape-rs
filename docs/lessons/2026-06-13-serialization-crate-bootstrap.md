# Lesson: Serialization Crate Bootstrap

Date: 2026-06-13
Scope: issue #108, `bluetape-rs-serialization` bootstrap

## What Changed

- Added the `crates/serialization` workspace crate as package
  `bluetape-rs-serialization` and library `bluetape_rs_serialization`.
- Added the opt-in root `serialization` facade feature without changing root
  defaults.
- Kept issue #108 to crate/facade/docs bootstrap only; serializer traits,
  adapters, and runtime binary encoding remain later `0.5.0` work.

## Lessons

- New crate `lib.rs` files must follow sibling crate style first. Long roadmap,
  issue history, usage guide, and non-goal prose belongs in README/spec/plan
  artifacts, not in the crate root.
- Public README snippets for unreleased crates must not point callers at an
  already-published version that lacks the new crate or feature. Use git/path or
  explicit post-release examples until release-prep updates versions.
- WIP/roadmap traceability matters even for bootstrap issues. Name the package,
  library, root feature, and non-goals where the milestone scope is described.
- Step 6-R evidence files are part of the gate, not after-the-fact polish. Add
  the implementation review artifact before claiming the gate is closed.

## Checks That Caught The Gaps

- Step 6-R user/caller review caught the unpublished `0.4.0` Cargo snippets.
- Step 6-R operator review caught missing tracked review and lesson artifacts.
- Step 6-R stability review caught weak `WIP.md` crate/facade traceability.

## Forward Rule

For every future `bluetape-rs` crate bootstrap, compare at least two sibling
crate roots before writing `lib.rs`, keep public Cargo snippets release-aware,
and create the Step 6-R review artifact before PR creation.
