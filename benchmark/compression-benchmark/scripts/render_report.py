from __future__ import annotations

import csv
import html
import math
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
DOC_DIR = ROOT / "docs/benchmark"
CHART_DIR = ROOT / "docs/images/readme-charts"

ECOSYSTEMS = [
    ("bluetape-rs", "#d99a5b"),
    ("bluetape-go", "#6aaed6"),
    ("bluetape4k-io", "#94c973"),
]
PAYLOADS = ["json", "text", "binary", "random"]
COMMON_COMPRESSORS = ["gzip", "deflate", "lz4", "snappy", "zstd"]
PAYLOAD_FILL = {
    "json": "#e8f3ff",
    "text": "#eaf8ef",
    "binary": "#fff3dd",
    "random": "#f6edff",
}


def load_rows() -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for path in sorted(DOC_DIR.glob("compression-same-condition-*.csv")):
        with path.open() as file:
            rows.extend(csv.DictReader(file))
    return rows


def find_row(rows: list[dict[str, str]], ecosystem: str, payload: str, compressor: str) -> dict[str, str]:
    for row in rows:
        if (
            row["ecosystem"] == ecosystem
            and row["direction"] == "compress"
            and row["payload_kind"] == payload
            and row["payload_size"] == "large"
            and row["compressor"] == compressor
        ):
            return row
    raise KeyError((ecosystem, payload, compressor))


def metric(row: dict[str, str], name: str) -> float:
    return float(row["mib_s"] if name == "throughput" else row["ratio"])


def row_metrics(rows: list[dict[str, str]], ecosystem: str, payload: str, size: str, compressor: str, direction: str) -> tuple[str, str]:
    for row in rows:
        if (
            row["ecosystem"] == ecosystem
            and row["direction"] == direction
            and row["payload_kind"] == payload
            and row["payload_size"] == size
            and row["compressor"] == compressor
        ):
            return f"{float(row['mib_s']):.2f}", f"{float(row['ratio']):.6g}"
    raise KeyError((ecosystem, payload, size, compressor, direction))


def large_compress_row(rows: list[dict[str, str]], ecosystem: str, payload: str, compressor: str) -> dict[str, str]:
    for row in rows:
        if (
            row["ecosystem"] == ecosystem
            and row["direction"] == "compress"
            and row["payload_kind"] == payload
            and row["payload_size"] == "large"
            and row["compressor"] == compressor
        ):
            return row
    raise KeyError((ecosystem, payload, compressor))


def text(x: float, y: float, value: object, cls: str, anchor: str = "start", size: int = 16) -> str:
    return (
        f'<text x="{x:.1f}" y="{y:.1f}" class="{cls}" text-anchor="{anchor}" '
        f'font-size="{size}">{html.escape(str(value))}</text>'
    )


def rect(x: float, y: float, w: float, h: float, fill: str, stroke: str = "#c9d7e4", rx: int = 18) -> str:
    return (
        f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" '
        f'rx="{rx}" fill="{fill}" stroke="{stroke}" stroke-width="1.2"/>'
    )


def log_width(value: float, minimum: float, maximum: float, width: float) -> float:
    return (math.log10(max(value, minimum)) - math.log10(minimum)) / (
        math.log10(maximum) - math.log10(minimum)
    ) * width


