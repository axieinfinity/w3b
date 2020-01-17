use w3b_types_core::hex::{self as hex_general, unprefixed as hex};

use crate::token::Token;

#[inline]
pub fn encode(tokens: &[Token]) -> String {
    let mut out = String::from("0x");
    encode_tokens_into(tokens, &mut out);
    out
}

fn encode_tokens_into(tokens: &[Token], out: &mut String) {
    let previous_len = out.len();
    let mut dynamic_checkpoints = vec![0; tokens.len()];

    for (index, token) in tokens.iter().enumerate() {
        if token.is_dynamic() {
            out.push_str(&"00".repeat(32));
            dynamic_checkpoints[index] = out.len();
        } else {
            encode_token_into(token, out);
        }
    }

    for (index, token) in tokens.iter().enumerate() {
        if dynamic_checkpoints[index] > 0 {
            let mut offset = (out.len() - previous_len) >> 1;
            let mut index = dynamic_checkpoints[index];

            while offset > 0 {
                index -= 1;

                unsafe {
                    out.as_bytes_mut()[index] = hex_general::HEX_CHARS[offset & 0xf];
                }

                offset >>= 4;
            }

            encode_token_into(token, out);
        }
    }
}

fn encode_token_into(token: &Token, out: &mut String) {
    use Token::*;

    match token {
        Int(int) => hex::read_exact_into(int.as_bytes(), out),
        Uint(uint) => hex::read_exact_into(uint.as_bytes(), out),
        Bool(bool) => hex::read_left_padded_into(&[*bool as u8], 32, out),
        Address(address) => hex::read_left_padded_into(address.as_bytes(), 32, out),

        FixedBytes(bytes) => encode_bytes_into(bytes.as_bytes(), out),

        String(string) => encode_len_and_bytes_into(string.as_bytes(), out),
        Bytes(bytes) => encode_len_and_bytes_into(bytes.as_bytes(), out),

        Array(tokens) => {
            encode_usize_into(tokens.len(), out);
            encode_tokens_into(tokens.as_slice(), out);
        }

        FixedArray(tokens) => encode_tokens_into(tokens.as_slice(), out),
        Tuple(tokens) => encode_tokens_into(tokens.as_slice(), out),
    }
}

#[inline]
fn encode_usize_into(value: usize, out: &mut String) {
    hex::read_left_padded_into(value.to_be_bytes().as_ref(), 32, out);
}

#[inline]
fn encode_bytes_into(bytes: &[u8], out: &mut String) {
    let max_byte_len = (bytes.len() + 31) >> 5 << 5;
    hex::read_right_padded_into(bytes, max_byte_len, out)
}

#[inline]
fn encode_len_and_bytes_into(bytes: &[u8], out: &mut String) {
    encode_usize_into(bytes.len(), out);
    encode_bytes_into(bytes, out)
}

#[cfg(test)]
mod tests {
    use w3b_types_abi::Bytes;

    use super::{encode, Token};

    #[test]
    fn encode_01() {
        assert_eq!(
            encode(&[Token::Uint(69_u8.into()), Token::Bool(true)]),
            vec![
                "0x",
                "0000000000000000000000000000000000000000000000000000000000000045",
                "0000000000000000000000000000000000000000000000000000000000000001",
            ]
            .join(""),
        );
    }

    #[test]
    fn encode_02() {
        assert_eq!(
            encode(&[Token::FixedArray(vec![
                Token::FixedBytes(Bytes::from_bytes("abc".as_bytes())),
                Token::FixedBytes(Bytes::from_bytes("def".as_bytes())),
            ])]),
            vec![
                "0x",
                "6162630000000000000000000000000000000000000000000000000000000000",
                "6465660000000000000000000000000000000000000000000000000000000000",
            ]
            .join(""),
        );
    }

