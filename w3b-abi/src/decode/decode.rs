use w3b_types_abi::{Address, Bytes, Int256, Uint256};
use w3b_types_core::{
    hex as hex_general,
    hex::{unprefixed as hex, HexError},
};

use crate::{param_type::ParamType, token::Token};

use super::error::DecodeError;

#[inline]
pub fn decode(input: &str, types: &[ParamType]) -> Result<Vec<Token>, DecodeError> {
    let input = hex_general::strip_prefix(input)?;

    let (tokens, _) = decode_tokens(input, types.len(), |index| &types[index])
        .map_err(shift_decode_indices(2))?;

    Ok(tokens)
}

fn decode_tokens<'a>(
    input: &str,
    size: usize,
    ty: impl Fn(usize) -> &'a ParamType,
) -> Result<(Vec<Token>, usize), DecodeError> {
    let mut offset = 0;
    let mut tokens = Vec::new();

    for index in 0..size {
        let ty = ty(index);

        if ty.is_dynamic() {
            let dynamic_offset =
                decode_usize(&input[offset..]).map_err(shift_indices(offset))? << 1;

            let (token, _) = decode_token(&input[dynamic_offset..], ty)
                .map_err(shift_decode_indices(dynamic_offset))?;

            tokens.push(token);
            offset += 64;
        } else {
            let (token, next_offset) =
                decode_token(&input[offset..], ty).map_err(shift_decode_indices(offset))?;

            tokens.push(token);
            offset += next_offset;
        }
    }

    Ok((tokens, offset))
}

fn decode_token(input: &str, ty: &ParamType) -> Result<(Token, usize), DecodeError> {
    Ok(match ty {
        ParamType::Int(_) => (Token::Int(Int256::from_hex_unprefixed(&input[..64])?), 64),
        ParamType::Uint(_) => (Token::Uint(Uint256::from_hex_unprefixed(&input[..64])?), 64),

        ParamType::Bool => {
            let bytes = input.as_bytes();

            for index in 0..63 {
                if bytes[index] != b'0' {
                    return Err(DecodeError::UnexpectedChar {
                        char: bytes[index].into(),
                        index,
                        expected: vec!['0'],
                    });
                }
            }

            if bytes[63] != b'0' && bytes[63] != b'1' {
                return Err(DecodeError::UnexpectedChar {
                    char: bytes[63].into(),
                    index: 63,
                    expected: vec!['0', '1'],
                });
            }

            (Token::Bool(bytes[63] == b'1'), 64)
        }

        ParamType::Address => (
            Token::Address(
                Address::from_hex_unprefixed(&input[24..64]).map_err(shift_indices(24))?,
            ),
            64,
        ),

        ParamType::FixedBytes(size) => {
            let (bytes, next_offset) = decode_sized_bytes(input, *size)?;
            (Token::FixedBytes(Bytes::new(bytes)), next_offset)
        }

        ParamType::String => {
            let (bytes, next_offset) = decode_bytes(input)?;

            let string = String::from_utf8(bytes).map_err(|error| {
                let error = error.utf8_error();

                DecodeError::InvalidUtf8 {
                    valid_up_to: error.valid_up_to(),
                    invalid_size: error.error_len(),
                }
            })?;

            (Token::String(string), next_offset)
        }

        ParamType::Bytes => {
            let (bytes, next_offset) = decode_bytes(input)?;
            (Token::Bytes(Bytes::new(bytes)), next_offset)
        }

        ParamType::Array(subtype) => {
            let size = decode_usize(input)?;

            let (tokens, next_offset) = decode_tokens(&input[64..], size, |_| subtype.as_ref())
                .map_err(shift_decode_indices(64))?;

            (Token::Array(tokens), next_offset + 64)
        }

        ParamType::FixedArray(subtype, size) => {
            let (tokens, next_offset) = decode_tokens(input, *size, |_| subtype.as_ref())?;
            (Token::FixedArray(tokens), next_offset)
        }

        ParamType::Tuple(subtypes) => {
            let (tokens, next_offset) = decode_tokens(input, subtypes.len(), |index| {
                subtypes.get(index).unwrap().as_ref()
            })?;

            (Token::Tuple(tokens), next_offset)
        }
    })
}

#[inline]
fn decode_usize(input: &str) -> Result<usize, HexError> {
    let offset = 64 - (std::mem::size_of::<usize>() << 1);
    let mut repr = [0; std::mem::size_of::<usize>()];
    hex::write_exact_into(&input[offset..64], &mut repr).map_err(shift_indices(offset))?;
    Ok(usize::from_be_bytes(repr))
}