def render_chart(rows: list[dict[str, str]], metric_name: str, output: Path) -> None:
    ratio = metric_name == "ratio"
    title = "Same-condition Compression Ratio" if ratio else "Same-condition Compression Throughput"
    subtitle = "Large payloads, lower is better, log scale" if ratio else "Large payloads, higher is better, MiB/s, log scale"
    unit = "ratio" if ratio else "MiB/s"
    ticks = [0.0002, 0.001, 0.01, 0.1, 1.0] if ratio else [50, 100, 300, 1000, 3000, 10000, 30000, 60000]
    minimum = 0.0002 if ratio else 50.0
    maximum = 1.2 if ratio else 60000.0

    width, height = 1900, 1360
    margin = 58
    header_h = 188
    footer_h = 84
    panel_gap = 26
    panel_w = (width - margin * 2 - panel_gap) / 2
    panel_h = (height - header_h - footer_h - margin - panel_gap) / 2
    left_pad = 116
    right_pad = 44
    top_pad = 86
    bottom_pad = 54
    bar_area_w = panel_w - left_pad - right_pad
    group_gap = 42
    bar_h = 9
    bar_gap = 5

    svg = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        "<defs>",
        "<style><![CDATA["
        ".title,.panel-title{font-family:'Architects Daughter','Comic Sans MS',cursive;fill:#243447;}"
        ".detail,.axis,.legend,.value,.muted{font-family:'Comic Mono','SFMono-Regular',monospace;fill:#243447;}"
        ".muted{fill:#64748b;}]]></style>",
        '<filter id="shadow" x="-10%" y="-10%" width="120%" height="130%"><feDropShadow dx="0" dy="8" stdDeviation="10" flood-color="#8aa0b6" flood-opacity="0.16"/></filter>',
        "</defs>",
        f'<rect width="{width}" height="{height}" fill="#eef6fb"/>',
        f'<rect x="{margin / 2}" y="{margin / 2}" width="{width - margin}" height="{height - margin}" rx="34" fill="#fff" stroke="#c7d7e6" stroke-width="2" filter="url(#shadow)"/>',
        text(width / 2, 72, title, "title", "middle", 38),
        text(width / 2, 110, subtitle, "muted", "middle", 19),
    ]

    legend_x = 552
    for idx, (ecosystem, color) in enumerate(ECOSYSTEMS):
        x = legend_x + idx * 260
        svg.append(rect(x, 132, 232, 38, "#f8fbff", "#cbddeb", 13))
        svg.append(f'<rect x="{x + 20}" y="145" width="26" height="12" rx="6" fill="{color}"/>')
        svg.append(text(x + 56, 156, ecosystem, "legend", "start", 15))

    for idx, payload in enumerate(PAYLOADS):
        col = idx % 2
        row = idx // 2
        px = margin + col * (panel_w + panel_gap)
        py = header_h + row * (panel_h + panel_gap)
        axis_x = px + left_pad
        axis_y = py + panel_h - bottom_pad
        plot_top = py + top_pad
        svg.append(rect(px, py, panel_w, panel_h, PAYLOAD_FILL[payload], "#c9d7e4", 24))
        svg.append(text(px + 28, py + 44, payload.upper(), "panel-title", "start", 29))
        svg.append(text(px + panel_w - 30, py + 42, unit, "muted", "end", 16))

        for tick in ticks:
            tx = axis_x + log_width(tick, minimum, maximum, bar_area_w)
            svg.append(f'<line x1="{tx:.1f}" y1="{plot_top:.1f}" x2="{tx:.1f}" y2="{axis_y:.1f}" stroke="#d7e3ee" stroke-width="1"/>')
            svg.append(text(tx, axis_y + 24, f"{tick:g}", "axis", "middle", 12))
        svg.append(f'<line x1="{axis_x:.1f}" y1="{axis_y:.1f}" x2="{axis_x + bar_area_w:.1f}" y2="{axis_y:.1f}" stroke="#6b7280" stroke-width="1.2"/>')

        for cidx, compressor in enumerate(COMMON_COMPRESSORS):
            base_y = plot_top + 22 + cidx * group_gap
            svg.append(text(px + 28, base_y + 17, compressor, "detail", "start", 16))
            for eidx, (ecosystem, color) in enumerate(ECOSYSTEMS):
                row_data = find_row(rows, ecosystem, payload, compressor)
                value = metric(row_data, metric_name)
                bar_w = log_width(value, minimum, maximum, bar_area_w)
                y = base_y + eidx * (bar_h + bar_gap)
                svg.append(f'<rect x="{axis_x:.1f}" y="{y:.1f}" width="{bar_w:.1f}" height="{bar_h}" rx="5" fill="{color}"/>')
                label = f"{value:.4f}" if ratio else f"{value:,.0f}"
                label_x = min(axis_x + bar_w + 7, axis_x + bar_area_w - 2)
                anchor = "start" if axis_x + bar_w + 7 < axis_x + bar_area_w - 2 else "end"
                svg.append(text(label_x, y + 8, label, "value", anchor, 11))

    footer_y = height - footer_h - margin / 2
    svg.append(rect(76, footer_y, width - 152, 54, "#f8fbff", "#d7e3ee", 18))
    svg.append(text(width / 2, footer_y + 23, "Shared fixtures: JSON/Text/Binary/Random x 1 KiB, 64 KiB, 512 KiB", "detail", "middle", 17))
    svg.append(text(width / 2, footer_y + 43, "Common charted compressors: gzip, deflate, lz4, snappy, zstd. zlib is preserved in Rust/Go raw CSV.", "muted", "middle", 14))
    svg.append("</svg>")
    output.write_text("\n".join(svg) + "\n")


