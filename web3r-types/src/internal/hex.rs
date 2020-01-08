use std::{error::Error, fmt};

const HEX_CHARS: &'static [u8] = b"0123456789abcdef";

/// Converts a slice of bytes to a hexadecimal string.
///
/// ```rust
/// use web3r_types::internal::hex::to_hex;
///
/// assert_eq!(&to_hex(&[0xf, 0xff, 0xf], true), "0xfff0f");
/// assert_eq!(&to_hex(&[0, 0, 0x37], true), "0x37");
/// assert_eq!(&to_hex(&[0, 0, 0xf], true), "0xf");
/// assert_eq!(&to_hex(&[0x1], true), "0x1");
///
/// assert_eq!(&to_hex(&[0xf, 0xff, 0xf], false), "0x0fff0f");
/// assert_eq!(&to_hex(&[0, 0, 0x37], false), "0x000037");
/// assert_eq!(&to_hex(&[0, 0, 0xf], false), "0x00000f");
/// assert_eq!(&to_hex(&[0x1], false), "0x01");
/// ```
pub fn to_hex(bytes: impl AsRef<[u8]>, skip_leading_zeros: bool) -> String {
    let bytes = bytes.as_ref();
    let mut index = 0;

    if skip_leading_zeros {
        while index < bytes.len() && bytes[index] == 0 {
            index += 1
        }
    }

    if index >= bytes.len() {
        return "0x0".to_owned();
    }

    let mut out = String::with_capacity((bytes.len() << 1) + 2);
    out.push_str("0x");

    if skip_leading_zeros && bytes[index] < 0x10 {
        unsafe {
            out.as_mut_vec().push(HEX_CHARS[bytes[index] as usize]);
        }

        index += 1;
    }

    for byte in &bytes[index..] {
        unsafe {
            out.as_mut_vec().push(HEX_CHARS[(byte >> 4) as usize]);
            out.as_mut_vec().push(HEX_CHARS[(byte & 0xf) as usize]);
        }
    }

    out
}

#[derive(PartialEq)]
pub enum HexError {
    MissingPrefix,
    InvalidChar { char: char, index: usize },
    IncorrectLen { len: usize, expected: usize },
    LenOverflow { len: usize, at_most: usize },
}

impl fmt::Debug for HexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HexError::MissingPrefix => write!(f, "missing 0x prefix"),

            HexError::InvalidChar { char, index } => write!(
                f,
                "invalid hexadecimal character {} at index {}",
                char, index
            ),

            HexError::IncorrectLen { len, expected } => {
                write!(f, "incorrect length {}, expected {}", len, expected)
            }

            HexError::LenOverflow { len, at_most } => {
                write!(f, "length overflow {}, expected at most {}", len, at_most)
            }
        }
    }
}

impl fmt::Display for HexError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for HexError {}

pub enum ExpectedHexLen {
    Exact(usize),
    AtMost(usize),
}

impl ExpectedHexLen {
    pub fn validate(&self, len: usize) -> Result<(), HexError> {
        match self {
            ExpectedHexLen::Exact(expected) => {
                if len != *expected {
                    return Err(HexError::IncorrectLen {
                        len,
                        expected: *expected,
                    });
                }
            }

            ExpectedHexLen::AtMost(at_most) => {
                if len > *at_most {
                    return Err(HexError::LenOverflow {
                        len,
                        at_most: *at_most,
                    });
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for ExpectedHexLen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpectedHexLen::Exact(len) => write!(f, "length of {}", len),
            ExpectedHexLen::AtMost(len) => write!(f, "length of at most {}", len),
        }
    }
}

/// Converts a hexadecimal string to a `Vec` of bytes.
///
/// ```rust
/// use web3r_types::internal::hex::{from_hex, HexError};
///
/// assert_eq!(from_hex("0xfff0f", None).unwrap(), vec![0xf, 0xff, 0xf]);
/// assert_eq!(from_hex("0x37", None).unwrap(), vec![0x37]);
/// assert_eq!(from_hex("0xf", None).unwrap(), vec![0xf]);
/// assert_eq!(from_hex("0x1", None).unwrap(), vec![0x1]);
///
/// assert_eq!(from_hex("0x0fff0f", None).unwrap(), vec![0xf, 0xff, 0xf]);
/// assert_eq!(from_hex("0x000037", None).unwrap(), vec![0x37]);
/// assert_eq!(from_hex("0x00000f", None).unwrap(), vec![0xf]);
/// assert_eq!(from_hex("0x01", None).unwrap(), vec![0x1]);
///
/// assert_eq!(from_hex("12345", None).unwrap_err(), HexError::MissingPrefix);
/// assert_eq!(from_hex("0x001g", None).unwrap_err(), HexError::InvalidChar { char: 'g', index: 5 });
/// ```
pub fn from_hex<'a>(
    hex: impl AsRef<str>,
    expected_len: impl Into<Option<&'a ExpectedHexLen>>,
) -> Result<Vec<u8>, HexError> {
    let hex = hex.as_ref();

    if !hex.starts_with("0x") {
        return Err(HexError::MissingPrefix);
    }

    if let Some(expected_len) = expected_len.into() {
        expected_len.validate(hex.len())?;
    }

    let mut out = Vec::with_capacity(hex.len() >> 1);
    let mut index = 2;

    while index < hex.len() && hex.as_bytes()[index] == b'0' {
        index += 1;
    }

    let mut byte = 0;
    let mut carry = (hex.len() - index) % 2 == 1;

    for (index, &char) in hex.as_bytes().iter().enumerate().skip(index) {
        byte <<= 4;

        match char {
            b'A'..=b'F' => byte |= char - b'A' + 10,
            b'a'..=b'f' => byte |= char - b'a' + 10,
            b'0'..=b'9' => byte |= char - b'0',

            _ => {
                return Err(HexError::InvalidChar {
                    char: char.into(),
                    index,
                });
            }
        }

        if carry {
            out.push(byte);
            byte = 0;
            carry = false;
        } else {
            carry = true;
        }
    }

    Ok(out)
}
