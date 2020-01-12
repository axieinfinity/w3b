use std::fmt;

use w3b_types_core::{
    hex,
    hex::HexError,
    impl_bytes,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    #[inline]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    #[inline]
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Self {
        Self(bytes.as_ref().into())
    }

    #[inline]
    pub fn from_hex(hex: impl AsRef<str>) -> Result<Self, HexError> {
        hex::write_exact(hex.as_ref()).map(Self)
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    #[inline]
    pub fn to_hex(&self) -> String {
        hex::read_exact(self.as_bytes())
    }

    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl fmt::LowerHex for Bytes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix_skip = (1 - f.alternate() as usize) << 1;
        write!(f, "{}", &self.to_hex()[prefix_skip..])
    }
}

impl fmt::UpperHex for Bytes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = "0x".repeat(f.alternate() as usize);
        write!(f, "{}{}", prefix, &self.to_hex().to_uppercase()[2..])
    }
}

impl Serialize for Bytes {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        hex::serialize_exact(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for Bytes {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        hex::deserialize(deserializer).map(Self)
    }
}

pub type Byte = Bytes1;

impl_bytes!(Bytes1; size = 1);
impl_bytes!(Bytes2; size = 2);
impl_bytes!(Bytes3; size = 3);
impl_bytes!(Bytes4; size = 4);
impl_bytes!(Bytes5; size = 5);
impl_bytes!(Bytes6; size = 6);
impl_bytes!(Bytes7; size = 7);
impl_bytes!(Bytes8; size = 8);
impl_bytes!(Bytes9; size = 9);
impl_bytes!(Bytes10; size = 10);
impl_bytes!(Bytes11; size = 11);
impl_bytes!(Bytes12; size = 12);
impl_bytes!(Bytes13; size = 13);
impl_bytes!(Bytes14; size = 14);
impl_bytes!(Bytes15; size = 15);
impl_bytes!(Bytes16; size = 16);
impl_bytes!(Bytes17; size = 17);
impl_bytes!(Bytes18; size = 18);
impl_bytes!(Bytes19; size = 19);
impl_bytes!(Bytes20; size = 20);
impl_bytes!(Bytes21; size = 21);
impl_bytes!(Bytes22; size = 22);
impl_bytes!(Bytes23; size = 23);
impl_bytes!(Bytes24; size = 24);
impl_bytes!(Bytes25; size = 25);
impl_bytes!(Bytes26; size = 26);
impl_bytes!(Bytes27; size = 27);
impl_bytes!(Bytes28; size = 28);
impl_bytes!(Bytes29; size = 29);
impl_bytes!(Bytes30; size = 30);
impl_bytes!(Bytes31; size = 31);
impl_bytes!(Bytes32; size = 32);
