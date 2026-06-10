use bluetape_rs_codec::{Base58DecodeError, decode_base58, encode_base58};

const STRESS_WORKERS: usize = 8;
const STRESS_ROUNDS: usize = 512;

#[test]
fn encodes_empty_input() {
    assert_eq!(encode_base58([]), "");
}

#[test]
fn decodes_empty_input() {
    assert_eq!(
        decode_base58("").expect("empty base58 is valid"),
        Vec::<u8>::new()
    );
}

#[test]
fn encodes_known_bitcoin_base58_vector() {
    assert_eq!(encode_base58(b"Hello World!"), "2NEpo7TZRRrLZSi2U");
    assert_eq!(encode_base58(b"Hello, World!"), "72k1xXWG59fYdzSNoA");
}

#[test]
fn round_trips_binary_data() {
    let bytes = [0x00, 0x00, 0x01, 0x7f, 0x80, 0xab, 0xcd, 0xef, 0xff];

    assert_eq!(
        decode_base58(encode_base58(bytes)).expect("valid base58"),
        bytes
    );
}

#[test]
fn preserves_leading_zero_bytes_as_ones() {
    assert_eq!(encode_base58([0, 0, 1]), "112");
    assert_eq!(decode_base58("112").expect("valid base58"), [0, 0, 1]);
}

#[test]
fn rejects_ambiguous_and_non_alphabet_bytes() {
    assert_eq!(
        decode_base58("0OIl").expect_err("ambiguous characters should fail"),
        Base58DecodeError::InvalidCharacter {
            index: 0,
            byte: b'0'
        }
    );
    assert_eq!(
        decode_base58("12 3").expect_err("space should fail"),
        Base58DecodeError::InvalidCharacter {
            index: 2,
            byte: b' '
        }
    );
}

#[test]
fn reports_utf8_bytes_by_byte_position() {
    assert_eq!(
        decode_base58("12é").expect_err("non-ascii byte should fail"),
        Base58DecodeError::InvalidCharacter {
            index: 2,
            byte: 0xc3
        }
    );
}

#[test]
fn formats_decode_errors_for_service_diagnostics() {
    assert_eq!(
        Base58DecodeError::InvalidCharacter {
            index: 3,
            byte: b'0'
        }
        .to_string(),
        "base58 input contains invalid byte 0x30 at position 3"
    );
}

#[test]
fn stress_round_trips_are_stable_across_threads() {
    let cases: &[&[u8]] = &[
        b"",
        b"Hello World!",
        b"Hello, World!",
        &[0, 0, 0, 1],
        &[0, 1, 2, 3, 254, 255],
        &[255; 32],
    ];

    std::thread::scope(|scope| {
        for worker in 0..STRESS_WORKERS {
            scope.spawn(move || {
                for round in 0..STRESS_ROUNDS {
                    let case = cases[(worker + round) % cases.len()];
                    let encoded = encode_base58(case);
                    let decoded = decode_base58(&encoded).expect("valid Base58 round trip");
                    assert_eq!(decoded, case, "worker={worker} round={round}");
                }
            });
        }
    });
}