    #[test]
    fn encode_03() {
        assert_eq!(
            encode(&[
                Token::Bytes(Bytes::from_bytes("dave".as_bytes())),
                Token::Bool(true),
                Token::Array(vec![
                    Token::Uint(1_u8.into()),
                    Token::Uint(2_u8.into()),
                    Token::Uint(3_u8.into()),
                ]),
            ]),
            vec![
                "0x",
                "0000000000000000000000000000000000000000000000000000000000000060",
                "0000000000000000000000000000000000000000000000000000000000000001",
                "00000000000000000000000000000000000000000000000000000000000000a0",
                "0000000000000000000000000000000000000000000000000000000000000004",
                "6461766500000000000000000000000000000000000000000000000000000000",
                "0000000000000000000000000000000000000000000000000000000000000003",
                "0000000000000000000000000000000000000000000000000000000000000001",
                "0000000000000000000000000000000000000000000000000000000000000002",
                "0000000000000000000000000000000000000000000000000000000000000003",
            ]
            .join(""),
        );
    }

    #[test]
    fn encode_04() {
        assert_eq!(
            encode(&[
                Token::Uint(0x123_u16.into()),
                Token::Array(vec![
                    Token::Uint(0x456_u16.into()),
                    Token::Uint(0x789_u16.into()),
                ]),
                Token::FixedBytes(Bytes::from_bytes("1234567890".as_bytes())),
                Token::Bytes(Bytes::from_bytes("Hello, world!".as_bytes())),
            ]),
            vec![
                "0x",
                "0000000000000000000000000000000000000000000000000000000000000123",
                "0000000000000000000000000000000000000000000000000000000000000080",
                "3132333435363738393000000000000000000000000000000000000000000000",
                "00000000000000000000000000000000000000000000000000000000000000e0",
                "0000000000000000000000000000000000000000000000000000000000000002",
                "0000000000000000000000000000000000000000000000000000000000000456",
                "0000000000000000000000000000000000000000000000000000000000000789",
                "000000000000000000000000000000000000000000000000000000000000000d",
                "48656c6c6f2c20776f726c642100000000000000000000000000000000000000",
            ]
            .join(""),
        );
    }

    #[test]
    fn encode_05() {
        assert_eq!(
            encode(&[
                Token::Array(vec![
                    Token::Array(vec![Token::Uint(1_u8.into()), Token::Uint(2_u8.into())]),
                    Token::Array(vec![Token::Uint(3_u8.into())]),
                ]),
                Token::Array(vec![
                    Token::String("one".to_owned()),
                    Token::String("two".to_owned()),
                    Token::String("three".to_owned()),
                ]),
            ]),
            vec![
                "0x",
                "0000000000000000000000000000000000000000000000000000000000000040",
                "0000000000000000000000000000000000000000000000000000000000000140",
                "0000000000000000000000000000000000000000000000000000000000000002",
                "0000000000000000000000000000000000000000000000000000000000000040",
                "00000000000000000000000000000000000000000000000000000000000000a0",
                "0000000000000000000000000000000000000000000000000000000000000002",
                "0000000000000000000000000000000000000000000000000000000000000001",
                "0000000000000000000000000000000000000000000000000000000000000002",
                "0000000000000000000000000000000000000000000000000000000000000001",
                "0000000000000000000000000000000000000000000000000000000000000003",
                "0000000000000000000000000000000000000000000000000000000000000003",
                "0000000000000000000000000000000000000000000000000000000000000060",
                "00000000000000000000000000000000000000000000000000000000000000a0",
                "00000000000000000000000000000000000000000000000000000000000000e0",
                "0000000000000000000000000000000000000000000000000000000000000003",
                "6f6e650000000000000000000000000000000000000000000000000000000000",
                "0000000000000000000000000000000000000000000000000000000000000003",
                "74776f0000000000000000000000000000000000000000000000000000000000",
                "0000000000000000000000000000000000000000000000000000000000000005",
                "7468726565000000000000000000000000000000000000000000000000000000",
            ]
            .join(""),
        );
    }
}
