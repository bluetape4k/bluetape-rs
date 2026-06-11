use bluetape_rs_compression::{CompressionError, algorithms};
use std::sync::{Arc, Barrier};
use std::thread;

#[cfg(any(feature = "zstd", feature = "lz4", feature = "snappy"))]
use bluetape_rs_compression::{CompressionConfig, CompressionLevel};

#[cfg(feature = "zstd")]
use bluetape_rs_compression::Compressor;

const PAYLOAD: &[u8] = br#"{"id":1,"name":"compression","tags":["blue","tape","rust"]}"#;

#[test]
fn registry_contains_compiled_algorithms() {
    for algorithm in algorithms() {
        assert!(!algorithm.name().is_empty());
    }
}

#[test]
fn compiled_algorithms_round_trip_payload() {
    for algorithm in algorithms() {
        let compressed = algorithm.compress(PAYLOAD).unwrap();
        let restored = algorithm.decompress(&compressed).unwrap();
        assert_eq!(restored, PAYLOAD, "{}", algorithm.name());
    }
}

#[test]
fn compiled_algorithms_round_trip_stays_stable_across_threads() {
    const WORKERS: usize = 8;
    const ITERATIONS: usize = 64;

    if algorithms().is_empty() {
        return;
    }

    let start = Arc::new(Barrier::new(WORKERS));
    let mut handles = Vec::with_capacity(WORKERS);

    for worker in 0..WORKERS {
        let start = Arc::clone(&start);
        handles.push(thread::spawn(move || {
            start.wait();

            for iteration in 0..ITERATIONS {
                let mut payload = PAYLOAD.to_vec();
                payload.extend_from_slice(format!(":{worker}:{iteration}").as_bytes());

                for algorithm in algorithms() {
                    let compressed = algorithm.compress(&payload).unwrap();
                    let restored = algorithm.decompress(&compressed).unwrap();
                    assert_eq!(
                        restored,
                        payload,
                        "{} worker={worker} iteration={iteration}",
                        algorithm.name()
                    );
                }
            }
        }));
    }

    for handle in handles {
        handle.join().expect("stress worker should not panic");
    }
}

#[test]
fn compiled_algorithms_round_trip_empty_input() {
    for algorithm in algorithms() {
        let compressed = algorithm.compress(&[]).unwrap();
        let restored = algorithm.decompress(&compressed).unwrap();
        assert!(restored.is_empty(), "{}", algorithm.name());
    }
}

#[test]
fn compiled_algorithms_reject_corrupted_payload() {
    for algorithm in algorithms() {
        let err = algorithm
            .decompress(b"not a valid compressed payload")
            .unwrap_err();
        assert!(
            matches!(err, CompressionError::Decompress { .. }),
            "{}",
            algorithm.name()
        );
    }
}

#[cfg(feature = "zstd")]
#[test]
fn zstd_rejects_custom_levels_that_do_not_fit_i32() {
    let err = bluetape_rs_compression::Zstd
        .compress_with_config(
            PAYLOAD,
            CompressionConfig::new().with_level(CompressionLevel::Custom(u32::MAX)),
        )
        .unwrap_err();

    assert!(matches!(
        err,
        CompressionError::UnsupportedLevel {
            algorithm: "zstd",
            level: u32::MAX,
            ..
        }
    ));
}

#[cfg(any(feature = "lz4", feature = "snappy"))]
#[test]
fn block_codecs_reject_non_default_levels() {
    for algorithm in algorithms() {
        if !matches!(algorithm.name(), "lz4" | "snappy") {
            continue;
        }

        let err = algorithm
            .compress_with_config(
                PAYLOAD,
                CompressionConfig::new().with_level(CompressionLevel::Best),
            )
            .unwrap_err();

        assert!(
            matches!(err, CompressionError::UnsupportedLevel { .. }),
            "{}",
            algorithm.name()
        );
    }
}
