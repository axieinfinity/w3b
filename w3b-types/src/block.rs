use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use w3b_types_core::hex::HexVisitor;

use super::hex::Hex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockNumber {
    Earliest,
    Latest,
    Pending,
    Number(u64),
}

impl Default for BlockNumber {
    #[inline]
    fn default() -> Self {
        BlockNumber::Latest
    }
}

macro_rules! impl_from_num {
    ($num:ident) => {
        impl From<$num> for BlockNumber {
            #[inline]
            fn from(value: $num) -> Self {
                BlockNumber::Number(value as u64)
            }
        }
    };
}

impl_from_num!(u8);
impl_from_num!(u16);
impl_from_num!(u32);
impl_from_num!(u64);

impl Serialize for BlockNumber {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            BlockNumber::Earliest => serializer.serialize_str("earliest"),
            BlockNumber::Latest => serializer.serialize_str("latest"),
            BlockNumber::Pending => serializer.serialize_str("pending"),
            BlockNumber::Number(block_number) => Hex::new(*block_number).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for BlockNumber {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = BlockNumber;

            #[inline]
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "\"earliest\", \"latest\", \"pending\", or a block number in hexadecimal",
                )
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "earliest" => BlockNumber::Earliest,
                    "latest" => BlockNumber::Latest,
                    "pending" => BlockNumber::Pending,
                    _ => {
                        let mut repr = [0; std::mem::size_of::<u64>()];
                        let visitor = HexVisitor::Expanded(&mut repr);
                        visitor.visit_str(v)?;
                        BlockNumber::Number(u64::from_be_bytes(repr))
                    }
                })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::BlockNumber;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&BlockNumber::Latest).unwrap(),
            "\"latest\"",
        );

        assert_eq!(
            serde_json::to_string(&BlockNumber::Number(0x1_000_000_000_000)).unwrap(),
            "\"0x1000000000000\"",
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<BlockNumber>("\"latest\"").unwrap(),
            BlockNumber::Latest,
        );

        assert_eq!(
            serde_json::from_str::<BlockNumber>("\"0x1000000000000\"").unwrap(),
            BlockNumber::Number(0x1_000_000_000_000),
        );
    }
}
