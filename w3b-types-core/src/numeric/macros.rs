#[macro_export]
macro_rules! impl_num {
    ($num:ident; @int, size = $n_bytes:literal $($tail:tt)*) => {
        impl_num!(@common $num, $n_bytes);
        impl_num!(@int $num);
        impl_num!($num $($tail)*);
    };

    ($num:ident; @uint, size = $n_bytes:literal $($tail:tt)*) => {
        impl_num!(@common $num, $n_bytes);
        impl_num!(@uint $num);
        impl_num!($num $($tail)*);
    };

    ($num:ident; @gt $($primitive:ty),*; $($tail:tt)*) => {
        impl_num!($num; @gt $($primitive),*);
        impl_num!($num; $($tail)*);
    };

    ($num:ident; @gt $($primitive:ty),*) => {
        $(
            impl_num!(@impl TryFrom<$num> for $primitive);
            impl_num!(@impl From<$primitive> for $num);
        )*
    };

    ($num:ident; @eq $($primitive:ty),*; $($tail:tt)*) => {
        impl_num!($num; @eq $($primitive),*);
        impl_num!($num; $($tail)*);
    };

    ($num:ident; @eq $($primitive:ty),*) => {
        $(
            impl_num!(@impl From<$num> for $primitive);
            impl_num!(@impl From<$primitive> for $num);
        )*
    };

    ($num:ident; @lt $($primitive:ty),*; $($tail:tt)*) => {
        impl_num!($num; @lt $($primitive),*);
        impl_num!($num; $($tail)*);
    };

    ($num:ident; @lt $($primitive:ty),*) => {
        $(
            impl_num!(@impl From<$num> for $primitive);
            impl_num!(@impl TryFrom<$primitive> for $num);
        )*
    };

    ($num:ident $(;)?) => {};

    (@common $num:ident, $n_bytes:literal) => {
        #[derive(Clone, PartialEq, Eq, Debug)]
        pub struct $num([u8; Self::NUM_BYTES]);

        impl $num {
            pub const NUM_BYTES: usize = $n_bytes;

            #[inline]
            pub fn new(repr: [u8; Self::NUM_BYTES]) -> Self {
                Self(repr)
            }

            #[inline]
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
                $crate::hex::write_left_expanded_into(hex.as_ref(), &mut repr)?;
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
                $crate::hex::read(self.as_bytes())
            }
        }

        impl ::std::fmt::LowerHex for $num {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let prefix_skip = (1 - f.alternate() as usize) << 1;
                write!(f, "{}", &self.to_hex()[prefix_skip..])
            }
        }

        impl ::std::fmt::UpperHex for $num {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let prefix = "0x".repeat(f.alternate() as usize);
                write!(f, "{}{}", prefix, &self.to_hex().to_uppercase()[2..])
            }
        }

        impl From<$num> for $crate::num_bigint::BigInt {
            #[inline]
            fn from(value: $num) -> Self {
                Self::from_bytes_be($crate::num_bigint::Sign::Plus, value.as_bytes())
            }
        }

        impl From<$num> for $crate::num_bigint::BigUint {
            #[inline]
            fn from(value: $num) -> Self {
                Self::from_bytes_be(value.as_bytes())
            }
        }

        impl $crate::serde::Serialize for $num {
            #[inline]
            fn serialize<S: $crate::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                $crate::hex::serialize(&self.0, serializer)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $num {
            #[inline]
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let mut repr = [0; Self::NUM_BYTES];
                $crate::hex::deserialize_expanded(&mut repr, deserializer)?;
                Ok(Self(repr))
            }
        }
    };

    (@int $num:ident) => {
        impl ::std::convert::TryFrom<$crate::num_bigint::BigInt> for $num {
            type Error = $crate::numeric::NumCastError;

            fn try_from(value: $crate::num_bigint::BigInt) -> Result<Self, Self::Error> {
                Self::from_bytes(value.to_signed_bytes_be())
                    .map_err(|_| Self::Error::new(value, stringify!($num)))
            }
        }

        impl ::std::convert::TryFrom<$crate::num_bigint::BigUint> for $num {
            type Error = $crate::numeric::NumCastError;

            #[inline]
            fn try_from(value: $crate::num_bigint::BigUint) -> Result<Self, Self::Error> {
                use ::std::convert::TryInto;
                $crate::num_bigint::BigInt::from(value).try_into()
            }
        }
    };

    (@uint $num:ident) => {
        impl ::std::convert::TryFrom<$crate::num_bigint::BigUint> for $num {
            type Error = $crate::numeric::NumCastError;

            fn try_from(value: $crate::num_bigint::BigUint) -> Result<Self, Self::Error> {
                Self::from_bytes(value.to_bytes_be())
                    .map_err(|_| Self::Error::new(value.into(), stringify!($num)))
            }
        }

        impl ::std::convert::TryFrom<$crate::num_bigint::BigInt> for $num {
            type Error = $crate::numeric::NumCastError;

            fn try_from(value: $crate::num_bigint::BigInt) -> Result<Self, Self::Error> {
                use ::std::convert::TryInto;

                match value.to_biguint() {
                    Some(value) => value.try_into(),
                    None => Err(Self::Error::new(value, stringify!($num))),
                }
            }
        }
    };

    (@impl From<$num:ident> for $primitive:ty) => {
        impl From<$num> for $primitive {
            #[inline]
            fn from(value: $num) -> Self {
                let mut repr = [0; ::std::mem::size_of::<$primitive>()];

                repr.as_mut()[::std::mem::size_of::<$primitive>() - $num::NUM_BYTES..]
                    .copy_from_slice(value.as_bytes());

                <$primitive>::from_be_bytes(repr)
            }
        }
    };

    (@impl From<$primitive:ty> for $num:ident) => {
        impl From<$primitive> for $num {
            #[inline]
            fn from(value: $primitive) -> Self {
                Self::from_bytes(value.to_be_bytes().as_ref()).unwrap()
            }
        }
    };

    (@impl TryFrom<$num:ident> for $primitive:ty) => {
        impl ::std::convert::TryFrom<$num> for $primitive {
            type Error = $crate::numeric::NumCastError;

            fn try_from(value: $num) -> Result<Self, Self::Error> {
                let mut bytes = value.as_bytes();

                while !bytes.is_empty() && bytes[0] == 0 {
                    bytes = &bytes[1..];
                }

                if bytes.len() <= ::std::mem::size_of::<$primitive>() {
                    let mut repr = [0; ::std::mem::size_of::<$primitive>()];

                    repr.as_mut()[::std::mem::size_of::<$primitive>() - bytes.len()..]
                        .copy_from_slice(bytes);

                    Ok(<$primitive>::from_be_bytes(repr))
                } else {
                    Err(Self::Error::new(value.into(), stringify!($primitive)))
                }
            }
        }
    };

    (@impl TryFrom<$primitive:ty> for $num:ident) => {
        impl ::std::convert::TryFrom<$primitive> for $num {
            type Error = $crate::numeric::NumCastError;

            #[inline]
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                Self::from_bytes(value.to_be_bytes().as_ref())
                    .map_err(|_| Self::Error::new(value.into(), stringify!($num)))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use num_bigint::BigUint;

    use crate::impl_num;

    impl_num!(Uint8; @uint, size = 1; @eq u8; @lt i8, i16, u16);
    impl_num!(Uint16; @uint, size = 2; @gt u8; @eq u16; @lt i8, i16);
    impl_num!(Uint24; @uint, size = 3; @gt u8, u16; @lt i8, i16);

    #[test]
    fn convenient_upcast() {
        let uint24: Uint24 = 257_u16.into();
        assert_eq!(uint24.as_bytes(), &[0, 1, 1]);
    }

    #[test]
    #[should_panic(expected = "cannot cast 256 to Uint8")]
    fn downcast_overflow_from_bigint() {
        let _uint8: Uint8 = BigUint::from(255_u16).try_into().unwrap();
        let _uint8: Uint8 = BigUint::from(256_u16).try_into().unwrap();
    }

    #[test]
    #[should_panic(expected = "cannot cast 256 to u8")]
    fn downcast_overflow_to_primitive() {
        let uint16 = Uint16::from(255_u16);
        let _u8: u8 = uint16.try_into().unwrap();

        let uint16 = Uint16::from(256_u16);
        let _i16: i16 = uint16.clone().try_into().unwrap();
        let _u16: u16 = uint16.clone().into();
        let _u8: u8 = uint16.try_into().unwrap();
    }
}
