use super::error::HexError;

const HEX_CHARS: &'static [u8] = b"0123456789abcdef";

/// ```rust
/// # use w3b_types_core::hex::read;
/// assert_eq!(&read(&[0xf, 0xff, 0xf]), "0xfff0f");
/// assert_eq!(&read(&[0, 0, 0xf]), "0xf");
/// assert_eq!(&read(&[0]), "0x0");
/// assert_eq!(&read(&[]), "0x0");
/// ```
#[inline]
pub fn read(bytes: &[u8]) -> String {
    let mut hex = String::from("0x");
    unprefixed::read_into(bytes, &mut hex);
    hex
}

/// ```rust
/// # use w3b_types_core::hex::read_left_padded;
/// assert_eq!(&read_left_padded(&[0xf, 0xff, 0xf], 3), "0x0fff0f");
/// assert_eq!(&read_left_padded(&[0xf], 2), "0x000f");
/// assert_eq!(&read_left_padded(&[0], 2), "0x0000");
/// assert_eq!(&read_left_padded(&[], 2), "0x0000");
/// ```
#[inline]
pub fn read_left_padded(bytes: &[u8], max_byte_len: usize) -> String {
    let mut hex = String::from("0x");
    unprefixed::read_left_padded_into(bytes, max_byte_len, &mut hex);
    hex
}

/// ```rust
/// # use w3b_types_core::hex::read_right_padded;
/// assert_eq!(&read_right_padded(&[0xf, 0xff, 0xf], 3), "0x0fff0f");
/// assert_eq!(&read_right_padded(&[0xf], 2), "0x0f00");
/// assert_eq!(&read_right_padded(&[0], 2), "0x0000");
/// assert_eq!(&read_right_padded(&[], 2), "0x0000");
/// ```
#[inline]
pub fn read_right_padded(bytes: &[u8], max_byte_len: usize) -> String {
    let mut hex = String::from("0x");
    unprefixed::read_right_padded_into(bytes, max_byte_len, &mut hex);
    hex
}

/// ```rust
/// # use w3b_types_core::hex::read_exact;
/// assert_eq!(read_exact(&[0xf, 0xff, 0xf]), "0x0fff0f");
/// assert_eq!(read_exact(&[0xf]), "0x0f");
/// assert_eq!(read_exact(&[0]), "0x00");
/// assert_eq!(read_exact(&[]), "0x");
/// ```
#[inline]
pub fn read_exact(bytes: &[u8]) -> String {
    let mut hex = String::from("0x");
    unprefixed::read_exact_into(bytes, &mut hex);
    hex
}

/// ```rust
/// # use w3b_types_core::hex::{write_exact, HexError};
///
/// assert_eq!(write_exact("0x07ff0f").unwrap(), &[0x7, 0xff, 0xf]);
/// assert_eq!(write_exact("0x3712").unwrap(), &[0x37, 0x12]);
/// assert_eq!(write_exact("0x00000f").unwrap(), &[0, 0, 0xf]);
/// assert_eq!(write_exact("0x0000").unwrap(), &[0, 0]);
/// assert_eq!(write_exact("0x").unwrap(), &[]);
///
/// assert_eq!(write_exact("12345").unwrap_err(), HexError::MissingPrefix);
/// assert_eq!(write_exact("0x001g").unwrap_err(), HexError::InvalidChar { char: 'g', index: 5 });
/// assert_eq!(write_exact("0x00000").unwrap_err(), HexError::InvalidOddLen { len: 7 });
/// ```
#[inline]
pub fn write_exact(hex: &str) -> Result<Vec<u8>, HexError> {
    let hex = strip_prefix(hex)?;

    if hex.len() & 1 == 0 {
        let mut bytes = vec![0; hex.len() >> 1];
        unprefixed::write_exact_into(hex, bytes.as_mut_slice()).map_err(shift_indices)?;
        Ok(bytes)
    } else {
        Err(HexError::InvalidOddLen { len: hex.len() + 2 })
    }
}

/// ```rust
/// # use w3b_types_core::hex::{write_left_padded, HexError};
///
/// assert_eq!(write_left_padded("0x7ff0f", 4).unwrap(), &[0, 0x7, 0xff, 0xf]);
/// assert_eq!(write_left_padded("0x3712", 3).unwrap(), &[0, 0x37, 0x12]);
/// assert_eq!(write_left_padded("0xf", 3).unwrap(), &[0, 0, 0xf]);
/// assert_eq!(write_left_padded("0x0000", 3).unwrap(), &[0, 0, 0]);
/// assert_eq!(write_left_padded("0x", 0).unwrap(), &[]);
///
/// assert_eq!(write_left_padded("0x", 1).unwrap_err(), HexError::NoDigits);
/// ```
#[inline]
pub fn write_left_padded(hex: &str, max_byte_len: usize) -> Result<Vec<u8>, HexError> {
    let hex = strip_prefix(hex)?;
    let mut bytes = vec![0; max_byte_len];

    unprefixed::write_left_padded_into(hex, max_byte_len, bytes.as_mut_slice())
        .map_err(shift_indices)?;

    Ok(bytes)
}

#[inline]
pub fn write_left_padded_into(
    hex: &str,
    max_byte_len: usize,
    bytes: &mut [u8],
) -> Result<(), HexError> {
    let hex = strip_prefix(hex)?;
    unprefixed::write_left_padded_into(hex, max_byte_len, bytes).map_err(shift_indices)
}

