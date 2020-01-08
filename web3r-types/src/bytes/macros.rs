#[macro_export]
macro_rules! impl_bytes {
    ($bytes:ident; size = $n_bytes:literal) => {
        pub struct $bytes([u8; Self::NUM_BYTES]);

        impl $bytes {
            pub const NUM_BYTES: usize = $n_bytes;

            pub fn new(repr: [u8; Self::NUM_BYTES]) -> Self {
                Self(repr)
            }

            pub fn from_bytes(bytes: &[u8]) -> Result<Self, ::std::array::TryFromSliceError> {
                <[u8; Self::NUM_BYTES] as ::std::convert::TryFrom<&[u8]>>::try_from(bytes).map(Self)
            }

            pub fn repr(&self) -> &[u8; Self::NUM_BYTES] {
                &self.0
            }

            pub fn bytes(&self) -> &[u8] {
                self.repr().as_ref()
            }
        }

        impl $crate::serde::Serialize for $bytes {
            fn serialize<S: $crate::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                $crate::internal::ser::serialize_fixed_bytes(&self.0, serializer)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $bytes {
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                $crate::internal::ser::deserialize_exact_size(deserializer).map(Self)
            }
        }
    };
}
