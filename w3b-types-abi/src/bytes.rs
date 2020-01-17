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
    pub fn from_hex_unprefixed(hex: impl AsRef<str>) -> Result<Self, HexError> {
        hex::unprefixed::write_exact(hex.as_ref()).map(Self)
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
    pub fn to_hex_unprefixed(&self) -> String {
        hex::unprefixed::read_exact(self.as_bytes())
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
        hex::deserialize_unbounded(deserializer).map(Self)
    }
}

macro_rules! impl_bytes_ext {
    ($bytes:ident; $($tail:tt)*) => {
        impl_bytes!($bytes; $($tail)*);

        impl From<$bytes> for Bytes {
            #[inline]
            fn from(value: $bytes) -> Self {
                Self::from_bytes(value.as_bytes())
            }
        }
    };
}

pub type Byte = Bytes1;

impl_bytes_ext!(Bytes1; size = 1);
impl_bytes_ext!(Bytes2; size = 2);
impl_bytes_ext!(Bytes3; size = 3);
impl_bytes_ext!(Bytes4; size = 4);
impl_bytes_ext!(Bytes5; size = 5);
impl_bytes_ext!(Bytes6; size = 6);
impl_bytes_ext!(Bytes7; size = 7);
impl_bytes_ext!(Bytes8; size = 8);
impl_bytes_ext!(Bytes9; size = 9);
impl_bytes_ext!(Bytes10; size = 10);
impl_bytes_ext!(Bytes11; size = 11);
impl_bytes_ext!(Bytes12; size = 12);
impl_bytes_ext!(Bytes13; size = 13);
impl_bytes_ext!(Bytes14; size = 14);
impl_bytes_ext!(Bytes15; size = 15);
impl_bytes_ext!(Bytes16; size = 16);
impl_bytes_ext!(Bytes17; size = 17);
impl_bytes_ext!(Bytes18; size = 18);
impl_bytes_ext!(Bytes19; size = 19);
impl_bytes_ext!(Bytes20; size = 20);
impl_bytes_ext!(Bytes21; size = 21);
impl_bytes_ext!(Bytes22; size = 22);
impl_bytes_ext!(Bytes23; size = 23);
impl_bytes_ext!(Bytes24; size = 24);
impl_bytes_ext!(Bytes25; size = 25);
impl_bytes_ext!(Bytes26; size = 26);
impl_bytes_ext!(Bytes27; size = 27);
impl_bytes_ext!(Bytes28; size = 28);
impl_bytes_ext!(Bytes29; size = 29);
impl_bytes_ext!(Bytes30; size = 30);
impl_bytes_ext!(Bytes31; size = 31);
impl_bytes_ext!(Bytes32; size = 32);
