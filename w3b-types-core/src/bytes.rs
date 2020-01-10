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

            pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, $crate::TypeError> {
                let bytes = bytes.as_ref();

                if bytes.len() <= Self::NUM_BYTES {
                    let mut repr = [0; Self::NUM_BYTES];
                    repr.as_mut()[Self::NUM_BYTES - bytes.len()..].copy_from_slice(bytes);
                    Ok(Self(repr))
                } else {
                    Err($crate::TypeError::SliceTooLong {
                        len: bytes.len(),
                        max: Self::NUM_BYTES,
                    })
                }
            }

            #[inline]
            pub fn from_hex(hex: impl AsRef<str>) -> Result<Self, $crate::hex::HexError> {
                let mut repr = [0; Self::NUM_BYTES];
                $crate::hex::from_hex(hex, true, &mut repr)?;
                Ok(Self(repr))
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
                $crate::hex::to_hex(self.as_bytes(), true)
            }
        }

        impl ::std::fmt::LowerHex for $bytes {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let skip_prefix = (1 - f.alternate() as usize) << 1;
                write!(f, "{}", &self.to_hex()[skip_prefix..])
            }
        }

        impl ::std::fmt::UpperHex for $bytes {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let prefix = "0x".repeat(f.alternate() as usize);
                write!(f, "{}{}", prefix, &self.to_hex().to_uppercase()[2..])
            }
        }

        impl $crate::serde::Serialize for $bytes {
            #[inline]
            fn serialize<S: $crate::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                $crate::hex::serialize_fixed_len(&self.0, serializer)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $bytes {
            #[inline]
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let mut repr = [0; Self::NUM_BYTES];
                $crate::hex::deserialize_fixed_len(&mut repr, deserializer)?;
                Ok(Self(repr))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use crate::impl_bytes;

    impl_bytes!(Bytes3; size = 3);

    #[test]
    fn from_hex() {
        let bytes3 = Bytes3::from_hex("0x007799").unwrap();
        assert_eq!(bytes3.as_bytes(), &[0, 0x77, 0x99]);
    }
}