def write_markdown(rows: list[dict[str, str]]) -> None:
    lines = [
        "# Same-condition Compression Benchmark",
        "",
        "This report compares `bluetape-rs`, `bluetape-go`, and `bluetape4k-io` with the same payload fixtures.",
        "It is a local same-condition snapshot, not a production ranking or regression threshold.",
        "",
        "![Compression throughput](../images/readme-charts/compression-throughput-large-payloads.svg)",
        "",
        "![Compression ratio](../images/readme-charts/compression-ratio-large-payloads.svg)",
        "",
        "## Run Conditions",
        "",
        "- Date: 2026-06-11",
        "- Host: Apple M5, darwin/arm64",
        "- Repository root: `/Users/debop/work/bluetape4k/bluetape-rs`",
        "- Required sibling checkouts: `/Users/debop/work/bluetape4k/bluetape-go` and `/Users/debop/work/bluetape4k/bluetape4k-projects` at the revisions in metadata.",
        "- Fixtures: `(cd benchmark/compression-benchmark/go && go run ./cmd/generate-payloads --output-dir /tmp/bluetape-compression-bench/payloads --manifest ../../../docs/benchmark/compression-fixtures-manifest.csv)`",
        "- Rust cwd: repository root; command: `cargo run -p compression-benchmark --release --locked -- --payload-dir /tmp/bluetape-compression-bench/payloads --output docs/benchmark/compression-same-condition-rust.csv`",
        "- Go cwd: `benchmark/compression-benchmark/go`; command: `go test -run '^$' -bench '^BenchmarkSameConditionCompressors' -benchmem -benchtime=100ms -count=1 ./... > ../../../docs/benchmark/raw/go-same-condition.txt`",
        "- JVM: tracked CSV preserved from the same local snapshot; rerun is BLOCKED because the recorded `bluetape4k-projects` revision does not contain a tracked same-condition benchmark test selector.",
        "- Shared fixtures: `/tmp/bluetape-compression-bench/payloads`",
        "- Matrix: JSON/Text/Binary/Random x small 1 KiB, medium 64 KiB, large 512 KiB",
        "- Throughput: higher is better; all CSV/report/chart throughput values are normalized to MiB/s",
        "- Compression ratio: lower is better",
        "- Go allocation counters are preserved in `docs/benchmark/raw/go-same-condition.txt`; the normalized cross-ecosystem CSV keeps common metrics only.",
        "- Source revisions and fixture hashes: `docs/benchmark/compression-same-condition-metadata.md`",
        "- CSV schema is normalized across ecosystems; `timing_provenance` records the source harness.",
        "",
        "## Caveats",
        "",
        "- This is a single local run on one host.",
        "- Rust, Go, and JVM use different lightweight harnesses, so short-window timing noise is expected.",
        "- Use the table for broad comparison under identical payload bytes, not for stable production rankings.",
        "- Allocation data is Go-only raw `-benchmem` evidence and is not normalized across Rust/JVM/Go.",
        "- `zlib` is preserved in Rust/Go raw CSVs but excluded from common charts because the JVM comparison set does not include zlib.",
        "",
        "## Per-Ecosystem Large Payload Snapshots",
        "",
        "These per-ecosystem snapshots come first so raw ecosystem behavior stays visible before normalized comparisons.",
        "",
    ]
    for ecosystem, _ in ECOSYSTEMS:
        lines.extend(
            [
                f"### {ecosystem}",
                "",
                "| payload | compressor | MiB/s | ratio |",
                "|---|---:|---:|---:|",
            ]
        )
        for payload in PAYLOADS:
            for compressor in COMMON_COMPRESSORS:
                row = large_compress_row(rows, ecosystem, payload, compressor)
                lines.append(f"| {payload} | {compressor} | {float(row['mib_s']):.2f} | {float(row['ratio']):.6g} |")
        lines.append("")
    lines.extend(
        [
            "## Winner Summary",
            "",
            "Large-payload compression winners are separated by metric because faster compressors are not always the smallest-output compressors.",
            "",
            "| payload | throughput winner | MiB/s | ratio winner | ratio |",
            "|---|---:|---:|---:|---:|",
        ]
    )
    for payload in PAYLOADS:
        candidates = [
            (ecosystem, compressor, large_compress_row(rows, ecosystem, payload, compressor))
            for ecosystem, _ in ECOSYSTEMS
            for compressor in COMMON_COMPRESSORS
        ]
        throughput = max(candidates, key=lambda item: float(item[2]["mib_s"]))
        ratio_winner = min(candidates, key=lambda item: float(item[2]["ratio"]))
        lines.append(
            f"| {payload} | {throughput[0]} {throughput[1]} | {float(throughput[2]['mib_s']):.2f} | "
            f"{ratio_winner[0]} {ratio_winner[1]} | {float(ratio_winner[2]['ratio']):.6g} |"
        )
    lines.extend(
        [
            "",
            "## Recommendation Notes",
            "",
            "These recommendations are scoped to this local same-condition snapshot and should not be read as production rankings.",
            "",
            "| use case | recommendation | excluded interpretation |",
            "|---|---|---|",
            "| Best compression ratio on compressible JSON/text/binary payloads | Prefer evaluating `zstd` first, then validate CPU cost with your service payloads. | This does not make `zstd` the default for every deployment or every payload size. |",
            "| Maximum throughput where modestly larger output is acceptable | Evaluate `lz4` and `snappy` first; they dominate many large-payload throughput rows. | Throughput wins do not imply smallest network/storage footprint. |",
            "| Interoperability with common gzip/deflate ecosystems | Keep `gzip` and `deflate` documented as compatibility choices. | Compatibility-oriented choices are not usually throughput or ratio winners in this snapshot. |",
            "| Random or already-compressed payloads | Avoid assuming compression helps; ratios are near or above 1.0 and CPU cost remains. | Random-payload results should not be generalized to structured service data. |",
            "| Cross-runtime comparison | Use the normalized tables for broad direction only and rerun in the target runtime before changing defaults. | The Rust, Go, and JVM harnesses are not statistically identical benchmark engines. |",
            "",
            "## Normalized Large Payload Comparison",
        "",
        "| payload | compressor | rs MiB/s | rs ratio | go MiB/s | go ratio | jvm MiB/s | jvm ratio |",
        "|---|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for payload in PAYLOADS:
        for compressor in COMMON_COMPRESSORS:
            values = []
            for ecosystem, _ in ECOSYSTEMS:
                row = find_row(rows, ecosystem, payload, compressor)
                values.extend([f"{float(row['mib_s']):.2f}", f"{float(row['ratio']):.6g}"])
            lines.append(f"| {payload} | {compressor} | {' | '.join(values)} |")
    lines.extend(
        [
            "",
            "## Full Payload Matrix",
            "",
            "The normalized tables below include compression and decompression throughput for every shared payload kind, payload size, compressor, ecosystem, and operation direction.",
            "",
        ]
    )
    for direction in ["compress", "decompress"]:
        lines.extend(
            [
                f"### {direction.title()}",
                "",
                "| payload | size | compressor | rs MiB/s | rs ratio | go MiB/s | go ratio | jvm MiB/s | jvm ratio |",
                "|---|---:|---:|---:|---:|---:|---:|---:|---:|",
            ]
        )
        for payload in PAYLOADS:
            for size in ["small", "medium", "large"]:
                for compressor in COMMON_COMPRESSORS:
                    values = []
                    for ecosystem, _ in ECOSYSTEMS:
                        values.extend(row_metrics(rows, ecosystem, payload, size, compressor, direction))
                    lines.append(f"| {payload} | {size} | {compressor} | {' | '.join(values)} |")
        lines.append("")
    lines.extend(
        [
            "",
            "## Raw CSV",
            "",
            "- `docs/benchmark/compression-same-condition-rust.csv`",
            "- `docs/benchmark/compression-same-condition-go.csv`",
            "- `docs/benchmark/compression-same-condition-jvm.csv`",
            "- `docs/benchmark/compression-fixtures-manifest.csv`",
            "- `docs/benchmark/raw/go-same-condition.txt`",
            "- `docs/benchmark/compression-same-condition-metadata.md`",
            "",
        ]
    )
    (DOC_DIR / "compression-same-condition-benchmark.md").write_text("\n".join(lines))


def main() -> None:
    CHART_DIR.mkdir(parents=True, exist_ok=True)
    DOC_DIR.mkdir(parents=True, exist_ok=True)
    rows = load_rows()
    render_chart(rows, "throughput", CHART_DIR / "compression-throughput-large-payloads.svg")
    render_chart(rows, "ratio", CHART_DIR / "compression-ratio-large-payloads.svg")
    write_markdown(rows)


if __name__ == "__main__":
    main()
