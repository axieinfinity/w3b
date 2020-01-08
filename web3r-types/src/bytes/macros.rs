#[macro_export]
macro_rules! impl_bytes {
    ($bytes:ident; size = $n_bytes:literal) => {
        #[derive(PartialEq, Eq, Debug)]
        pub struct $bytes([u8; Self::NUM_BYTES]);

        impl $bytes {
            pub const NUM_BYTES: usize = $n_bytes;

            #[inline]
            pub fn new(repr: [u8; Self::NUM_BYTES]) -> Self {
                Self(repr)
            }

            #[inline]
            pub fn from_bytes(
                bytes: impl AsRef<[u8]>,
            ) -> Result<Self, ::std::array::TryFromSliceError> {
                <[u8; Self::NUM_BYTES] as ::std::convert::TryFrom<&[u8]>>::try_from(bytes.as_ref())
                    .map(Self)
            }

            #[inline]
            pub fn from_hex(hex: impl AsRef<str>) -> Result<Self, $crate::HexError> {
                $crate::internal::hex::from_hex(
                    hex,
                    &$crate::internal::hex::ExpectedHexLen::Exact((Self::NUM_BYTES << 1) + 2),
                )
                .map(|bytes| Self::from_bytes(bytes.as_slice()).unwrap())
            }

            #[inline]
            pub fn as_repr(&self) -> &[u8; Self::NUM_BYTES] {
                &self.0
            }

            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                self.as_repr().as_ref()
            }

            #[inline]
            pub fn to_hex(&self) -> String {
                $crate::internal::hex::to_hex(self.as_bytes(), false)
            }
        }

        impl ::std::fmt::LowerHex for $bytes {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if f.alternate() {
                    write!(f, "0x")?;
                }

                write!(f, "{}", &self.to_hex()[2..])
            }
        }

        impl ::std::fmt::UpperHex for $bytes {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if f.alternate() {
                    write!(f, "0x")?;
                }

                write!(f, "{}", &self.to_hex().to_uppercase()[2..])
            }
        }

        impl $crate::serde::Serialize for $bytes {
            #[inline]
            fn serialize<S: $crate::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                $crate::internal::ser::serialize_fixed_bytes(&self.0, serializer)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $bytes {
            #[inline]
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                $crate::internal::ser::deserialize_exact_size(deserializer).map(Self)
            }
        }
    };
}
