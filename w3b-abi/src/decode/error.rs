use std::{error::Error, fmt};

use w3b_types_core::hex::HexError;

#[derive(PartialEq)]
pub enum DecodeError {
    Hex {
        inner: HexError,
    },
    UnexpectedChar {
        char: char,
        index: usize,
        expected: Vec<char>,
    },
    InvalidUtf8 {
        valid_up_to: usize,
        invalid_size: Option<usize>,
    },
}

impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::Hex { inner } => inner.fmt(f),

            DecodeError::UnexpectedChar {
                char,
                index,
                expected,
            } => write!(
                f,
                "unexpected character {} at index {}, expected {:?}",
                char, index, expected,
            ),

            DecodeError::InvalidUtf8 {
                valid_up_to,
                invalid_size,
            } => write!(
                f,
                "invalid UTF-8 bytes (valid up to {}{})",
                valid_up_to,
                invalid_size
                    .map(|invalid_size| format!(", invalid size {}", invalid_size))
                    .unwrap_or_default(),
            ),
        }
    }
}

impl fmt::Display for DecodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl Error for DecodeError {}

impl From<HexError> for DecodeError {
    #[inline]
    fn from(inner: HexError) -> Self {
        DecodeError::Hex { inner }
    }
}
