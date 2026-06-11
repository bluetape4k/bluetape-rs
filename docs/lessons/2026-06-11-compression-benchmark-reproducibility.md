# Compression Benchmark Reproducibility

When benchmark results cross `bluetape-rs`, `bluetape-go`, and `bluetape4k-io`,
the report needs enough local evidence to reconstruct the comparison without
trusting `/tmp` state or memory.

## Lesson

- Keep fixture hashes in a tracked manifest.
- Record sibling repository revisions beside the report.
- Normalize CSV schemas before charting, including units and provenance.
- Use `MiB/s` or `MB/s` consistently; do not mix Rust hand calculations with
  Go benchmark labels without conversion.
- Label short local runs as same-condition snapshots, not production rankings.
- Avoid local filesystem paths in generated SVG assets.
- Keep unreleased crate install snippets as path/Git examples until the release
  exists.
- Make generated reports fail fast when a required benchmark row is missing;
  `n/a` placeholders can hide incomplete evidence.
- Keep source revision provenance tied to the actual review baseline, not an
  earlier implementation commit.
- Public reproduction commands must name the cwd, sibling checkout assumptions,
  and exact BLOCKED reason when a cross-repo harness is not tracked.

## Applied In Issue #83

- `docs/benchmark/compression-fixtures-manifest.csv`
- `docs/benchmark/compression-same-condition-metadata.md`
- `benchmark/compression-benchmark/scripts/normalize_csv.py`
- `benchmark/compression-benchmark/scripts/render_report.py`
- `docs/review/2026-06-11-issue-83-compression-benchmark-review.md`

## Applied In Issues #81/#82

- `benchmark/compression-benchmark/src/main.rs`
- `benchmark/compression-benchmark/go/cmd/generate-payloads`
- `benchmark/compression-benchmark/scripts/render_report.py`
- `docs/benchmark/compression-same-condition-benchmark.md`
- `docs/benchmark/compression-same-condition-metadata.md`
- `docs/review/2026-06-11-issue-81-82-benchmark-evidence-review.md`
