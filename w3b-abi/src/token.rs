use w3b_types_abi::{Address, Bytes, Int256, Uint256};
use w3b_types_core::hex::unprefixed as hex;

use super::encode;

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Int(Int256),
    Uint(Uint256),
    Bool(bool),
    Address(Address),
    String(String),
    Bytes(Bytes),
    Array(Vec<Token>),
    FixedBytes(Bytes),
    FixedArray(Vec<Token>),
    Tuple(Vec<Token>),
}

impl Token {
    pub fn is_dynamic(&self) -> bool {
        match self {
            Token::String(_) | Token::Bytes(_) | Token::Array(_) => true,
            Token::FixedArray(tokens) => tokens.iter().any(Token::is_dynamic),
            _ => false,
        }
    }

    pub(super) fn encode_into(&self, out: &mut String) {
        use Token::*;

        match self {
            Int(int) => hex::read_exact_into(int.as_bytes(), out),
            Uint(uint) => hex::read_exact_into(uint.as_bytes(), out),
            Bool(bool) => hex::read_left_padded_into(&[*bool as u8], 32, out),
            Address(address) => hex::read_left_padded_into(address.as_bytes(), 32, out),

            FixedBytes(bytes) => encode_bytes_into(bytes.as_bytes(), out),

            String(string) => encode_len_and_bytes_into(string.as_bytes(), out),
            Bytes(bytes) => encode_len_and_bytes_into(bytes.as_bytes(), out),

            Array(tokens) => {
                encode_usize_into(tokens.len(), out);
                encode::encode_into(tokens.as_slice(), out);
            }

            FixedArray(tokens) => encode::encode_into(tokens.as_slice(), out),
            Tuple(tokens) => encode::encode_into(tokens.as_slice(), out),
        }
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
