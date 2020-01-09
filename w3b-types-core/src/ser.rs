use std::{array::LengthAtMost32, convert::TryInto, fmt};

use serde::{de, Deserializer, Serializer};

use super::hex::{self, ExpectedHexLen, HexError};

#[inline]
pub fn serialize_fixed_bytes<S: Serializer>(
    bytes: &[u8],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&hex::to_hex(bytes, false))
}

#[inline]
pub fn serialize_numeric<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&hex::to_hex(bytes, true))
}

struct Visitor(Option<ExpectedHexLen>);

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Vec<u8>;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0.as_ref() {
            Some(expected_len) => write!(
                formatter,
                "a 0x-prefixed hexadecimal string with {}",
                expected_len,
            ),

            None => write!(formatter, "a 0x-prefixed hexadecimal string"),
        }
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        hex::from_hex(v, self.0.as_ref()).map_err(|error| match error {
            HexError::MissingPrefix | HexError::InvalidChar { .. } => E::custom(error),
            HexError::IncorrectLen { len, .. } | HexError::LenOverflow { len, .. } => {
                E::invalid_length(len, &self)
            }
        })
    }
}

#[inline]
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    deserializer.deserialize_str(Visitor(None))
}

#[inline]
pub fn deserialize_exact_size<'de, D, const NUM_BYTES: usize>(
    deserializer: D,
) -> Result<[u8; NUM_BYTES], D::Error>
where
    D: Deserializer<'de>,
    [u8; NUM_BYTES]: LengthAtMost32,
{
    deserializer
        .deserialize_str(Visitor(Some(ExpectedHexLen::Exact((NUM_BYTES << 1) + 2))))
        .map(|bytes| bytes.as_slice().try_into().unwrap())
}

#[inline]
pub fn deserialize_at_most_size<'de, D, const NUM_BYTES: usize>(
    deserializer: D,
) -> Result<[u8; NUM_BYTES], D::Error>
where
    D: Deserializer<'de>,
    [u8; NUM_BYTES]: LengthAtMost32,
{
    deserializer
        .deserialize_str(Visitor(Some(ExpectedHexLen::AtMost((NUM_BYTES << 1) + 2))))
        .map(|bytes| bytes.as_slice().try_into().unwrap())
}
