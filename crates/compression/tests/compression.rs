use bluetape_rs_compression::{
    CompressionConfig, CompressionError, Compressor, DEFAULT_MAX_DECOMPRESSED_SIZE, algorithms,
};
use std::error::Error;
use std::io::{Read, Write};
use std::sync::{Arc, Barrier};
use std::thread;

#[cfg(any(feature = "zstd", feature = "lz4", feature = "snappy"))]
use bluetape_rs_compression::CompressionLevel;

const PAYLOAD: &[u8] = br#"{"id":1,"name":"compression","tags":["blue","tape","rust"]}"#;
fn large_payload() -> Vec<u8> {
    vec![b'x'; 20_000]
}

#[test]
fn registry_contains_compiled_algorithms() {
    for algorithm in algorithms() {
        assert!(!algorithm.name().is_empty());
    }
}

#[test]
fn default_config_applies_decode_safety_limit() {
    assert_eq!(
        CompressionConfig::new().max_decompressed_size,
        Some(DEFAULT_MAX_DECOMPRESSED_SIZE)
    );
    assert_eq!(
        CompressionConfig::default().max_decompressed_size,
        Some(DEFAULT_MAX_DECOMPRESSED_SIZE)
    );
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
        let corrupted: &[u8] = match algorithm.name() {
            "deflate" => &[0x07],
            "snappy" => &[0x80],
            _ => b"bad",
        };

        let err = algorithm.decompress(corrupted).unwrap_err();
        assert!(
            matches!(
                err,
                CompressionError::Decompress { .. }
                    | CompressionError::DecompressInit { .. }
                    | CompressionError::DecompressRead { .. }
            ),
            "{}",
            algorithm.name()
        );
        assert!(err.source().is_some(), "{}", algorithm.name());
        assert!(err.to_string().contains(algorithm.name()));
    }
}

#[test]
fn compiled_algorithms_reject_decompressed_size_over_limit() {
    for algorithm in algorithms() {
        let compressed = algorithm.compress(PAYLOAD).unwrap();
        let err = algorithm
            .decompress_with_config(
                &compressed,
                CompressionConfig::new().with_max_decompressed_size(PAYLOAD.len() - 1),
            )
            .unwrap_err();
        assert!(
            matches!(err, CompressionError::DecompressedSizeLimitExceeded { .. }),
            "{}",
            algorithm.name()
        );
    }
}

#[test]
fn compiled_algorithms_accept_exact_decompressed_size_limit() {
    for algorithm in algorithms() {
        let compressed = algorithm.compress(PAYLOAD).unwrap();
        let restored = algorithm
            .decompress_with_config(
                &compressed,
                CompressionConfig::new().with_max_decompressed_size(PAYLOAD.len()),
            )
            .unwrap();
        assert_eq!(restored, PAYLOAD, "{}", algorithm.name());
    }
}

#[test]
fn compiled_algorithms_stream_round_trip_payload() {
    for algorithm in algorithms() {
        let mut compressed = Vec::new();
        let plain_bytes = algorithm
            .compress_stream(PAYLOAD, &mut compressed, CompressionConfig::new())
            .unwrap();
        assert_eq!(plain_bytes, PAYLOAD.len() as u64, "{}", algorithm.name());

        let mut restored = Vec::new();
        let restored_bytes = algorithm
            .decompress_stream(&compressed[..], &mut restored, CompressionConfig::new())
            .unwrap();
        assert_eq!(restored_bytes, PAYLOAD.len() as u64, "{}", algorithm.name());
        assert_eq!(restored, PAYLOAD, "{}", algorithm.name());
    }
}

#[test]
fn compiled_algorithms_stream_reject_decompressed_size_over_limit() {
    for algorithm in algorithms() {
        let mut compressed = Vec::new();
        algorithm
            .compress_stream(PAYLOAD, &mut compressed, CompressionConfig::new())
            .unwrap();

        let mut restored = Vec::new();
        let err = algorithm
            .decompress_stream(
                &compressed[..],
                &mut restored,
                CompressionConfig::new().with_max_decompressed_size(PAYLOAD.len() - 1),
            )
            .unwrap_err();
        assert!(
            matches!(err, CompressionError::DecompressedSizeLimitExceeded { .. }),
            "{}",
            algorithm.name()
        );
    }
}

