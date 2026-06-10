use bluetape_rs_codec::{
    TextDecodeError, decode_base64_url_unpadded, decode_utf8_text, decode_utf8_text_lossy,
    encode_base64_url_unpadded, encode_utf8_text,
};

#[test]
fn encodes_utf8_text_for_binary_codecs() {
    assert_eq!(encode_utf8_text(""), Vec::<u8>::new());
    assert_eq!(encode_utf8_text("blue테이프"), "blue테이프".as_bytes());
    assert_eq!(
        encode_base64_url_unpadded(encode_utf8_text("blue테이프")),
        "Ymx1Ze2FjOydtO2UhA"
    );
}

#[test]
fn decodes_utf8_text_after_binary_decoding() {
    let bytes = decode_base64_url_unpadded("Ymx1Ze2FjOydtO2UhA").expect("valid encoded text bytes");

    assert_eq!(
        decode_utf8_text(bytes).expect("valid UTF-8 text"),
        "blue테이프"
    );
}

#[test]
fn rejects_invalid_utf8_without_lossy_replacement() {
    assert_eq!(
        decode_utf8_text([b'a', 0xff, b'z']).expect_err("invalid UTF-8 should fail"),
        TextDecodeError::InvalidUtf8 {
            valid_up_to: 1,
            error_len: Some(1)
        }
    );
}

#[test]
fn reports_incomplete_utf8_sequence() {
    assert_eq!(
        decode_utf8_text([0xe2, 0x82]).expect_err("incomplete UTF-8 should fail"),
        TextDecodeError::InvalidUtf8 {
            valid_up_to: 0,
            error_len: None
        }
    );
}

#[test]
fn decodes_utf8_text_lossy_only_when_callers_opt_in() {
    assert_eq!(decode_utf8_text_lossy([b'a', 0xff, b'z']), "a\u{fffd}z");
}

#[test]
fn formats_decode_errors_for_service_diagnostics() {
    assert_eq!(
        TextDecodeError::InvalidUtf8 {
            valid_up_to: 2,
            error_len: Some(1)
        }
        .to_string(),
        "text input contains invalid UTF-8 sequence of length 1 after byte 2"
    );
    assert_eq!(
        TextDecodeError::InvalidUtf8 {
            valid_up_to: 3,
            error_len: None
        }
        .to_string(),
        "text input contains incomplete UTF-8 sequence after byte 3"
    );
}
