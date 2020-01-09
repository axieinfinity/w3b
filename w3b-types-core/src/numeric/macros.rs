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
        #[derive(PartialEq, Eq, Debug)]
        pub struct $num([u8; Self::NUM_BYTES]);

        impl $num {
            pub const NUM_BYTES: usize = $n_bytes;

            #[inline]
            pub fn new(repr: [u8; Self::NUM_BYTES]) -> Self {
                Self(repr)
            }

            #[inline]
            pub fn from_bytes(bytes: &[u8]) -> Result<Self, ::std::array::TryFromSliceError> {
                <[u8; Self::NUM_BYTES] as ::std::convert::TryFrom<&[u8]>>::try_from(bytes).map(Self)
            }

            #[inline]
            pub fn from_hex(hex: impl AsRef<str>) -> Result<Self, $crate::hex::HexError> {
                $crate::hex::from_hex(
                    hex,
                    &$crate::hex::ExpectedHexLen::AtMost((Self::NUM_BYTES << 1) + 2),
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
                $crate::hex::to_hex(self.as_bytes(), true)
            }
        }

        impl ::std::fmt::LowerHex for $num {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if f.alternate() {
                    write!(f, "0x")?;
                }

                write!(f, "{}", &self.to_hex()[2..])
            }
        }

        impl ::std::fmt::UpperHex for $num {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if f.alternate() {
                    write!(f, "0x")?;
                }

                write!(f, "{}", &self.to_hex().to_uppercase()[2..])
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
                $crate::ser::serialize_numeric(&self.0, serializer)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $num {
            #[inline]
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                $crate::ser::deserialize_at_most_size(deserializer).map(Self)
            }
        }
    };

    (@int $num:ident) => {
        impl ::std::convert::TryFrom<$crate::num_bigint::BigInt> for $num {
            type Error = $crate::numeric::NumCastError;

            fn try_from(value: $crate::num_bigint::BigInt) -> Result<Self, Self::Error> {
                let bytes = value.to_signed_bytes_be();

                if bytes.len() <= Self::NUM_BYTES {
                    let mut repr = [0; Self::NUM_BYTES];
                    repr.as_mut()[Self::NUM_BYTES - bytes.len()..].copy_from_slice(&*bytes);
                    Ok(Self(repr))
                } else {
                    Err(Self::Error::new(value, stringify!($num)))
                }
            }
        }

        impl ::std::convert::TryFrom<$crate::num_bigint::BigUint> for $num {
            type Error = <Self as ::std::convert::TryFrom<$crate::num_bigint::BigInt>>::Error;

            #[inline]
            fn try_from(value: $crate::num_bigint::BigUint) -> Result<Self, Self::Error> {
                <Self as ::std::convert::TryFrom<$crate::num_bigint::BigInt>>::try_from(
                    value.into(),
                )
            }
        }
    };

    (@uint $num:ident) => {
        impl ::std::convert::TryFrom<$crate::num_bigint::BigUint> for $num {
            type Error = $crate::numeric::NumCastError;

            fn try_from(value: $crate::num_bigint::BigUint) -> Result<Self, Self::Error> {
                let bytes = value.to_bytes_be();

                if bytes.len() <= Self::NUM_BYTES {
                    let mut repr = [0; Self::NUM_BYTES];
                    repr.as_mut()[Self::NUM_BYTES - bytes.len()..].copy_from_slice(&*bytes);
                    Ok(Self(repr))
                } else {
                    Err(Self::Error::new(value.into(), stringify!($num)))
                }
            }
        }

        impl ::std::convert::TryFrom<$crate::num_bigint::BigInt> for $num {
            type Error = <Self as ::std::convert::TryFrom<$crate::num_bigint::BigUint>>::Error;

            fn try_from(value: $crate::num_bigint::BigInt) -> Result<Self, Self::Error> {
                match value.to_biguint() {
                    Some(value) => {
                        <Self as ::std::convert::TryFrom<$crate::num_bigint::BigUint>>::try_from(
                            value,
                        )
                    }

                    None => Err(Self::Error::new(value, stringify!($num))),
                }
            }
        }
    };

    (@impl From<$num:ident> for $primitive:ty) => {
        impl From<$num> for $primitive {
            #[inline]
            fn from(value: $num) -> Self {
                <Self as $crate::num_traits::NumCast>::from($crate::num_bigint::BigInt::from(value))
                    .unwrap()
            }
        }
    };

    (@impl From<$primitive:ty> for $num:ident) => {
        impl From<$primitive> for $num {
            #[inline]
            fn from(value: $primitive) -> Self {
                <Self as ::std::convert::TryFrom<$crate::num_bigint::BigInt>>::try_from(
                    value.into(),
                )
                .unwrap()
            }
        }
    };

    (@impl TryFrom<$num:ident> for $primitive:ty) => {
        impl ::std::convert::TryFrom<$num> for $primitive {
            type Error = $crate::numeric::NumCastError;

            fn try_from(value: $num) -> Result<Self, Self::Error> {
                let value = $crate::num_bigint::BigInt::from(value);

                <Self as $crate::num_traits::NumCast>::from(value.clone())
                    .ok_or(Self::Error::new(value, stringify!($primitive)))
            }
        }
    };

    (@impl TryFrom<$primitive:ty> for $num:ident) => {
        impl ::std::convert::TryFrom<$primitive> for $num {
            type Error = <Self as ::std::convert::TryFrom<$crate::num_bigint::BigInt>>::Error;

            #[inline]
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                <Self as ::std::convert::TryFrom<$crate::num_bigint::BigInt>>::try_from(
                    value.into(),
                )
            }
        }
    };
}
