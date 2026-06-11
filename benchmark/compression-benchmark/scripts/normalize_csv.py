#!/usr/bin/env python3
from __future__ import annotations

import csv
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
DOC_DIR = ROOT / "docs/benchmark"
GO_RAW = DOC_DIR / "raw/go-same-condition.txt"

HEADER = [
    "ecosystem",
    "compressor",
    "direction",
    "payload_kind",
    "payload_size",
    "original_bytes",
    "compressed_bytes",
    "ratio",
    "iterations",
    "total_ns",
    "ns_op",
    "mib_s",
    "timing_provenance",
]

SIZE_BYTES = {"small": 1024, "medium": 65536, "large": 524288}

GO_LINE = re.compile(
    r"^BenchmarkSameConditionCompressors/"
    r"(?P<direction>compress|decompress)/"
    r"(?P<compressor>[^/]+)/"
    r"(?P<kind>[^/]+)/"
    r"(?P<size>[^-]+)-\d+\s+"
    r"(?P<iterations>\d+)\s+"
    r"(?P<ns_op>[0-9.]+)\s+ns/op\s+"
    r"(?P<mb_s>[0-9.]+)\s+MB/s\s+"
    r"(?P<compressed>[0-9.]+)\s+compressed_bytes\s+"
    r"(?P<ratio>[0-9.]+)\s+ratio"
)


def normalize_existing(path: Path, provenance: str) -> None:
    with path.open(newline="") as file:
        rows = list(csv.DictReader(file))
    with path.open("w", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=HEADER)
        writer.writeheader()
        for row in rows:
            normalized = {name: row.get(name, "") for name in HEADER}
            if not normalized["mib_s"] and row.get("mb_s"):
                normalized["mib_s"] = row["mb_s"]
            if not normalized["mib_s"] and row.get("original_bytes") and row.get("ns_op"):
                normalized["mib_s"] = (
                    f"{float(row['original_bytes']) / float(row['ns_op']) * 1_000_000_000.0 / (1024.0 * 1024.0):.2f}"
                )
            normalized["timing_provenance"] = provenance
            writer.writerow(normalized)


def normalize_go(raw_path: Path = GO_RAW) -> None:
    rows: list[dict[str, str]] = []
    for line in raw_path.read_text().splitlines():
        match = GO_LINE.match(line)
        if not match:
            continue
        data = match.groupdict()
        original_bytes = SIZE_BYTES[data["size"]]
        iterations = int(data["iterations"])
        ns_op = float(data["ns_op"])
        rows.append(
            {
                "ecosystem": "bluetape-go",
                "compressor": data["compressor"],
                "direction": data["direction"],
                "payload_kind": data["kind"],
                "payload_size": data["size"],
                "original_bytes": str(original_bytes),
                "compressed_bytes": str(int(float(data["compressed"]))),
                "ratio": data["ratio"],
                "iterations": str(iterations),
                "total_ns": str(round(iterations * ns_op)),
                "ns_op": f"{ns_op:.1f}",
                "mib_s": f"{original_bytes / ns_op * 1_000_000_000.0 / (1024.0 * 1024.0):.2f}",
                "timing_provenance": "go-testing-benchmark",
            }
        )
    if not rows:
        raise SystemExit(f"no Go benchmark rows parsed from {raw_path}")
    with (DOC_DIR / "compression-same-condition-go.csv").open("w", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=HEADER)
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    normalize_existing(DOC_DIR / "compression-same-condition-rust.csv", "rust-instant-loop")
    normalize_existing(DOC_DIR / "compression-same-condition-jvm.csv", "junit-instant-loop")
    normalize_go()


if __name__ == "__main__":
    main()
