use std::fmt;

use serde::{de, de::Visitor, Deserializer, Serializer};

use super::{convert, error::HexError};

#[inline]
pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&convert::read(bytes))
}

#[inline]
pub fn serialize_exact<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&convert::read_exact(bytes))
}

pub struct HexVisitor<'a> {
    exact: bool,
    bytes: &'a mut [u8],
}

impl<'a> HexVisitor<'a> {
    #[inline]
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self {
            exact: false,
            bytes,
        }
    }

    #[inline]
    pub fn exact(bytes: &'a mut [u8]) -> Self {
        Self { exact: true, bytes }
    }
}

impl<'a, 'de> Visitor<'de> for HexVisitor<'a> {
    type Value = ();

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a 0x-prefixed hexadecimal string with {} {}",
            if self.exact {
                format!("a fixed length of")
            } else {
                format!("a length of at most")
            },
            self.bytes.len() << 1,
        )
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        if self.exact {
            convert::write_exact_into(v, self.bytes)
        } else {
            convert::write_expanded_into(v, self.bytes)
        }
        .map_err(|error| match error {
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
    deserializer.deserialize_str(HexVisitor::exact(bytes.as_mut()))
}
