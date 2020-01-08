use std::{error::Error, fmt};

use num_bigint::BigInt;

pub struct NumCastError {
    value: BigInt,
    into_type: String,
}

impl NumCastError {
    #[inline]
    pub fn new(value: BigInt, into_type: impl Into<String>) -> Self {
        Self {
            value,
            into_type: into_type.into(),
        }
    }
}

impl fmt::Debug for NumCastError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cannot cast {} to {}",
            self.value.to_str_radix(10),
            self.into_type
        )
    }
}

impl fmt::Display for NumCastError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for NumCastError {}
