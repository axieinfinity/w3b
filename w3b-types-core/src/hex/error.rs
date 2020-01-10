use std::{error::Error, fmt};

#[derive(PartialEq)]
pub enum HexError {
    MissingPrefix,
    InvalidChar { ch: char, index: usize },
    IncorrectLen { len: usize, expected: usize },
    LenTooLong { len: usize, max: usize },
}

impl fmt::Debug for HexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HexError::MissingPrefix => write!(f, "missing 0x prefix"),

            HexError::InvalidChar { ch, index } => {
                write!(f, "invalid hexadecimal character {} at index {}", ch, index)
            }

            HexError::IncorrectLen { len, expected } => {
                write!(f, "incorrect length at {}, expected {}", len, expected)
            }

            HexError::LenTooLong { len, max } => {
                write!(f, "length too long at {}, expected at most {}", len, max)
            }
        }
    }
}

impl fmt::Display for HexError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl Error for HexError {}
