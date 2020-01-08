use std::{array::LengthAtMost32, convert::TryInto, fmt};

use serde::{de, Deserializer, Serializer};

use super::hex;

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

enum ExpectedSize {
    Exact(usize),
    AtMost(usize),
}

impl ExpectedSize {
    fn matches(&self, size: usize) -> bool {
        match self {
            ExpectedSize::Exact(exact) => size == *exact,
            ExpectedSize::AtMost(at_most) => size <= *at_most,
        }
    }
}

impl fmt::Display for ExpectedSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpectedSize::Exact(len) => write!(f, "size of {} in bytes", len),
            ExpectedSize::AtMost(len) => write!(f, "size of at most {} in bytes", len),
        }
    }
}

struct Visitor(ExpectedSize);

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Vec<u8>;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a 0x-prefixed hexadecimal string with {}", self.0)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        hex::from_hex(v).map_err(E::custom).and_then(|bytes| {
            if self.0.matches(bytes.len()) {
                Ok(bytes)
            } else {
                Err(E::invalid_length((v.len() - 1) / 2, &self))
            }
        })
    }
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
        .deserialize_str(Visitor(ExpectedSize::Exact(NUM_BYTES)))
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
        .deserialize_str(Visitor(ExpectedSize::AtMost(NUM_BYTES)))
        .map(|bytes| bytes.as_slice().try_into().unwrap())
}
