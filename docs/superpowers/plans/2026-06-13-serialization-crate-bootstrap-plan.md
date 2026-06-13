# Serialization Crate Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the focused `bluetape-rs-serialization` workspace crate and optional root facade feature for issue #108 without expanding beyond the reviewed bootstrap slice.

**Architecture:** The new crate owns serialization vocabulary and documentation boundaries, while the root `bluetape-rs` crate only exposes it through an opt-in facade feature. Issue #108 intentionally stops before first binary adapter implementation; later `0.5.0` issues add contracts/adapters/tests after this crate boundary is reviewable.

**Tech Stack:** Rust 2024, Cargo workspace resolver 3, additive Cargo features, Rustdoc, README/README.ko parity.

---

## File Structure

- Create `crates/serialization/Cargo.toml`: package metadata, crate name, default-empty feature set.
- Create `crates/serialization/src/lib.rs`: concise crate-level Rustdoc that
  follows sibling crate style; keep long boundary, migration, and non-goal
  prose in README/spec/plan docs.
- Create `crates/serialization/README.md`: English user-facing crate boundary, usage paths, mismatch semantics, migration note, and non-goals.
- Create `crates/serialization/README.ko.md`: Korean localized crate boundary with the same sections and non-goal set.
- Modify `Cargo.toml`: insert `crates/serialization` while preserving every existing workspace member, add workspace dependency, optional root dependency, and `serialization` feature.
- Modify `Cargo.lock`: refresh after the local workspace package is added, then use `--locked` for subsequent verification.
- Modify `src/lib.rs`: root facade re-export behind `#[cfg(feature = "serialization")]`.
- Modify `README.md`: rename package table entry and add direct-crate vs root-facade guidance.
- Modify `README.ko.md`: keep localized package table and guidance in sync.
- Review `WIP.md`: confirm issue #108 does not require additional WIP edits after the merged `0.5.x` split.

## Task 1: Serialization Crate Skeleton

**Complexity:** low
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Create: `crates/serialization/Cargo.toml`
- Create: `crates/serialization/src/lib.rs`

- [x] **Step 1: Create package metadata**

Create `crates/serialization/Cargo.toml`:

```toml
[package]
name = "bluetape-rs-serialization"
version = "0.4.0"
edition.workspace = true
rust-version.workspace = true
description = "Rust-native serialization contracts for bluetape-rs."
license.workspace = true
repository.workspace = true
readme = "README.md"
keywords = ["serialization", "serde", "binary", "cache"]
categories = ["encoding", "development-tools"]

[lib]
name = "bluetape_rs_serialization"
path = "src/lib.rs"

[features]
default = []

[dependencies]
```

Version policy: issue #108 lands on the current development version line. The
crate package stays at the current workspace version until release-preparation
updates crate versions for `0.5.0`, but public README examples must not point
callers at unpublished `0.4.0` registry artifacts.

- [x] **Step 2: Create concise crate Rustdoc**

Create `crates/serialization/src/lib.rs`:

```rust
//! Rust-native serialization boundary for bluetape-rs.
//!
//! This bootstrap crate reserves the focused `0.5.0` serialization package and
//! keeps the root facade opt-in. Serializer traits, payload envelopes, and
//! adapters are added in later reviewed issues.
//!
//! ```text
//! // Enable the root facade when a single dependency is more convenient:
//! // bluetape-rs = { version = "...", features = ["serialization"] }
//! ```

#[cfg(test)]
mod tests {
    #[test]
    fn crate_metadata_matches_serialization_boundary() {
        assert_eq!(env!("CARGO_PKG_NAME"), "bluetape-rs-serialization");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.4.0");
    }
}
```

This shape follows existing sibling crate roots such as `crates/core`,
`crates/compression`, and `crates/logging`: short crate-level Rustdoc in
`lib.rs`, focused exports/tests in the crate root, and detailed usage/non-goal
material in README/spec/plan docs.

## Task 2: Workspace And Facade Wiring

**Complexity:** medium
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `src/lib.rs`

- [x] **Step 1: Insert the workspace member without replacing the array**

In `Cargo.toml`, insert only this line after the existing `crates/logging`
member and preserve every existing workspace member, including benchmark and
test crates:

```toml
    "crates/serialization",
