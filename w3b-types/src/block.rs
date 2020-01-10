use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use w3b_types_core::hex::{self, HexVisitor};

#[derive(PartialEq, Eq, Debug)]
pub struct BlockNumber(u64);

impl Serialize for BlockNumber {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        hex::serialize(self.0.to_be_bytes().as_ref(), serializer)
    }
}

impl<'de> Deserialize<'de> for BlockNumber {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut bytes = [0; 8];
        hex::deserialize(&mut bytes, deserializer)?;
        Ok(Self(u64::from_be_bytes(bytes)))
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
                    _ => {
                        let mut bytes = [0; 8];
                        let visitor = HexVisitor::new(&mut bytes);
                        visitor.visit_str(v)?;
                        Ok(BlockId::Number(BlockNumber(u64::from_be_bytes(bytes))))
                    }
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockId, BlockNumber};

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&BlockNumber(0x6789)).unwrap(),
            "\"0x6789\"",
        );

        assert_eq!(
            serde_json::to_string(&BlockId::Latest).unwrap(),
            "\"latest\"",
        );

        assert_eq!(
            serde_json::to_string(&BlockId::Number(BlockNumber(0x1_000_000_000_000))).unwrap(),
            "\"0x1000000000000\"",
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<BlockNumber>("\"0x6789\"").unwrap(),
            BlockNumber(0x6789),
        );

        assert_eq!(
            serde_json::from_str::<BlockId>("\"latest\"").unwrap(),
            BlockId::Latest,
        );

        assert_eq!(
            serde_json::from_str::<BlockId>("\"0x1000000000000\"").unwrap(),
            BlockId::Number(BlockNumber(0x1_000_000_000_000)),
        );
    }
}
