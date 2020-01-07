#[doc(hidden)]
#[macro_export]
macro_rules! __impl_num {
    ($num:ident; @int, size = $n_bytes:literal $($tail:tt)*) => {
        __impl_num!(@common $num, $n_bytes);
        __impl_num!(@int $num);
        __impl_num!($num $($tail)*);
    };

    ($num:ident; @uint, size = $n_bytes:literal $($tail:tt)*) => {
        __impl_num!(@common $num, $n_bytes);
        __impl_num!(@uint $num);
        __impl_num!($num $($tail)*);
    };

    ($num:ident; @gt $($primitive:ty),*; $($tail:tt)*) => {
        __impl_num!($num; @gt $($primitive),*);
        __impl_num!($num; $($tail)*);
    };

    ($num:ident; @gt $($primitive:ty),*) => {
        $(
            __impl_num!(@impl TryFrom<$num> for $primitive);
            __impl_num!(@impl From<$primitive> for $num);
        )*
    };

    ($num:ident; @eq $($primitive:ty),*; $($tail:tt)*) => {
        __impl_num!($num; @eq $($primitive),*);
        __impl_num!($num; $($tail)*);
    };

    ($num:ident; @eq $($primitive:ty),*) => {
        $(
            __impl_num!(@impl From<$num> for $primitive);
            __impl_num!(@impl From<$primitive> for $num);
        )*
    };

    ($num:ident; @lt $($primitive:ty),*; $($tail:tt)*) => {
        __impl_num!($num; @lt $($primitive),*);
        __impl_num!($num; $($tail)*);
    };

    ($num:ident; @lt $($primitive:ty),*) => {
        $(
            __impl_num!(@impl From<$num> for $primitive);
            __impl_num!(@impl TryFrom<$primitive> for $num);
        )*
    };

    ($num:ident $(;)?) => {};

    (@common $num:ident, $n_bytes:literal) => {
        pub struct $num([u8; Self::NUM_BYTES]);

        impl $num {
            pub const NUM_BYTES: usize = $n_bytes;

            #[inline]
            pub fn with_repr(repr: [u8; Self::NUM_BYTES]) -> Self {
                Self(repr)
            }

            #[inline]
            pub fn repr(&self) -> &[u8; Self::NUM_BYTES] {
                &self.0
            }

            #[inline]
            pub fn bytes(&self) -> &[u8] {
                self.repr().as_ref()
            }
        }

        impl From<$num> for ::num_bigint::BigInt {
            #[inline]
            fn from(value: $num) -> Self {
                Self::from_bytes_be(::num_bigint::Sign::Plus, value.bytes())
            }
        }

        impl From<$num> for ::num_bigint::BigUint {
            #[inline]
            fn from(value: $num) -> Self {
                Self::from_bytes_be(value.bytes())
            }
        }
    };

    (@int $num:ident) => {
        impl ::std::convert::TryFrom<::num_bigint::BigInt> for $num {
            type Error = $crate::numeric::NumericConversionError;

            fn try_from(value: ::num_bigint::BigInt) -> Result<Self, Self::Error> {
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

        impl ::std::convert::TryFrom<::num_bigint::BigUint> for $num {
            type Error = <Self as ::std::convert::TryFrom<::num_bigint::BigInt>>::Error;

            fn try_from(value: ::num_bigint::BigUint) -> Result<Self, Self::Error> {
                <Self as ::std::convert::TryFrom<::num_bigint::BigInt>>::try_from(value.into())
            }
        }
    };

    (@uint $num:ident) => {
        impl ::std::convert::TryFrom<::num_bigint::BigUint> for $num {
            type Error = $crate::numeric::NumericConversionError;

            fn try_from(value: ::num_bigint::BigUint) -> Result<Self, Self::Error> {
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

        impl ::std::convert::TryFrom<::num_bigint::BigInt> for $num {
            type Error = <Self as ::std::convert::TryFrom<::num_bigint::BigUint>>::Error;

            fn try_from(value: ::num_bigint::BigInt) -> Result<Self, Self::Error> {
                match value.to_biguint() {
                    Some(value) => <Self as ::std::convert::TryFrom<::num_bigint::BigUint>>::try_from(value),
                    None => Err(Self::Error::new(value, stringify!($num))),
                }
            }
        }
    };

    (@impl From<$num:ident> for $primitive:ty) => {
        impl From<$num> for $primitive {
            #[inline]
            fn from(value: $num) -> Self {
                <Self as ::num_traits::NumCast>::from(::num_bigint::BigInt::from(value)).unwrap()
            }
        }
    };

    (@impl From<$primitive:ty> for $num:ident) => {
        impl From<$primitive> for $num {
            #[inline]
            fn from(value: $primitive) -> Self {
                <Self as ::std::convert::TryFrom<::num_bigint::BigInt>>::try_from(value.into()).unwrap()
            }
        }
    };

    (@impl TryFrom<$num:ident> for $primitive:ty) => {
        impl ::std::convert::TryFrom<$num> for $primitive {
            type Error = $crate::numeric::NumericConversionError;

            fn try_from(value: $num) -> Result<Self, Self::Error> {
                let value = ::num_bigint::BigInt::from(value);

                <Self as ::num_traits::NumCast>::from(value.clone())
                    .ok_or(Self::Error::new(value, stringify!($primitive)))
            }
        }
    };

    (@impl TryFrom<$primitive:ty> for $num:ident) => {
        impl ::std::convert::TryFrom<$primitive> for $num {
            type Error = <Self as ::std::convert::TryFrom<::num_bigint::BigInt>>::Error;

            #[inline]
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                <Self as ::std::convert::TryFrom<::num_bigint::BigInt>>::try_from(value.into())
            }
        }
    };
}