```

- [x] **Step 2: Register the workspace dependency**

In `[workspace.dependencies]`, add:

```toml
bluetape-rs-serialization = { path = "crates/serialization", version = "0.4.0" }
```

- [x] **Step 3: Add the opt-in root feature**

In `[features]`, keep `default = ["core"]` unchanged and add:

```toml
serialization = ["dep:bluetape-rs-serialization"]
```

- [x] **Step 4: Add the optional root dependency**

In `[dependencies]`, add:

```toml
bluetape-rs-serialization = { workspace = true, optional = true }
```

- [x] **Step 5: Add the gated root facade**

In `src/lib.rs`, add:

```rust
#[cfg(feature = "serialization")]
pub use bluetape_rs_serialization as serialization;
```

- [x] **Step 6: Refresh the lock file once before locked verification**

Run:

```bash
cargo check -p bluetape-rs-serialization --all-features
```

Expected:

- `Cargo.lock` is refreshed if Cargo needs to record the new workspace package.
- Subsequent verification commands use `--locked`.

- [x] **Step 7: Verify feature wiring locally**

Run:

```bash
cargo metadata --no-deps --format-version 1 --locked
cargo check -p bluetape-rs --locked
cargo check -p bluetape-rs --no-default-features --locked
cargo check -p bluetape-rs --features serialization --locked
```

Expected:

- Metadata includes every previous workspace member plus `crates/serialization`.
- Metadata includes package `bluetape-rs-serialization` with readme, license,
  repository, and version aligned to the current development version policy.
- Plain default `cargo check -p bluetape-rs --locked` still uses the existing
  default feature set and does not require the serialization facade.
- `--no-default-features` does not require the facade.
- `--features serialization` resolves the optional facade dependency.

## Task 3: User-Facing Documentation Parity

**Complexity:** low
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Create: `crates/serialization/README.md`
- Create: `crates/serialization/README.ko.md`
- Modify: `README.md`
- Modify: `README.ko.md`
- Review: `WIP.md`

- [x] **Step 1: Write English crate README**

Create `crates/serialization/README.md` with these sections:

````markdown
# bluetape-rs-serialization

Rust-native serialization boundary for `bluetape-rs`.

This crate is the bootstrap slice for the `0.5.0` cache-first binary
serialization milestone. It creates the package and documentation boundary first
so later issues can add traits, typed errors, binary payload envelopes, and
adapters behind reviewed feature flags.

## Usage

Direct crate dependency:

```toml
# Until 0.5.0 is published:
bluetape-rs-serialization = { git = "https://github.com/bluetape4k/bluetape-rs", package = "bluetape-rs-serialization" }

# After 0.5.0 is published:
bluetape-rs-serialization = "0.5"
```

```rust
use bluetape_rs_serialization as serialization;
```

Root facade usage:

```toml
# Until 0.5.0 is published:
bluetape-rs = { git = "https://github.com/bluetape4k/bluetape-rs", features = ["serialization"] }

# After 0.5.0 is published:
bluetape-rs = { version = "0.5", features = ["serialization"] }
```

```rust
use bluetape_rs::serialization;
```

The root facade is unavailable unless the `serialization` feature is enabled.
Default `bluetape-rs` builds remain unchanged.

The bootstrap crate exposes no serializer traits or adapters yet. Those arrive
in later reviewed `0.5.0` issues.

## Boundary

`0.5.0` starts with cache-first binary payload support. Payload metadata,
versioning, trust profiles, typed failures, and adapter contracts are added in
follow-up issues.

`Option<T>` represents absent values. Empty bytes are payload data and must not
be treated as a hidden null convention.

Future unsupported-version, wrong-format, and wrong-trust-profile cases are
typed decode failures. They must not decode as `None`, silently fall back, or
try alternate adapters. Cache eviction, namespace migration, and rebuild policy
belong to callers.

## Migration / Compatibility

Existing `bluetape-rs` users do not need code or Cargo changes for issue #108.
Callers only opt in by depending on `bluetape-rs-serialization` directly or by
enabling `features = ["serialization"]` on `bluetape-rs`.

## Issue #108 Bootstrap Non-goals

- Serializer traits or concrete adapters
- Runtime binary payload encoding
- Global serializer registry

## Not In The `0.5.0` Core/Binary Milestone

- JSON adapter
- Protobuf adapter
- Avro adapter
- Apache Fory adapter
- Testcontainers integration
- SQL or SQLx integration
- Resilience, retry, circuit-breaker, or fallback policies
- Unsafe deserialization
- Hidden global serializers
- Hidden default serializers
- Env-selected adapters
- Dynamic type loading
- Schema registry support
````

- [x] **Step 2: Write Korean crate README with full parity**

Create `crates/serialization/README.ko.md` with the same section set as the
English README:

- `Usage`: direct dependency, `use bluetape_rs_serialization as serialization;`,
  root facade dependency, `use bluetape_rs::serialization;`, default facade
  unavailable note, and boundary-only bootstrap note.
- `Boundary`: `Option<T>`, empty bytes, unsupported-version, wrong-format,
  wrong-trust-profile typed failures, no `None` fallback, no alternate-adapter
  fallback, caller-owned cache eviction/namespace migration/rebuild policy.
- `Migration / Compatibility`: existing `bluetape-rs` users need no change;
  opt in through the direct crate or `features = ["serialization"]`.
- `Issue #108 Bootstrap Non-goals`: no traits/adapters, no runtime binary
  payload encoding, no global registry.
