use std::fmt;

use num_bigint::BigUint;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use w3b_types_core::{
    hex::{self, HexError},
    ser,
};

#[derive(PartialEq, Eq, Debug)]
pub struct BlockNumber(BigUint);

impl Serialize for BlockNumber {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ser::serialize_numeric(self.0.to_bytes_be().as_slice(), serializer)
    }
}

impl<'de> Deserialize<'de> for BlockNumber {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ser::deserialize(deserializer).map(|bytes| Self(BigUint::from_bytes_be(bytes.as_slice())))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum BlockId {
    Earliest,
    Latest,
    Pending,
    Number(BlockNumber),
}

impl Serialize for BlockId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            BlockId::Earliest => serializer.serialize_str("earliest"),
            BlockId::Latest => serializer.serialize_str("latest"),
            BlockId::Pending => serializer.serialize_str("pending"),
            BlockId::Number(block_number) => block_number.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for BlockId {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = BlockId;

            #[inline]
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "\"earliest\", \"latest\", \"pending\", or a block number in hexadecimal",
                )
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match v {
                    "earliest" => Ok(BlockId::Earliest),
                    "latest" => Ok(BlockId::Latest),
                    "pending" => Ok(BlockId::Pending),
                    _ => hex::from_hex(v, None)
                        .map_err(|error| match error {
                            HexError::MissingPrefix | HexError::InvalidChar { .. } => {
                                E::custom(error)
                            }
                            HexError::IncorrectLen { len, .. }
                            | HexError::LenOverflow { len, .. } => E::invalid_length(len, &self),
                        })
                        .map(|bytes| {
                            BlockId::Number(BlockNumber(BigUint::from_bytes_be(bytes.as_slice())))
                        }),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::{BlockId, BlockNumber};

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&BlockNumber(BigUint::from(0x6789_u16))).unwrap(),
            "\"0x6789\"",
        );

        assert_eq!(
            serde_json::to_string(&BlockId::Latest).unwrap(),
            "\"latest\"",
        );

        assert_eq!(
            serde_json::to_string(&BlockId::Number(BlockNumber(BigUint::from(
                0x1_000_000_000_000_u64
            ))))
            .unwrap(),
            "\"0x1000000000000\"",
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<BlockNumber>("\"0x6789\"").unwrap(),
            BlockNumber(BigUint::from(0x6789_u16)),
        );

        assert_eq!(
            serde_json::from_str::<BlockId>("\"latest\"").unwrap(),
            BlockId::Latest,
        );

        assert_eq!(
            serde_json::from_str::<BlockId>("\"0x1000000000000\"").unwrap(),
            BlockId::Number(BlockNumber(BigUint::from(0x1_000_000_000_000_u64))),
        );
    }
}
