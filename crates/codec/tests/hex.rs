use bluetape_rs_codec::{HexDecodeError, decode_hex, encode_hex_lower, encode_hex_upper};

#[test]
fn encodes_empty_input() {
    assert_eq!(encode_hex_lower([]), "");
    assert_eq!(encode_hex_upper([]), "");
}

#[test]
fn encodes_binary_data_with_lowercase_and_uppercase_variants() {
    let bytes = [0x00, 0x01, 0x7f, 0x80, 0xab, 0xcd, 0xef, 0xff];

    assert_eq!(encode_hex_lower(bytes), "00017f80abcdefff");
    assert_eq!(encode_hex_upper(bytes), "00017F80ABCDEFFF");
}

#[test]
fn decodes_empty_input() {
    assert_eq!(
        decode_hex("").expect("empty hex is valid"),
        Vec::<u8>::new()
    );
}

#[test]
fn decodes_mixed_case_hex() {
    assert_eq!(
        decode_hex("00017F80abcdefff").expect("valid mixed-case hex"),
        vec![0x00, 0x01, 0x7f, 0x80, 0xab, 0xcd, 0xef, 0xff]
    );
}

#[test]
fn rejects_odd_length_with_byte_length() {
    assert_eq!(
        decode_hex("abc").expect_err("odd-length hex should fail"),
        HexDecodeError::OddLength { len: 3 }
    );
}

#[test]
fn rejects_invalid_character_with_byte_position() {
    assert_eq!(
        decode_hex("00xz").expect_err("invalid hex should fail"),
        HexDecodeError::InvalidCharacter {
            index: 2,
            byte: b'x'
        }
    );
}

#[test]
fn rejects_invalid_second_nibble_with_position() {
    assert_eq!(
        decode_hex("0g").expect_err("invalid low nibble should fail"),
        HexDecodeError::InvalidCharacter {
            index: 1,
            byte: b'g'
        }
    );
}

#[test]
fn rejects_non_ascii_input_by_byte_position() {
    assert_eq!(
        decode_hex("00é").expect_err("non-ascii hex should fail"),
        HexDecodeError::InvalidCharacter {
            index: 2,
            byte: 0xc3
        }
    );
}

#[test]
fn formats_decode_errors_for_service_diagnostics() {
    assert_eq!(
        HexDecodeError::OddLength { len: 7 }.to_string(),
        "hex input must have even byte length, got 7"
    );
    assert_eq!(
        HexDecodeError::InvalidCharacter {
            index: 4,
            byte: b'_'
        }
        .to_string(),
        "hex input contains invalid byte 0x5f at position 4"
    );
}
