use bluetape_rs_codec::{
    Base64DecodeError, decode_base64, decode_base64_unpadded, decode_base64_url,
    decode_base64_url_unpadded, encode_base64, encode_base64_unpadded, encode_base64_url,
    encode_base64_url_unpadded,
};

#[test]
fn encodes_empty_input() {
    assert_eq!(encode_base64([]), "");
    assert_eq!(encode_base64_unpadded([]), "");
    assert_eq!(encode_base64_url([]), "");
    assert_eq!(encode_base64_url_unpadded([]), "");
}

#[test]
fn encodes_standard_padded_and_unpadded_variants() {
    assert_eq!(encode_base64(b"f"), "Zg==");
    assert_eq!(encode_base64(b"fo"), "Zm8=");
    assert_eq!(encode_base64(b"foo"), "Zm9v");
    assert_eq!(encode_base64_unpadded(b"f"), "Zg");
    assert_eq!(encode_base64_unpadded(b"fo"), "Zm8");
    assert_eq!(encode_base64_unpadded(b"foo"), "Zm9v");
}

#[test]
fn encodes_url_safe_padded_and_unpadded_variants() {
    assert_eq!(encode_base64_url([0xfb, 0xff]), "-_8=");
    assert_eq!(encode_base64_url_unpadded([0xfb, 0xff]), "-_8");
    assert_eq!(encode_base64_url([0xfb, 0xff, 0xff]), "-___");
    assert_eq!(encode_base64_url_unpadded([0xfb, 0xff, 0xff]), "-___");
}

#[test]
fn decodes_standard_padded_and_unpadded_variants() {
    assert_eq!(decode_base64("").expect("empty base64"), Vec::<u8>::new());
    assert_eq!(decode_base64("Zg==").expect("padded f"), b"f");
    assert_eq!(decode_base64("Zm8=").expect("padded fo"), b"fo");
    assert_eq!(decode_base64("Zm9v").expect("padded foo"), b"foo");

    assert_eq!(
        decode_base64_unpadded("").expect("empty unpadded base64"),
        Vec::<u8>::new()
    );
    assert_eq!(decode_base64_unpadded("Zg").expect("unpadded f"), b"f");
    assert_eq!(decode_base64_unpadded("Zm8").expect("unpadded fo"), b"fo");
    assert_eq!(
        decode_base64_unpadded("Zm9v").expect("unpadded foo"),
        b"foo"
    );
}

#[test]
fn decodes_url_safe_padded_and_unpadded_variants() {
    assert_eq!(
        decode_base64_url("-_8=").expect("url-safe padded"),
        vec![0xfb, 0xff]
    );
    assert_eq!(
        decode_base64_url_unpadded("-_8").expect("url-safe unpadded"),
        vec![0xfb, 0xff]
    );
}

#[test]
fn standard_decoder_rejects_url_safe_alphabet() {
    assert_eq!(
        decode_base64("-_8=").expect_err("standard alphabet should reject '-'"),
        Base64DecodeError::InvalidByte {
            index: 0,
            byte: b'-'
        }
    );
}

#[test]
fn url_safe_decoder_rejects_standard_alphabet() {
    assert_eq!(
        decode_base64_url("+/8=").expect_err("url-safe alphabet should reject '+'"),
        Base64DecodeError::InvalidByte {
            index: 0,
            byte: b'+'
        }
    );
}

#[test]
fn unpadded_decoders_reject_padding() {
    assert_eq!(
        decode_base64_unpadded("Zm8=").expect_err("padding should fail"),
        Base64DecodeError::InvalidPadding
    );
    assert_eq!(
        decode_base64_url_unpadded("-_8=").expect_err("padding should fail"),
        Base64DecodeError::InvalidPadding
    );
}

#[test]
fn padded_decoders_reject_missing_required_padding() {
    assert_eq!(
        decode_base64("Zm8").expect_err("canonical padding should be required"),
        Base64DecodeError::InvalidPadding
    );
    assert_eq!(
        decode_base64_url("-_8").expect_err("canonical URL-safe padding should be required"),
        Base64DecodeError::InvalidPadding
    );
}

#[test]
fn decoders_report_invalid_length() {
    assert_eq!(
        decode_base64("Z").expect_err("single base64 character is invalid"),
        Base64DecodeError::InvalidLength { len: 1 }
    );
}

#[test]
fn formats_decode_errors_for_service_diagnostics() {
    assert_eq!(
        Base64DecodeError::InvalidByte {
            index: 2,
            byte: b' '
        }
        .to_string(),
        "base64 input contains invalid byte 0x20 at position 2"
    );
    assert_eq!(
        Base64DecodeError::InvalidLength { len: 5 }.to_string(),
        "base64 input has invalid byte length 5"
    );
    assert_eq!(
        Base64DecodeError::InvalidLastSymbol {
            index: 3,
            byte: b'B'
        }
        .to_string(),
        "base64 input contains invalid last symbol 0x42 at position 3"
    );
    assert_eq!(
        Base64DecodeError::InvalidPadding.to_string(),
        "base64 input violates padding policy"
    );
}