- `Not In The 0.5.0 Core/Binary Milestone`: JSON, Protobuf, Avro, Fory,
  Testcontainers, SQL/SQLx, resilience/fallback policies, unsafe
  deserialization, hidden global serializers, hidden default serializers,
  env-selected adapters, dynamic type loading, and schema registry support.

- [x] **Step 3: Update root README package table**

In `README.md`, replace the serialization package name and pin benchmark wording
to the later benchmark milestone:

```markdown
| Serialization | `bluetape-rs-serialization` | Reserves the cache-first binary payload SerDe boundary first, then adds JSON, Protobuf, Avro, Fory, and the cross-repo benchmark track in `0.5.5` after adapters exist. |
```

Add a short note near the package table:

```markdown
Use `bluetape-rs-serialization` directly for crate-level docs, or enable
`features = ["serialization"]` on `bluetape-rs` for the root facade.
Issue #108 is a crate/facade/docs bootstrap only; serializer traits, concrete
adapters, and runtime binary encoding arrive in later reviewed `0.5.0` issues.
```

- [x] **Step 4: Update Korean README package table**

In `README.ko.md`, replace the serialization package name and pin benchmark
wording to `0.5.5`:

```markdown
| Serialization | `bluetape-rs-serialization` | Cache-first binary payload SerDe를 먼저 제공하고, JSON, Protobuf, Avro, Fory를 순차 확장한 뒤 adapter가 준비되면 `0.5.5`에서 cross-repo benchmark track을 진행합니다. |
```

Add the equivalent Korean note near the package table.

```markdown
crate-level 문서는 `bluetape-rs-serialization`을 직접 사용하고, root facade가
필요하면 `bluetape-rs`에서 `features = ["serialization"]`을 활성화하세요.
Issue #108은 crate/facade/docs bootstrap만 수행합니다. Serializer trait,
concrete adapter, runtime binary encoding은 review된 후속 `0.5.0` issue에서
추가합니다.
```

- [x] **Step 5: Confirm WIP parity**

Run:

```bash
rg -n "bluetape-rs-serde|bluetape-rs-serialization|0\\.5\\.0|0\\.5\\.5" README.md README.ko.md WIP.md crates/serialization
```

Expected:

- No remaining `bluetape-rs-serde` references.
- `README.md`, `README.ko.md`, `WIP.md`, crate README, and Rustdoc use the same
  crate name, root feature name, and `0.5.0` non-goal list.
- Root README files either contain the same direct-crate/root-facade guidance or
  explicitly link to the crate README for the detailed `Option<T>`, empty bytes,
  version/format/trust-profile mismatch, and no-fallback semantics.
- `WIP.md` still documents the `0.5.x` split. If it lacks issue #108 bootstrap
  text, the PR body must state that no additional WIP change was required
  because the milestone split is already present.

## Task 4: Feature And Documentation Verification

**Complexity:** medium
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Verify: `Cargo.toml`
- Verify: `Cargo.lock`
- Verify: `src/lib.rs`
- Verify: `crates/serialization/**`
- Verify: `README.md`
- Verify: `README.ko.md`
- Verify: `WIP.md`

- [x] **Step 1: Verify default dependency exclusion**

Run:

```bash
cargo tree -e features -p bluetape-rs --locked
cargo tree -e features -p bluetape-rs --no-default-features --locked
cargo tree -e features -p bluetape-rs --features serialization --locked
```

Expected:

- Plain default tree does not include `bluetape-rs-serialization`.
- `--no-default-features` does not include `bluetape-rs-serialization`.
- `--features serialization` includes `bluetape-rs-serialization`.
- No tree includes JSON, Protobuf, Avro, Fory, Testcontainers, SQL, or
  resilience dependencies from this bootstrap.

- [x] **Step 2: Verify facade absence and presence**

Run:

```bash
cargo check -p bluetape-rs --locked
cargo check -p bluetape-rs --no-default-features --locked
cargo check -p bluetape-rs --features serialization --locked
```

Expected:

- Default and no-default builds do not require `bluetape_rs::serialization`.
- The root facade path is available only when `features = ["serialization"]` is
  enabled. If no executable doc/example can prove the negative case yet, Step
  6-R must inspect `src/lib.rs` and verify the `#[cfg(feature = "serialization")]`
  guard before PR creation.

- [x] **Step 3: Verify docs and examples**

Run:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc -p bluetape-rs-serialization --all-features --no-deps --locked
RUSTDOCFLAGS="-D warnings" cargo doc -p bluetape-rs --features serialization --no-deps --locked
```

Expected:

- Rustdoc builds with warnings denied.
- Direct crate usage, root facade usage, default-unavailable note,
  no-fallback/mismatch behavior, and migration/compatibility text appear in the
  crate README/Rustdoc pair and localized README.

- [x] **Step 4: Verify formatting and whitespace**

Run:

```bash
cargo fmt --all --check
git diff --check
```

Expected:

- Formatting is stable.
- No whitespace errors.

## Task 5: Workspace Validation And Review Preparation

**Complexity:** medium
**Required skill:** `$bluetape-rs-patterns`

**Files:**
- Verify all changed files.
- Create: `docs/review/2026-06-13-issue-108-serialization-crate-review.md`
- Create: `docs/lessons/2026-06-13-serialization-crate-bootstrap.md`

- [x] **Step 1: Run issue acceptance validation**

Run:

```bash
cargo metadata --no-deps --format-version 1 --locked
cargo check -p bluetape-rs-serialization --all-features --locked
cargo test -p bluetape-rs-serialization --all-features --locked
cargo check -p bluetape-rs --locked
cargo check -p bluetape-rs --features serialization --locked
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
git diff --check
```

Expected:

- All commands pass before Step 5 verification and Step 6-R code review.
- Metadata confirms previous workspace members are preserved and
  `bluetape-rs-serialization` is added.

- [x] **Step 2: Record release-readiness boundary**

Issue #108 does not claim release readiness for the whole `0.5.0` serializer
line. Before any later release-ready claim, run or record the release-guide
dry-run requirement such as:

```bash
cargo publish --workspace --dry-run --locked
```

For this PR, record package metadata evidence from `cargo metadata` and state
that publish dry-run belongs to the later `0.5.0` release-readiness issue.

- [x] **Step 3: Carry Step 2-R deferred P2 items into later issues**

Do not implement binary adapter bounds in issue #108. Record in
`docs/review/2026-06-13-issue-108-serialization-crate-review.md` and the PR body
that later `0.5.0` adapter/release-readiness issues must track:

- encoded-size limit
- decompressed-size limit
- compression ratio limit
- collection bound
- nesting/depth bound
- corrupt bytes
- truncated bytes
- trailing bytes
- empty bytes
- unknown format id
- content-type mismatch
- unsupported version
- wrong target type
- trust-profile mismatch
- oversized payload
- compressed-invalid payload
- adapter failure cases
- executable doc/example checks once examples expose real APIs

These items block later adapter issue closure or `0.5.0` release-readiness, not
issue #108 bootstrap completion.

- [x] **Step 4: Prepare Step 6-R code review evidence**

Review scope:

- Cargo workspace and feature wiring.
- Root facade gating.
- Crate Rustdoc/README boundary.
- Root README locale parity.
- Default dependency exclusion.
- No adapter code, no unsafe deserialization path, no global/default registry.

Expected Step 6-R gate:

- Six review perspectives plus current-session integration.
- `P0=0 P1=0` before lessons, commit, or PR creation.
- Review evidence saved to
  `docs/review/2026-06-13-issue-108-serialization-crate-review.md`.
- Lessons saved to
  `docs/lessons/2026-06-13-serialization-crate-bootstrap.md`.
- PR body final section is `## DoD Status` and includes `P0=0 P1=0` evidence.

## Self-Review

1. Spec coverage:
   - Issue #108 crate bootstrap, workspace registration, root facade gating,
     unchanged defaults, docs, README parity, and explicit non-goals all map to
     tasks above.
   - The plan intentionally does not implement the first binary adapter because
     the reviewed issue #108 slice excludes it.
   - Step 2-R deferred resource-bound and doc/example checks are trackable for
     later adapter/release-readiness work.
2. Placeholder scan:
   - No `TBD`, `TODO`, or vague implementation placeholders are present.
   - Deferred binary-adapter runtime-bound work is explicitly assigned to later
     `0.5.0` issues, not hidden in this bootstrap.
3. Type consistency:
   - Package: `bluetape-rs-serialization`.
   - Library: `bluetape_rs_serialization`.
   - Root feature: `serialization`.
   - Root facade path: `bluetape_rs::serialization`.