#[test]
fn compiled_algorithms_stream_accept_exact_limit_and_stop_before_overflow_write() {
    for algorithm in algorithms() {
        let payload = large_payload();
        let mut compressed = Vec::new();
        algorithm
            .compress_stream(&payload[..], &mut compressed, CompressionConfig::new())
            .unwrap();

        let mut exact = Vec::new();
        let restored_bytes = algorithm
            .decompress_stream(
                &compressed[..],
                &mut exact,
                CompressionConfig::new().with_max_decompressed_size(payload.len()),
            )
            .unwrap();
        assert_eq!(restored_bytes, payload.len() as u64, "{}", algorithm.name());
        assert_eq!(exact, payload, "{}", algorithm.name());

        let mut partial = Vec::new();
        let err = algorithm
            .decompress_stream(
                &compressed[..],
                &mut partial,
                CompressionConfig::new().with_max_decompressed_size(payload.len() - 1),
            )
            .unwrap_err();
        assert!(
            matches!(err, CompressionError::DecompressedSizeLimitExceeded { .. }),
            "{}",
            algorithm.name()
        );
        assert!(
            partial.len() < payload.len(),
            "{} wrote past the configured limit boundary",
            algorithm.name()
        );
    }
}

#[test]
fn compiled_algorithms_construct_stream_writer_and_reader() {
    for algorithm in algorithms() {
        let mut writer = algorithm
            .compression_writer(Vec::new(), CompressionConfig::new())
            .unwrap();
        writer.write_all(PAYLOAD).unwrap();
        let compressed = writer.finish().unwrap();

        let mut reader = algorithm
            .decompression_reader(&compressed[..], CompressionConfig::new())
            .unwrap();
        let mut restored = Vec::new();
        reader.read_to_end(&mut restored).unwrap();
        assert_eq!(restored, PAYLOAD, "{}", algorithm.name());
    }
}

#[test]
fn compiled_algorithms_report_stream_read_failures_with_source() {
    for algorithm in algorithms() {
        let err = algorithm
            .compress_stream(FailingRead, Vec::new(), CompressionConfig::new())
            .unwrap_err();
        assert!(
            matches!(err, CompressionError::CompressRead { .. }),
            "{}",
            algorithm.name()
        );
        assert!(err.source().is_some(), "{}", algorithm.name());
    }
}

#[test]
fn compiled_algorithms_report_stream_write_failures_with_source() {
    for algorithm in algorithms() {
        let err = algorithm
            .compress_stream(PAYLOAD, FailingWrite, CompressionConfig::new())
            .unwrap_err();
        assert!(
            matches!(
                err,
                CompressionError::CompressWrite { .. } | CompressionError::CompressFinish { .. }
            ),
            "{}",
            algorithm.name()
        );
        assert!(err.source().is_some(), "{}", algorithm.name());

        let mut compressed = Vec::new();
        algorithm
            .compress_stream(PAYLOAD, &mut compressed, CompressionConfig::new())
            .unwrap();

        let err = algorithm
            .decompress_stream(
                &compressed[..],
                FailingWrite,
                CompressionConfig::new().with_max_decompressed_size(PAYLOAD.len()),
            )
            .unwrap_err();
        assert!(
            matches!(err, CompressionError::DecompressWrite { .. }),
            "{}",
            algorithm.name()
        );
        assert!(err.source().is_some(), "{}", algorithm.name());
    }
}

#[test]
fn compiled_algorithms_report_direct_reader_limit_as_typed_source() {
    for algorithm in algorithms() {
        let mut compressed = Vec::new();
        algorithm
            .compress_stream(PAYLOAD, &mut compressed, CompressionConfig::new())
            .unwrap();

        let mut reader = algorithm
            .decompression_reader(
                &compressed[..],
                CompressionConfig::new().with_max_decompressed_size(PAYLOAD.len() - 1),
            )
            .unwrap();
        let mut restored = Vec::new();
        let err = reader.read_to_end(&mut restored).unwrap_err();
        let source = err
            .get_ref()
            .and_then(|source| source.downcast_ref::<CompressionError>())
            .expect("direct reader limit should preserve typed source");
        assert!(
            matches!(
                source,
                CompressionError::DecompressedSizeLimitExceeded { .. }
            ),
            "{}",
            algorithm.name()
        );
    }
}

#[test]
fn custom_compressor_implementors_keep_source_compatibility() {
    #[derive(Clone, Copy)]
    struct Passthrough;

    impl Compressor for Passthrough {
        fn name(&self) -> &'static str {
            "passthrough"
        }

        fn compress_with_config(
            &self,
            plain: &[u8],
            _config: CompressionConfig,
        ) -> Result<Vec<u8>, CompressionError> {
            Ok(plain.to_vec())
        }

        fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>, CompressionError> {
            Ok(compressed.to_vec())
        }
    }

    let mut compressed = Vec::new();
    let copied = Passthrough
        .compress_stream(PAYLOAD, &mut compressed, CompressionConfig::new())
        .unwrap();
    assert_eq!(copied, PAYLOAD.len() as u64);
    assert_eq!(compressed, PAYLOAD);

    let mut restored = Vec::new();
    let copied = Passthrough
        .decompress_stream(&compressed[..], &mut restored, CompressionConfig::new())
        .unwrap();
    assert_eq!(copied, PAYLOAD.len() as u64);
    assert_eq!(restored, PAYLOAD);

    let err = Passthrough
        .compression_writer(Vec::new(), CompressionConfig::new())
        .err()
        .expect("fallback direct writer construction should be unsupported");
    assert!(matches!(
        err,
        CompressionError::UnsupportedOperation {
            operation: "compression_writer",
            ..
        }
    ));

    let err = Passthrough
        .decompression_reader(&compressed[..], CompressionConfig::new())
        .err()
        .expect("fallback direct reader construction should be unsupported");
    assert!(matches!(
        err,
        CompressionError::UnsupportedOperation {
            operation: "decompression_reader",
            ..
        }
    ));
}

