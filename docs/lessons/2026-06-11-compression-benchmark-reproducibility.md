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

## Applied In Issue #83

- `docs/benchmark/compression-fixtures-manifest.csv`
- `docs/benchmark/compression-same-condition-metadata.md`
- `benchmark/compression-benchmark/scripts/normalize_csv.py`
- `benchmark/compression-benchmark/scripts/render_report.py`
- `docs/review/2026-06-11-issue-83-compression-benchmark-review.md`
