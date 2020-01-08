mod error;
mod macros;
mod numeric;

pub use error::*;
pub use numeric::*;

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use num_bigint::BigUint;

    use super::{Uint16, Uint8};

    #[test]
    fn convenient_upcast() {
        let uint16: Uint16 = 255u8.into();
        assert_eq!(uint16.bytes(), &[0, 255]);
    }

    #[test]
    #[should_panic(expected = "cannot convert 256 to Uint8")]
    fn downcast_overflow() {
        let _uint8: Uint8 = BigUint::from(255_u16).try_into().unwrap();
        let _uint8: Uint8 = BigUint::from(256_u16).try_into().unwrap();
    }
}
