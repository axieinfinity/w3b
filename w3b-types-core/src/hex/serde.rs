use std::fmt;

use serde::{de, de::Visitor, Deserializer, Serializer};

use super::{convert, error::HexError};

#[inline]
pub fn serialize<B: AsRef<[u8]>, S: Serializer>(
    bytes: B,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&convert::to_hex(bytes, false))
}

#[inline]
pub fn serialize_fixed_len<B: AsRef<[u8]>, S: Serializer>(
    bytes: B,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&convert::to_hex(bytes, true))
}

pub struct HexVisitor<'a> {
    fixed_len: bool,
    out: &'a mut [u8],
}

impl<'a> HexVisitor<'a> {
    #[inline]
    pub fn new(out: &'a mut [u8]) -> Self {
        Self {
            fixed_len: false,
            out,
        }
    }

    #[inline]
    pub fn fixed_len(out: &'a mut [u8]) -> Self {
        Self {
            fixed_len: true,
            out,
        }
    }
}

impl<'a, 'de> Visitor<'de> for HexVisitor<'a> {
    type Value = ();

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a 0x-prefixed hexadecimal string with {} {}",
            if self.fixed_len {
                format!("a fixed length of")
            } else {
                format!("a length of at most")
            },
            convert::hex_len(self.out.len()),
        )
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        convert::from_hex(v, self.fixed_len, &mut *self.out).map_err(|error| match error {
            HexError::IncorrectLen { len, .. } | HexError::LenTooLong { len, .. } => {
                E::invalid_length(len, &self)
            }

            _ => E::custom(error),
        })
    }
}

#[inline]
pub fn deserialize<'de, B: AsMut<[u8]>, D: Deserializer<'de>>(
    mut bytes: B,
    deserializer: D,
) -> Result<(), D::Error> {
    deserializer.deserialize_str(HexVisitor::new(bytes.as_mut()))
}

#[inline]
pub fn deserialize_fixed_len<'de, B: AsMut<[u8]>, D: Deserializer<'de>>(
    mut bytes: B,
    deserializer: D,
) -> Result<(), D::Error> {
    deserializer.deserialize_str(HexVisitor::fixed_len(bytes.as_mut()))
}
