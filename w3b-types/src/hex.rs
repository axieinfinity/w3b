pub struct Hex<T>(T);

impl<T> Hex<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    #[inline]
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Hex<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

macro_rules! impl_num {
    ($num:ident) => {
        impl $crate::serde::Serialize for Hex<$num> {
            #[inline]
            fn serialize<S: $crate::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                $crate::w3b_types_core::hex::serialize(self.0.to_be_bytes().as_ref(), serializer)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for Hex<$num> {
            #[inline]
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let mut repr = [0; ::std::mem::size_of::<$num>()];
                $crate::w3b_types_core::hex::deserialize_expanded(&mut repr, deserializer)?;
                Ok(Self($num::from_be_bytes(repr)))
            }
        }
    };
}

impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);

#[cfg(has_i128)]
const _I128_IMPLS: () = {
    impl_num!(i128);
    impl_num!(u128);
};
