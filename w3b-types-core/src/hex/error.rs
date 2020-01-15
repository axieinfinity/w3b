use std::{error::Error, fmt};

#[derive(PartialEq)]
pub enum HexError {
    MissingPrefix,
    NoDigits,
    InvalidChar { char: char, index: usize },
    InvalidOddLen { len: usize },
    IncorrectLen { len: usize, expected: usize },
    LenTooLong { len: usize, max: usize },
}

impl fmt::Debug for HexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HexError::*;

        match self {
            MissingPrefix => write!(f, "missing 0x prefix"),
            NoDigits => write!(f, "missing hexadecimal digits"),

            InvalidChar { char, index } => write!(
                f,
                "invalid hexadecimal character {} at index {}",
                char, index
            ),

            InvalidOddLen { len } => {
                write!(f, "invalid odd length at {}, expected an even length", len)
            }

            IncorrectLen { len, expected } => {
                write!(f, "incorrect length at {}, expected {}", len, expected)
            }

            LenTooLong { len, max } => {
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
