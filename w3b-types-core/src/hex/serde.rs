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

pub enum HexVisitor<'a> {
    Bytes(&'a mut Option<Vec<u8>>),
    Expanded(&'a mut [u8]),
    Exact(&'a mut [u8]),
}

impl<'a, 'de> Visitor<'de> for HexVisitor<'a> {
    type Value = ();

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use HexVisitor::*;

        write!(formatter, "a 0x-prefixed hexadecimal string")?;

        match self {
            Expanded(bytes) => write!(formatter, " with a length of at most {}", bytes.len() << 1),
            Exact(bytes) => write!(formatter, " with an exact length of {}", bytes.len() << 1),
            Bytes(_) => Ok(()),
        }
    }

    fn visit_str<E: de::Error>(mut self, v: &str) -> Result<Self::Value, E> {
        use HexVisitor::*;

        match &mut self {
            Expanded(bytes) => convert::write_expanded_into(v, *bytes),
            Exact(bytes) => convert::write_exact_into(v, *bytes),
            Bytes(maybe_bytes) => convert::write_exact(v).map(|bytes| **maybe_bytes = Some(bytes)),
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
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    let mut maybe_bytes = None;
    deserializer.deserialize_str(HexVisitor::Bytes(&mut maybe_bytes))?;
    Ok(maybe_bytes.unwrap())
}

#[inline]
pub fn deserialize_expanded<'de, D: Deserializer<'de>>(
    bytes: &mut [u8],
    deserializer: D,
) -> Result<(), D::Error> {
    deserializer.deserialize_str(HexVisitor::Expanded(bytes))
}

#[inline]
pub fn deserialize_exact<'de, D: Deserializer<'de>>(
    bytes: &mut [u8],
    deserializer: D,
) -> Result<(), D::Error> {
    deserializer.deserialize_str(HexVisitor::Exact(bytes))
}