#[inline]
fn decode_sized_bytes(input: &str, size: usize) -> Result<(Vec<u8>, usize), HexError> {
    let bytes = hex::write_exact(&input[..size << 1])?;
    let next_offset = ((size << 1) + 63) >> 6 << 6;
    Ok((bytes, next_offset))
}

#[inline]
fn decode_bytes(input: &str) -> Result<(Vec<u8>, usize), HexError> {
    let size = decode_usize(input)?;
    let (bytes, next_offset) = decode_sized_bytes(&input[64..], size).map_err(shift_indices(64))?;
    Ok((bytes, next_offset + 64)) // The first 32 bytes are for the size.
}

#[inline]
fn shift_indices(shift: usize) -> impl FnOnce(HexError) -> HexError {
    move |error| hex_general::shift_indices(error, shift)
}

fn shift_decode_indices(shift: usize) -> impl FnOnce(DecodeError) -> DecodeError {
    use DecodeError::*;

    move |error| match error {
        Hex { inner } => Hex {
            inner: hex_general::shift_indices(inner, shift),
        },

        UnexpectedChar {
            char,
            index,
            expected,
        } => UnexpectedChar {
            char,
            index: index + shift,
            expected,
        },

        InvalidUtf8 {
            valid_up_to,
            invalid_size,
        } => InvalidUtf8 {
            valid_up_to: valid_up_to + shift,
            invalid_size,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{decode, Bytes, ParamType, Token};

    #[test]
    fn decode_01() {
        assert_eq!(
            decode(
                &vec![
                    "0x",
                    "0000000000000000000000000000000000000000000000000000000000000045",
                    "0000000000000000000000000000000000000000000000000000000000000001",
                ]
                .join(""),
                &[ParamType::Uint(8), ParamType::Bool],
            )
            .unwrap(),
            vec![Token::Uint(69_u8.into()), Token::Bool(true)],
        );
    }

    #[test]
    fn decode_02() {
        assert_eq!(
            decode(
                &vec![
                    "0x",
                    "6162630000000000000000000000000000000000000000000000000000000000",
                    "6465660000000000000000000000000000000000000000000000000000000000",
                ]
                .join(""),
                &[ParamType::FixedArray(Box::new(ParamType::FixedBytes(3)), 2)],
            )
            .unwrap(),
            vec![Token::FixedArray(vec![
                Token::FixedBytes(Bytes::from_bytes("abc".as_bytes())),
                Token::FixedBytes(Bytes::from_bytes("def".as_bytes())),
            ])],
        );
    }

    #[test]
    fn decode_03() {
        assert_eq!(
            decode(
                &vec![
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
                &[
                    ParamType::Bytes,
                    ParamType::Bool,
                    ParamType::Array(Box::new(ParamType::Uint(256))),
                ],
            )
            .unwrap(),
            &[
                Token::Bytes(Bytes::from_bytes("dave".as_bytes())),
                Token::Bool(true),
                Token::Array(vec![
                    Token::Uint(1_u8.into()),
                    Token::Uint(2_u8.into()),
                    Token::Uint(3_u8.into()),
                ]),
            ],
        );
    }

    #[test]
    fn decode_04() {
        assert_eq!(
            decode(
                &vec![
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
                &[
                    ParamType::Uint(256),
                    ParamType::Array(Box::new(ParamType::Uint(32))),
                    ParamType::FixedBytes(10),
                    ParamType::Bytes,
                ],
            )
            .unwrap(),
            &[
                Token::Uint(0x123_u16.into()),
                Token::Array(vec![
                    Token::Uint(0x456_u16.into()),
                    Token::Uint(0x789_u16.into()),
                ]),
                Token::FixedBytes(Bytes::from_bytes("1234567890".as_bytes())),
                Token::Bytes(Bytes::from_bytes("Hello, world!".as_bytes())),
            ],
        );
    }

    #[test]
    fn decode_05() {
        assert_eq!(
            decode(
                &vec![
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
                &[
                    ParamType::Array(Box::new(ParamType::Array(Box::new(ParamType::Uint(256))),)),
                    ParamType::Array(Box::new(ParamType::String)),
                ],
            )
            .unwrap(),
            &[
                Token::Array(vec![
                    Token::Array(vec![Token::Uint(1_u8.into()), Token::Uint(2_u8.into())]),
                    Token::Array(vec![Token::Uint(3_u8.into())]),
                ]),
                Token::Array(vec![
                    Token::String("one".to_owned()),
                    Token::String("two".to_owned()),
                    Token::String("three".to_owned()),
                ]),
            ],
        );
    }
}
