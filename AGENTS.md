# AGENTS.md - bluetape-rs

This repository is the Rust backend library line for the bluetape ecosystem.
It is governed by the workspace-level `AGENTS.md`; this file narrows the rules
for Rust work in this repo.

## Language Policy

- User-facing chat remains Korean.
- Public contributor artifacts are English: README.md, KDoc-equivalent Rustdoc,
  CHANGELOG, release notes, GitHub issues, PR bodies, and commit messages.
- Keep `README.md` and `README.ko.md` synchronized whenever public behavior,
  package scope, or roadmap changes.
- Agent-facing guidance such as this file stays concise English.

## Project Position

- Do not port Kotlin/JVM `bluetape4k` APIs mechanically.
- Do not rewrite `bluetape-go` package shapes mechanically.
- Prefer Rust-native contracts: `Result`, `Option`, lifetimes, `Send`/`Sync`,
  traits, feature flags, typed builders, and compile-fail tests where useful.
- Favor small crates with clear backend service value over broad utility bags.

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

## Git Workflow

- `develop` is the integration branch.
- Do not push directly to `main`.
- Use English Lore-protocol commit messages as required by the workspace
  guidance.