#[inline]
pub fn write_left_expanded_into(hex: &str, bytes: &mut [u8]) -> Result<(), HexError> {
    let hex = strip_prefix(hex)?;
    unprefixed::write_left_expanded_into(hex, bytes).map_err(shift_indices)
}

/// ```rust
/// # use w3b_types_core::hex::{write_exact_into, HexError};
/// assert_eq!(write_exact_into("0x00000", &mut [0, 0, 0]).unwrap_err(), HexError::IncorrectLen { len: 7, expected: 8 });
/// ```
#[inline]
pub fn write_exact_into(hex: &str, bytes: &mut [u8]) -> Result<(), HexError> {
    let hex = strip_prefix(hex)?;
    unprefixed::write_exact_into(hex, bytes).map_err(shift_indices)
}

#[inline]
fn strip_prefix(hex: &str) -> Result<&str, HexError> {
    if hex.starts_with("0x") {
        Ok(&hex[2..])
    } else {
        Err(HexError::MissingPrefix)
    }
}

fn shift_indices(error: HexError) -> HexError {
    use HexError::*;

    match error {
        InvalidChar { char, index } => InvalidChar {
            char,
            index: index + 2,
        },

        InvalidOddLen { len } => InvalidOddLen { len: len + 2 },

        IncorrectLen { len, expected } => IncorrectLen {
            len: len + 2,
            expected: expected + 2,
        },

        LenTooLong { len, max } => LenTooLong {
            len: len + 2,
            max: max + 2,
        },

        _ => error,
    }
}

pub mod unprefixed {
    use super::HEX_CHARS;
    use crate::hex::HexError;

    #[inline]
    pub fn read(bytes: &[u8]) -> String {
        let mut hex = String::new();
        read_into(bytes, &mut hex);
        hex
    }

    #[inline]
    pub fn read_left_padded(bytes: &[u8], len: usize) -> String {
        let mut hex = String::new();
        read_left_padded_into(bytes, len, &mut hex);
        hex
    }

    #[inline]
    pub fn read_right_padded(bytes: &[u8], len: usize) -> String {
        let mut hex = String::new();
        read_right_padded_into(bytes, len, &mut hex);
        hex
    }

    #[inline]
    pub fn read_exact(bytes: &[u8]) -> String {
        let mut hex = String::new();
        read_exact_into(bytes, &mut hex);
        hex
    }

    pub(super) fn read_into(mut bytes: &[u8], hex: &mut String) {
        while !bytes.is_empty() && bytes[0] == 0 {
            bytes = &bytes[1..];
        }

        if !bytes.is_empty() {
            if bytes[0] <= 0xf {
                unsafe {
                    hex.as_mut_vec().push(HEX_CHARS[bytes[0] as usize]);
                }

                bytes = &bytes[1..];
            }

            read_exact_into(bytes, hex);
        } else {
            hex.push('0');
        }
    }

    #[inline]
    pub fn read_left_padded_into(bytes: &[u8], max_byte_len: usize, hex: &mut String) {
        assert!(bytes.len() <= max_byte_len, "maximum byte length exceeded");
        pad_into(max_byte_len - bytes.len(), hex);
        read_exact_into(bytes, hex);
    }

    #[inline]
    pub fn read_right_padded_into(bytes: &[u8], max_byte_len: usize, hex: &mut String) {
        assert!(bytes.len() <= max_byte_len, "maximum byte length exceeded");
        read_exact_into(bytes, hex);
        pad_into(max_byte_len - bytes.len(), hex);
    }

    #[inline]
    pub fn read_exact_into(bytes: &[u8], hex: &mut String) {
        for byte in bytes {
            unsafe {
                hex.as_mut_vec().push(HEX_CHARS[(byte >> 4) as usize]);
                hex.as_mut_vec().push(HEX_CHARS[(byte & 0xf) as usize]);
            }
        }
    }

    #[inline]
    fn pad_into(byte_len: usize, hex: &mut String) {
        for _ in 0..byte_len {
            hex.push_str("00");
        }
    }

    pub fn write_left_padded_into(
        hex: &str,
        max_byte_len: usize,
        mut bytes: &mut [u8],
    ) -> Result<(), HexError> {
        assert!(max_byte_len <= bytes.len(), "output byte length exceeded");

        if max_byte_len > 0 && hex.is_empty() {
            return Err(HexError::NoDigits);
        }

        let padding_byte_len =
            max_byte_len
                .checked_sub(hex.len() + 1 >> 1)
                .ok_or(HexError::LenTooLong {
                    len: hex.len(),
                    max: max_byte_len << 1,
                })?;

        for _ in 0..padding_byte_len {
            bytes[0] = 0;
            bytes = &mut bytes[1..];
        }

        let mut byte = 0;
        let mut carry = hex.len() & 1 == 1;

        for (index, &char) in hex.as_bytes().iter().enumerate() {
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
                bytes[0] = byte;
                bytes = &mut bytes[1..];
            }

            carry = !carry;
        }

        Ok(())
    }

    #[inline]
    pub fn write_left_expanded_into(hex: &str, bytes: &mut [u8]) -> Result<(), HexError> {
        write_left_padded_into(hex, bytes.len(), bytes)
    }

    pub fn write_exact_into(hex: &str, bytes: &mut [u8]) -> Result<(), HexError> {
        let expected_hex_len = bytes.len() << 1;

        if hex.len() == expected_hex_len {
            write_left_expanded_into(hex, bytes)
        } else {
            Err(HexError::IncorrectLen {
                len: hex.len(),
                expected: expected_hex_len,
            })
        }
    }
}
