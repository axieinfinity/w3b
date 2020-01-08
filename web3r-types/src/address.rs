use crate::impl_bytes;

use super::bytes::Bytes20;

impl_bytes!(Address; size = 20);

impl From<Bytes20> for Address {
    #[inline]
    fn from(value: Bytes20) -> Self {
        Self(*value.repr())
    }
}
