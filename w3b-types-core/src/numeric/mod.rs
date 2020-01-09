mod error;
mod macros;

pub use error::*;

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use num_bigint::BigUint;

    use crate::impl_num;

    impl_num!(Uint8; @uint, size = 1);
    impl_num!(Uint16; @uint, size = 2; @gt u8);

    #[test]
    fn convenient_upcast() {
        let uint16: Uint16 = 255u8.into();
        assert_eq!(uint16.as_bytes(), &[0, 255]);
    }

    #[test]
    #[should_panic(expected = "cannot cast 256 to Uint8")]
    fn downcast_overflow() {
        let _uint8: Uint8 = BigUint::from(255_u16).try_into().unwrap();
        let _uint8: Uint8 = BigUint::from(256_u16).try_into().unwrap();
    }
}
