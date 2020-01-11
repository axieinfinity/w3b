use super::error::HexError;

const HEX_CHARS: &'static [u8] = b"0123456789abcdef";

pub const fn hex_len(num_bytes: usize) -> usize {
    // 2 hexadecimal digits for each byte and additional 2 for the 0x prefix
    (num_bytes << 1) + 2
}

/// Converts a slice of bytes to a hexadecimal string.
///
/// ```rust
/// use w3b_types_core::hex::to_hex;
///
/// assert_eq!(&to_hex(&[0xf, 0xff, 0xf], false), "0xfff0f");
/// assert_eq!(&to_hex(&[0, 0, 0x37], false), "0x37");
/// assert_eq!(&to_hex(&[0, 0, 0xf], false), "0xf");
/// assert_eq!(&to_hex(&[0x1], false), "0x1");
///
/// assert_eq!(&to_hex(&[0xf, 0xff, 0xf], true), "0x0fff0f");
/// assert_eq!(&to_hex(&[0, 0, 0x37], true), "0x000037");
/// assert_eq!(&to_hex(&[0, 0, 0xf], true), "0x00000f");
/// assert_eq!(&to_hex(&[0x1], true), "0x01");
/// ```
pub fn to_hex(bytes: impl AsRef<[u8]>, fixed_len: bool) -> String {
    let bytes = bytes.as_ref();
    let mut index = 0;

    if !fixed_len {
        while index < bytes.len() && bytes[index] == 0 {
            index += 1
        }
    }

    if index >= bytes.len() {
        return "0x0".to_owned();
    }

    let mut out = String::with_capacity(hex_len(bytes.len()));
    out.push_str("0x");

    if !fixed_len && bytes[index] < 0x10 {
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

/// Converts a hexadecimal string to a `Vec` of bytes.
///
/// ```rust
/// use w3b_types_core::hex::{from_hex, HexError};
///
/// fn assert_bytes(hex: &str, expected_bytes: &[u8]) {
///   let mut bytes = [0; 3];
///   from_hex(hex, false, &mut bytes).unwrap();
///   assert_eq!(&bytes, expected_bytes);
/// }
///
/// assert_bytes("0x07ff0f", &[0x7, 0xff, 0xf]);
/// assert_bytes("0x3712", &[0, 0x37, 0x12]);
/// assert_bytes("0x0000f", &[0, 0, 0xf]);
/// assert_bytes("0x0000", &[0, 0, 0]);
///
/// fn assert_error(hex: &str, exact_len: bool, expected_error: HexError) {
///   let mut bytes = [0; 3];
///   let error = from_hex(hex, exact_len, &mut bytes).unwrap_err();
///   assert_eq!(error, expected_error);
/// }
///
/// assert_error("12345", false, HexError::MissingPrefix);
/// assert_error("0x", false, HexError::NoDigits);
/// assert_error("0x001g", false, HexError::InvalidChar { ch: 'g', index: 5 });
/// assert_error("0x00000", true, HexError::IncorrectLen { len: 7, expected: 8 });
/// assert_error("0x0000000", false, HexError::LenTooLong { len: 9, max: 8 });
/// ```
pub fn from_hex<'a>(
    hex: impl AsRef<str>,
    fixed_len: bool,
    mut bytes: impl AsMut<[u8]>,
) -> Result<(), HexError> {
    let hex = hex.as_ref();
    let bytes = bytes.as_mut();

    if !hex.starts_with("0x") {
        return Err(HexError::MissingPrefix);
    }

    if hex.len() <= 2 {
        return Err(HexError::NoDigits);
    }

    let len = hex.len();
    let max_len = hex_len(bytes.len());

    if fixed_len && len != max_len {
        return Err(HexError::IncorrectLen {
            len,
            expected: max_len,
        });
    }

    if len > max_len {
        return Err(HexError::LenTooLong { len, max: max_len });
    }

    let mut position = bytes.len();
    let mut carry = false;

    for (index, &ch) in hex.as_bytes().iter().enumerate().skip(2).rev() {
        let nibble = match ch {
            b'A'..=b'F' => ch - b'A' + 10,
            b'a'..=b'f' => ch - b'a' + 10,
            b'0'..=b'9' => ch - b'0',

            _ => {
                return Err(HexError::InvalidChar {
                    ch: ch.into(),
                    index,
                });
            }
        };

        if carry {
            bytes[position] |= nibble << 4;
            carry = false;
        } else {
            position -= 1;
            bytes[position] = nibble;
            carry = true;
        }
    }

    Ok(())
}
