# Issue #81/#82 Step 6-R Review

Scope: `issue-81-82-benchmark-evidence` against `origin/develop`.

Baseline: `5b211ccaa2e2967179580260095e05690c2319f5`.

## Reviewed Surface

- `benchmark/compression-benchmark`
- `docs/benchmark`
- `docs/images/readme-charts`
- `README.md`, `README.ko.md`

## Subagent Lanes

| lane | role | final result | notes |
|---|---|---|---|
| General diff | `code-reviewer` | `P0=0 P1=0` | Fixed fail-fast behavior for missing matrix rows and updated Rust revision provenance. |
| Acceptance verifier | `verifier` | `P0=0 P1=0` | Added per-ecosystem sections, winner summary, and full matrix evidence for #81/#82. |
| Benchmark/runtime | `performance-reviewer` | `P0=0 P1=0` | Fixed provenance and chart scale; residual P2 notes snapshot date/host are fixed metadata. |
| Library user/docs | `library-user-reviewer` | `P0=0 P1=0` | Documented cwd, sibling checkout assumptions, Go local replace, and JVM rerun BLOCKED status. |

## Local Validation Evidence

- `cargo test -p compression-benchmark --locked`
- `cargo test --workspace --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `(cd benchmark/compression-benchmark/go && go test ./...)`
- `python3 -m py_compile benchmark/compression-benchmark/scripts/normalize_csv.py benchmark/compression-benchmark/scripts/render_report.py`
- `(cd benchmark/compression-benchmark/go && go run ./cmd/generate-payloads --output-dir /tmp/bluetape-compression-bench/payloads --manifest ../../../docs/benchmark/compression-fixtures-manifest.csv)`
- `cargo run -p compression-benchmark --release --locked -- --payload-dir /tmp/bluetape-compression-bench/payloads --output /tmp/bluetape-compression-bench/rust-schema-check.csv`
- `python3 benchmark/compression-benchmark/scripts/render_report.py`
- `rsvg-convert` regenerated throughput and ratio PNG charts from SVG.
- `git diff --check`

## Repairs From Initial Review

- Made the Rust benchmark runner emit the normalized `timing_provenance` column directly.
- Made report rendering fail fast when a required matrix row is missing.
- Regenerated the fixture manifest from the same payload bytes as the benchmark fixtures.
- Added per-ecosystem large-payload sections before normalized comparisons.
- Added a winner summary separating throughput winners from compression-ratio winners.
- Expanded the full payload matrix to include payload kind, size, compressor, ecosystem, and direction.
- Updated chart scale to cover current large-payload throughput values.
- Clarified Go-only allocation evidence and non-normalized Rust/JVM memory data.
- Documented cwd and sibling checkout assumptions for Rust, Go, and JVM evidence.
- Marked JVM rerun as BLOCKED because the recorded `bluetape4k-projects` revision has no tracked same-condition compression benchmark selector.
- Synchronized README and README.ko.md benchmark discovery text with the full matrix.

## Gate Verdict

`P0=0 P1=0`

Step 6-R local/native 7-Tier review passes for PR creation.
