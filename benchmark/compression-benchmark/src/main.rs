use std::env;
use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::Instant;

use bluetape_rs_compression::algorithms;

#[derive(Debug, Clone)]
struct Options {
    payload_dir: PathBuf,
    output: PathBuf,
}

#[derive(Debug, Clone)]
struct Payload {
    kind: &'static str,
    size: &'static str,
    bytes: Vec<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse()?;
    let payloads = load_payloads(&options.payload_dir)?;
    let mut rows = vec![String::from(
        "ecosystem,compressor,direction,payload_kind,payload_size,original_bytes,compressed_bytes,ratio,iterations,total_ns,ns_op,mib_s",
    )];

    for algorithm in algorithms() {
        for payload in &payloads {
            let compressed = algorithm.compress(&payload.bytes)?;
            let restored = algorithm.decompress(&compressed)?;
            assert_eq!(
                restored,
                payload.bytes,
                "{} {:?} round-trip",
                algorithm.name(),
                payload
            );

            let iterations = iterations_for(payload.size);
            let compress_ns = elapsed(iterations, || {
                black_box(
                    algorithm
                        .compress(black_box(&payload.bytes))
                        .expect("compress"),
                )
                .len()
            });
            rows.push(row(
                algorithm.name(),
                "compress",
                payload,
                compressed.len(),
                iterations,
                compress_ns,
            ));

            let decompress_ns = elapsed(iterations, || {
                black_box(
                    algorithm
                        .decompress(black_box(&compressed))
                        .expect("decompress"),
                )
                .len()
            });
            rows.push(row(
                algorithm.name(),
                "decompress",
                payload,
                compressed.len(),
                iterations,
                decompress_ns,
            ));
        }
    }

    if let Some(parent) = options.output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&options.output, rows.join("\n") + "\n")?;
    println!("rust-compression-benchmark={}", options.output.display());
    Ok(())
}

impl Options {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let mut payload_dir = PathBuf::from("/tmp/bluetape-compression-bench/payloads");
        let mut output = PathBuf::from("docs/benchmark/compression-same-condition-rust.csv");
        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--payload-dir" => {
                    payload_dir = PathBuf::from(args.next().ok_or("--payload-dir needs a value")?);
                }
                "--output" => {
                    output = PathBuf::from(args.next().ok_or("--output needs a value")?);
                }
                other => return Err(format!("unknown argument: {other}").into()),
            }
        }
        Ok(Self {
            payload_dir,
            output,
        })
    }
}

fn load_payloads(dir: &Path) -> Result<Vec<Payload>, Box<dyn std::error::Error>> {
    let mut payloads = Vec::new();
    for kind in ["json", "text", "binary", "random"] {
        for size in ["small", "medium", "large"] {
            let path = dir.join(format!("{kind}-{size}.bin"));
            payloads.push(Payload {
                kind,
                size,
                bytes: fs::read(&path)
                    .map_err(|err| format!("failed to read fixture {}: {err}", path.display()))?,
            });
        }
    }
    Ok(payloads)
}

fn iterations_for(size: &str) -> u32 {
    match size {
        "small" => 500,
        "medium" => 80,
        _ => 10,
    }
}

fn elapsed(iterations: u32, mut run: impl FnMut() -> usize) -> u128 {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(run());
    }
    start.elapsed().as_nanos()
}

fn row(
    compressor: &str,
    direction: &str,
    payload: &Payload,
    compressed_bytes: usize,
    iterations: u32,
    total_ns: u128,
) -> String {
    let ns_op = total_ns as f64 / f64::from(iterations);
    let mib_s = payload.bytes.len() as f64 / ns_op * 1_000_000_000.0 / (1024.0 * 1024.0);
    let ratio = compressed_bytes as f64 / payload.bytes.len() as f64;
    format!(
        "bluetape-rs,{compressor},{direction},{},{},{},{compressed_bytes},{ratio:.6},{iterations},{total_ns},{ns_op:.1},{mib_s:.2}",
        payload.kind,
        payload.size,
        payload.bytes.len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_payload_matrix_in_stable_order() {
        let unique = format!(
            "bluetape-rs-bench-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let dir = std::env::temp_dir().join(unique);
        fs::create_dir_all(&dir).expect("test fixture dir");

        for kind in ["json", "text", "binary", "random"] {
            for size in ["small", "medium", "large"] {
                fs::write(
                    dir.join(format!("{kind}-{size}.bin")),
                    format!("{kind}-{size}"),
                )
                .expect("write fixture");
            }
        }

        let payloads = load_payloads(&dir).expect("load fixtures");
        fs::remove_dir_all(&dir).expect("remove fixture dir");

        assert_eq!(payloads.len(), 12);
        assert_eq!(payloads[0].kind, "json");
        assert_eq!(payloads[0].size, "small");
        assert_eq!(payloads[0].bytes, b"json-small");
        assert_eq!(payloads[11].kind, "random");
        assert_eq!(payloads[11].size, "large");
    }

    #[test]
    fn missing_payload_reports_fixture_path() {
        let unique = format!(
            "bluetape-rs-bench-missing-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let dir = std::env::temp_dir().join(unique);
        fs::create_dir_all(&dir).expect("test fixture dir");

        let err = load_payloads(&dir).expect_err("missing fixture");
        fs::remove_dir_all(&dir).expect("remove fixture dir");

        assert!(
            err.to_string().contains("json-small.bin"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn iteration_schedule_matches_payload_sizes() {
        assert_eq!(iterations_for("small"), 500);
        assert_eq!(iterations_for("medium"), 80);
        assert_eq!(iterations_for("large"), 10);
    }

    #[test]
    fn row_formats_schema_ratio_and_throughput() {
        let payload = Payload {
            kind: "json",
            size: "small",
            bytes: vec![0; 1024],
        };

        let row = row("gzip", "compress", &payload, 256, 4, 4_000);

        assert_eq!(
            row,
            "bluetape-rs,gzip,compress,json,small,1024,256,0.250000,4,4000,1000.0,976.56"
        );
    }
}
