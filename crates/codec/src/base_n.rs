//! Shared base-N byte conversion for base-family codecs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BaseNDecodeError {
    InvalidCharacter { index: usize, byte: u8 },
}

pub(crate) fn encode_base_n(bytes: &[u8], alphabet: &[u8]) -> String {
    debug_assert!((2..=256).contains(&alphabet.len()));

    if bytes.is_empty() {
        return String::new();
    }

    let zeros = bytes
        .iter()
        .position(|byte| *byte != 0)
        .unwrap_or(bytes.len());
    let mut input = bytes.to_vec();
    let mut encoded = vec![0_u8; input.len() * 2];
    let mut output_start = encoded.len();
    let mut input_start = zeros;

    while input_start < input.len() {
        let remainder = divmod(&mut input, input_start, 256, alphabet.len() as u32);
        output_start -= 1;
        encoded[output_start] = alphabet[remainder as usize];
        if input[input_start] == 0 {
            input_start += 1;
        }
    }

    while output_start < encoded.len() && encoded[output_start] == alphabet[0] {
        output_start += 1;
    }
    for _ in 0..zeros {
        output_start -= 1;
        encoded[output_start] = alphabet[0];
    }

    String::from_utf8(encoded[output_start..].to_vec())
        .expect("base-N alphabets must contain ASCII bytes")
}

pub(crate) fn decode_base_n(encoded: &str, alphabet: &[u8]) -> Result<Vec<u8>, BaseNDecodeError> {
    debug_assert!((2..=256).contains(&alphabet.len()));

    if encoded.is_empty() {
        return Ok(Vec::new());
    }

    let mut digits = Vec::with_capacity(encoded.len());
    for (index, byte) in encoded.bytes().enumerate() {
        let digit = alphabet.iter().position(|candidate| *candidate == byte);
        match digit {
            Some(digit) => digits.push(digit as u8),
            None => return Err(BaseNDecodeError::InvalidCharacter { index, byte }),
        }
    }

    let zeros = digits
        .iter()
        .position(|digit| *digit != 0)
        .unwrap_or(digits.len());
    let mut decoded = vec![0_u8; encoded.len()];
    let mut output_start = decoded.len();
    let mut input_start = zeros;

    while input_start < digits.len() {
        let remainder = divmod(&mut digits, input_start, alphabet.len() as u32, 256);
        output_start -= 1;
        decoded[output_start] = remainder as u8;
        if digits[input_start] == 0 {
            input_start += 1;
        }
    }

    while output_start < decoded.len() && decoded[output_start] == 0 {
        output_start += 1;
    }

    Ok(decoded[(output_start - zeros)..].to_vec())
}

fn divmod(number: &mut [u8], first_digit: usize, base: u32, divisor: u32) -> u32 {
    let mut remainder = 0_u32;
    for digit in number.iter_mut().skip(first_digit) {
        let value = remainder * base + u32::from(*digit);
        *digit = (value / divisor) as u8;
        remainder = value % divisor;
    }
    remainder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_base58_style_values() {
        let alphabet = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        let bytes = [0, 0, 1, 2, 3, 254, 255];

        let encoded = encode_base_n(&bytes, alphabet);

        assert_eq!(
            decode_base_n(&encoded, alphabet).expect("valid base"),
            bytes
        );
    }

    #[test]
    fn round_trips_base62_style_values() {
        let alphabet = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let bytes = [0, 0, 1, 2, 3, 254, 255];

        let encoded = encode_base_n(&bytes, alphabet);

        assert_eq!(
            decode_base_n(&encoded, alphabet).expect("valid base"),
            bytes
        );
    }
}