struct FailingRead;

impl Read for FailingRead {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("injected read failure"))
    }
}

struct FailingWrite;

impl Write for FailingWrite {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("injected write failure"))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "lz4")]
#[test]
fn lz4_rejects_declared_size_over_limit_before_block_decode() {
    let declared_size = 128 * 1024 * 1024_u32;
    let compressed = declared_size.to_le_bytes();
    let err = bluetape_rs_compression::Lz4
        .decompress_with_config(
            &compressed,
            CompressionConfig::new().with_max_decompressed_size(1024),
        )
        .unwrap_err();
    assert!(matches!(
        err,
        CompressionError::DecompressedSizeLimitExceeded {
            algorithm: "lz4",
            limit: 1024,
            actual,
        } if actual == declared_size as usize
    ));
}

#[cfg(feature = "snappy")]
#[test]
fn snappy_rejects_declared_size_over_limit_before_raw_decode() {
    let declared_size = 128 * 1024 * 1024_usize;
    let compressed = encode_varint(declared_size);
    let err = bluetape_rs_compression::Snappy
        .decompress_with_config(
            &compressed,
            CompressionConfig::new().with_max_decompressed_size(1024),
        )
        .unwrap_err();
    assert!(matches!(
        err,
        CompressionError::DecompressedSizeLimitExceeded {
            algorithm: "snappy",
            limit: 1024,
            actual,
        } if actual == declared_size
    ));
}

#[cfg(feature = "snappy")]
fn encode_varint(mut value: usize) -> Vec<u8> {
    let mut encoded = Vec::new();
    while value >= 0x80 {
        encoded.push((value as u8) | 0x80);
        value >>= 7;
    }
    encoded.push(value as u8);
    encoded
}

#[test]
fn compiled_algorithms_stream_round_trip_stays_stable_across_threads() {
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
                payload.extend_from_slice(format!(":stream:{worker}:{iteration}").as_bytes());

                for algorithm in algorithms() {
                    let mut compressed = Vec::new();
                    algorithm
                        .compress_stream(&payload[..], &mut compressed, CompressionConfig::new())
                        .unwrap();

                    let mut restored = Vec::new();
                    algorithm
                        .decompress_stream(&compressed[..], &mut restored, CompressionConfig::new())
                        .unwrap();
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
        handle
            .join()
            .expect("stream stress worker should not panic");
    }
}

#[test]
fn compiled_algorithms_limit_failures_stay_stable_across_threads() {
    const WORKERS: usize = 8;
    const ITERATIONS: usize = 32;

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
                let mut payload = vec![b'x'; 20_000];
                payload.extend_from_slice(format!(":limit:{worker}:{iteration}").as_bytes());

                for algorithm in algorithms() {
                    let compressed = algorithm.compress(&payload).unwrap();
                    let err = algorithm
                        .decompress_with_config(
                            &compressed,
                            CompressionConfig::new().with_max_decompressed_size(payload.len() - 1),
                        )
                        .unwrap_err();
                    assert!(
                        matches!(err, CompressionError::DecompressedSizeLimitExceeded { .. }),
                        "{} worker={worker} iteration={iteration}",
                        algorithm.name()
                    );

                    let mut streamed = Vec::new();
                    algorithm
                        .compress_stream(&payload[..], &mut streamed, CompressionConfig::new())
                        .unwrap();
                    let mut restored = Vec::new();
                    let err = algorithm
                        .decompress_stream(
                            &streamed[..],
                            &mut restored,
                            CompressionConfig::new().with_max_decompressed_size(payload.len() - 1),
                        )
                        .unwrap_err();
                    assert!(
                        matches!(err, CompressionError::DecompressedSizeLimitExceeded { .. }),
                        "{} stream worker={worker} iteration={iteration}",
                        algorithm.name()
                    );
                    assert!(restored.len() < payload.len(), "{}", algorithm.name());
                }
            }
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("limit failure stress worker should not panic");
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
