use super::error::HexError;

const HEX_CHARS: &'static [u8] = b"0123456789abcdef";

/// Converts a slice of bytes to a hexadecimal string.
///
/// ```rust
/// use w3b_types_core::hex::to_hex;
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

/// Converts a hexadecimal string to a `Vec` of bytes.
///
/// ```rust
/// use w3b_types_core::hex::{from_hex, HexError};
///
/// fn assert_bytes(hex: &str, expected_bytes: &[u8]) {
///   let mut bytes = [0; 10];
///   let len = from_hex(hex, false, &mut bytes).unwrap();
///   assert_eq!(&bytes[bytes.len() - len..], expected_bytes);
/// }
///
/// assert_bytes("0x7ff0f", &[0x7, 0xff, 0xf]);
/// assert_bytes("0x3712", &[0x37, 0x12]);
/// assert_bytes("0xf", &[0xf]);
///
/// assert_bytes("0x07ff0f", &[0x7, 0xff, 0xf]);
/// assert_bytes("0x003712", &[0x37, 0x12]);
/// assert_bytes("0x00000f", &[0xf]);
/// assert_bytes("0x0000", &[0x0]);
///
/// fn assert_error(hex: &str, exact_len: bool, expected_error: HexError) {
///   let mut bytes = [0; 3];
///   let error = from_hex(hex, exact_len, &mut bytes).unwrap_err();
///   assert_eq!(error, expected_error);
/// }
///
/// assert_error("12345", false, HexError::MissingPrefix);
/// assert_error("0x001g", false, HexError::InvalidChar { ch: 'g', index: 5 });
/// assert_error("0x00000", true, HexError::IncorrectLen { len: 7, expected: 8 });
/// assert_error("0x0000000", false, HexError::LenTooLong { len: 9, max: 8 });
/// ```
pub fn from_hex<'a>(
    hex: impl AsRef<str>,
    exact_len: bool,
    mut bytes: impl AsMut<[u8]>,
) -> Result<usize, HexError> {
    let hex = hex.as_ref();
    let bytes = bytes.as_mut();

    if !hex.starts_with("0x") {
        return Err(HexError::MissingPrefix);
    }

    let len = hex.len();
    let max_len = (bytes.len() << 1) + 2;

    if exact_len && len != max_len {
        return Err(HexError::IncorrectLen {
            len,
            expected: max_len,
        });
    }

    if len > max_len {
        return Err(HexError::LenTooLong { len, max: max_len });
    }

    let hex = hex.as_bytes();
    let mut index = 2;

    while index < hex.len() && hex[index] == b'0' {
        index += 1;
    }

    let mut position = bytes.len();
    let mut carry = false;

    for (index, &ch) in hex.iter().enumerate().skip(index).rev() {
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

    if position == bytes.len() {
        position -= 1;
    }

    Ok(bytes.len() - position)
}
