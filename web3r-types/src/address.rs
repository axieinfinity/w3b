use crate::impl_bytes;

use super::bytes::Bytes20;

impl_bytes!(Address; size = 20);

impl From<Bytes20> for Address {
    #[inline]
    fn from(value: Bytes20) -> Self {
        Self(*value.as_repr())
    }
}

impl From<Address> for Bytes20 {
    #[inline]
    fn from(value: Address) -> Self {
        Bytes20::new(*value.as_repr())
    }
}

#[cfg(test)]
mod tests {
    use super::Address;

    #[test]
    fn from_hex() {
        assert_eq!(
            Address::from_hex(String::from("0x") + &"6".repeat(40)).unwrap(),
            Address([0x66; 20])
        );
    }

    #[test]
    #[should_panic(expected = "incorrect length 41, expected 42")]
    fn from_hex_with_incorrect_len() {
        assert_eq!(
            Address::from_hex(String::from("0x") + &"6".repeat(39)).unwrap(),
            Address([0x66; 20])
        );
    }

    #[test]
    fn to_hex() {
        assert_eq!(
            Address([0x88; 20]).to_hex(),
            String::from("0x") + &"8".repeat(40)
        );
    }

    #[test]
    fn to_hex_with_leading_zeroes() {
        let mut address = Address([0x88; 20]);
        address.0[0] = 0;
        address.0[1] = 7;
        assert_eq!(address.to_hex(), String::from("0x0007") + &"8".repeat(36));
    }

    #[test]
    fn format_as_hex() {
        assert_eq!(format!("{:x}", Address([0xaa; 20])), "a".repeat(40));

        assert_eq!(
            format!("{:#x}", Address([0x88; 20])),
            format!("0x{}", "8".repeat(40)),
        );

        assert_eq!(format!("{:X}", Address([0xbc; 20])), "BC".repeat(20));

        assert_eq!(
            format!("{:#X}", Address([0xdd; 20])),
            format!("0x{}", "D".repeat(40))
        );
    }
}
