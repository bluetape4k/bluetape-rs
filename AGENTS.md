# AGENTS.md - bluetape-rs

This repository inherits the workspace guidance from `../AGENTS.md`.
Read and follow the workspace root guide first. This file only adds
repo-specific layout, commands, domain rules, and local exceptions.


This repository is the Rust backend library line for the bluetape ecosystem.
It is governed by the workspace-level `AGENTS.md`; this file narrows the rules
for Rust work in this repo.

## Mandatory Workflow

- Every task in this repository must start by loading and applying
  `bluetape4k-workflow`.
- Classify the work type through `bluetape4k-workflow` before editing files,
  committing, creating PRs, changing GitHub issues, or running release/project
  management steps.
- For Rust implementation, Rust review, Rust PR review, or Rust release
  preflight, also load and apply `bluetape-rs-patterns`.
- Code-changing work must use an issue-scoped git worktree under `.worktrees/`
  unless the user explicitly selects the current checkout.
- Code-changing Type A/B/C work and every PR that contains code must complete
  the required `bluetape4k-workflow` review gates before being reported as
  ready:
  - Step 6-R local/native 7-Tier review on the implemented diff.
  - Step 7-R post-PR review before any CI/merge-ready claim.
  - Native subagent review lanes when the session supports them, with
    `code-reviewer` and `verifier` as the minimum code-review baseline and
    additional reviewers selected by risk signals.
  - Explicit `P0=0 P1=0` evidence before advancing to the next gate.
- PR bodies must end with the workflow-required `## DoD Status` section, and
  review evidence must be recorded as required by `bluetape4k-workflow`.
- If any required workflow gate was skipped, stop downstream work, mark the
  gate as failed, run the missing gate, repair all P0/P1 findings, and only then
  continue.

## Project Position

- Do not port Kotlin/JVM `bluetape4k` APIs mechanically.
- Do not rewrite `bluetape-go` package shapes mechanically.
- Prefer Rust-native contracts: `Result`, `Option`, lifetimes, `Send`/`Sync`,
  traits, feature flags, typed builders, and compile-fail tests where useful.
- Favor small crates with clear backend service value over broad utility bags.

## Rust Ecosystem Conventions

- Assume contributors may be new to Rust; preserve Rust ecosystem standards and
  conventions instead of optimizing for familiarity with Kotlin/JVM or Go.
- Prefer idiomatic Rust API shapes over mechanical ports: ownership-aware
  types, explicit lifetimes only when needed, `Result`/`Option` contracts,
  traits for behavior, typed configuration structs, and additive feature flags.
- Public APIs should have Rustdoc that teaches the intended usage: concise
  explanation, compile-friendly `# Examples`, `# Errors` for `Result` APIs,
  `# Panics` only when the public contract can panic, and `# Safety` only for
  unsafe functions or unsafe trait contracts.
- Prefer explicit error enums and standard ecosystem crates only when they fit
  the local crate boundary; do not add dependencies just to mimic another
  bluetape language line.
- For async infrastructure work, follow `tokio` and Rust structured-concurrency
  conventions unless a concrete compatibility issue requires another runtime.
- Treat `cargo fmt`, `cargo test`, `cargo clippy -D warnings`, and `cargo doc`
  as the ordinary quality bar for public API or documentation changes.

## Rust Standards

- Use Rust 2024 edition unless a concrete compatibility issue requires a
  documented exception.
- Keep public APIs documented with Rustdoc.
- Prefer `thiserror`-style explicit error enums once dependencies are added.
- Use `tokio` as the default async runtime for infrastructure-facing packages.
- Avoid `unsafe`; if unavoidable, isolate it, document invariants, and add tests.
- Keep feature flags additive and avoid enabling heavy integrations by default.
- Prefer builders or typed configuration structs over long positional argument
  lists.

## SQL Direction

- The SQL track should start as an inspectable SQL AST plus dialect renderer and
  SQLx execution adapter.
- Do not claim full ORM support until lifecycle, relation loading, migrations,
  and transaction semantics are explicitly designed and tested.
- Keep raw SQL escape hatches explicit and clearly named.
- Preserve SQL strings and bind values separately.
- Use container-backed PostgreSQL tests before claiming database support.

## Testing And Verification

- Run `cargo fmt --all` after Rust source changes.
- Run `cargo test --workspace` for ordinary changes.
- Add `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  once the dependency/tooling baseline is established.
- Use Testcontainers-backed tests for Redis, PostgreSQL, Kafka, AWS emulator, or
  graph database support.
- For public API or behavior changes, update both README files and WIP/roadmap
  notes when needed.
