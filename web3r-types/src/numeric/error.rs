use std::{error::Error, fmt};

use num_bigint::BigInt;

pub struct NumericConversionError {
    value: BigInt,
    into_type: String,
}

impl NumericConversionError {
    #[inline]
    pub fn new(value: BigInt, into_type: impl Into<String>) -> Self {
        Self {
            value,
            into_type: into_type.into(),
        }
    }
}

impl fmt::Debug for NumericConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cannot convert {} to {}",
            self.value.to_str_radix(10),
            self.into_type
        )
    }
}

impl fmt::Display for NumericConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for NumericConversionError {}
