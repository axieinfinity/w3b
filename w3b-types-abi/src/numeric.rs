use w3b_types_core::impl_num;

pub type Int = Int256;
pub type Uint = Uint256;

impl_num! {
    Int8;
    @int, size = 1;
    @eq i8;
    @lt i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int16;
    @int, size = 2;
    @gt i8, u8;
    @eq i16;
    @lt i32, i64, u16, u32, u64;
}

impl_num! {
    Int24;
    @int, size = 3;
    @gt i8, i16, u8, u16;
    @lt i32, i64, u32, u64;
}

impl_num! {
    Int32;
    @int, size = 4;
    @gt i8, i16, u8, u16;
    @eq i32;
    @lt i64, u32, u64;
}

impl_num! {
    Int40;
    @int, size = 5;
    @gt i8, i16, i32, u8, u16, u32;
    @lt i64, u64;
}

impl_num! {
    Int48;
    @int, size = 6;
    @gt i8, i16, i32, u8, u16, u32;
    @lt i64, u64;
}

impl_num! {
    Int56;
    @int, size = 7;
    @gt i8, i16, i32, u8, u16, u32;
    @lt i64, u64;
}

impl_num! {
    Int64;
    @int, size = 8;
    @gt i8, i16, i32, u8, u16, u32;
    @eq i64;
    @lt u64;
}

impl_num! {
    Int72;
    @int, size = 9;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int80;
    @int, size = 10;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int88;
    @int, size = 11;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int96;
    @int, size = 12;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int104;
    @int, size = 13;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int112;
    @int, size = 14;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int120;
    @int, size = 15;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int128;
    @int, size = 16;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int136;
    @int, size = 17;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int144;
    @int, size = 18;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int152;
    @int, size = 19;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int160;
    @int, size = 20;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int168;
    @int, size = 21;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int176;
    @int, size = 22;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int184;
    @int, size = 23;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int192;
    @int, size = 24;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int200;
    @int, size = 25;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int208;
    @int, size = 26;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int216;
    @int, size = 27;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int224;
    @int, size = 28;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int232;
    @int, size = 29;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int240;
    @int, size = 30;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int248;
    @int, size = 31;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Int256;
    @int, size = 32;
    @gt i8, i16, i32, i64, u8, u16, u32, u64;
}

impl_num! {
    Uint8;
    @uint, size = 1;
    @eq u8;
    @lt i8, i16, i32, i64, u16, u32, u64;
}

impl_num! {
    Uint16;
    @uint, size = 2;
    @gt u8;
    @eq u16;
    @lt i8, i16, i32, i64, u32, u64;
}

impl_num! {
    Uint24;
    @uint, size = 3;
    @gt u8, u16;
    @lt i8, i16, i32, i64, u32, u64;
}

impl_num! {
    Uint32;
    @uint, size = 4;
    @gt u8, u16;
    @eq u32;
    @lt i8, i16, i32, i64, u64;
}

impl_num! {
    Uint40;
    @uint, size = 5;
    @gt u8, u16, u32;
    @lt i8, i16, i32, i64, u64;
}

impl_num! {
    Uint48;
    @uint, size = 6;
    @gt u8, u16, u32;
    @lt i8, i16, i32, i64, u64;
}

impl_num! {
    Uint56;
    @uint, size = 7;
    @gt u8, u16, u32;
    @lt i8, i16, i32, i64, u64;
}

impl_num! {
    Uint64;
    @uint, size = 8;
    @gt u8, u16, u32;
    @eq u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint72;
    @uint, size = 9;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint80;
    @uint, size = 10;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint88;
    @uint, size = 11;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint96;
    @uint, size = 12;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint104;
    @uint, size = 13;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint112;
    @uint, size = 14;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint120;
    @uint, size = 15;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint128;
    @uint, size = 16;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint136;
    @uint, size = 17;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint144;
    @uint, size = 18;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint152;
    @uint, size = 19;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint160;
    @uint, size = 20;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint168;
    @uint, size = 21;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint176;
    @uint, size = 22;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint184;
    @uint, size = 23;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint192;
    @uint, size = 24;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint200;
    @uint, size = 25;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint208;
    @uint, size = 26;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint216;
    @uint, size = 27;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint224;
    @uint, size = 28;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint232;
    @uint, size = 29;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint240;
    @uint, size = 30;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint248;
    @uint, size = 31;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

impl_num! {
    Uint256;
    @uint, size = 32;
    @gt u8, u16, u32, u64;
    @lt i8, i16, i32, i64;
}

#[cfg(has_i128)]
const _I128_IMPLS: () = {
    impl_num!(Int8; @lt i128, u128);
    impl_num!(Int16; @lt i128, u128);
    impl_num!(Int24; @lt i128, u128);
    impl_num!(Int32; @lt i128, u128);
    impl_num!(Int40; @lt i128, u128);
    impl_num!(Int48; @lt i128, u128);
    impl_num!(Int56; @lt i128, u128);
    impl_num!(Int64; @lt i128, u128);
    impl_num!(Int72; @lt i128, u128);
    impl_num!(Int80; @lt i128, u128);
    impl_num!(Int88; @lt i128, u128);
    impl_num!(Int96; @lt i128, u128);
    impl_num!(Int104; @lt i128, u128);
    impl_num!(Int112; @lt i128, u128);
    impl_num!(Int120; @lt i128, u128);
    impl_num!(Int128; @eq i128; @lt u128);
    impl_num!(Int136; @gt i128, u128);
    impl_num!(Int144; @gt i128, u128);
    impl_num!(Int152; @gt i128, u128);
    impl_num!(Int160; @gt i128, u128);
    impl_num!(Int168; @gt i128, u128);
    impl_num!(Int176; @gt i128, u128);
    impl_num!(Int184; @gt i128, u128);
    impl_num!(Int192; @gt i128, u128);
    impl_num!(Int200; @gt i128, u128);
    impl_num!(Int208; @gt i128, u128);
    impl_num!(Int216; @gt i128, u128);
    impl_num!(Int224; @gt i128, u128);
    impl_num!(Int232; @gt i128, u128);
    impl_num!(Int240; @gt i128, u128);
    impl_num!(Int248; @gt i128, u128);
    impl_num!(Int256; @gt i128, u128);

    impl_num!(Uint8; @lt i128, u128);
    impl_num!(Uint16; @lt i128, u128);
    impl_num!(Uint24; @lt i128, u128);
    impl_num!(Uint32; @lt i128, u128);
    impl_num!(Uint40; @lt i128, u128);
    impl_num!(Uint48; @lt i128, u128);
    impl_num!(Uint56; @lt i128, u128);
    impl_num!(Uint64; @lt i128, u128);
    impl_num!(Uint72; @lt i128, u128);
    impl_num!(Uint80; @lt i128, u128);
    impl_num!(Uint88; @lt i128, u128);
    impl_num!(Uint96; @lt i128, u128);
    impl_num!(Uint104; @lt i128, u128);
    impl_num!(Uint112; @lt i128, u128);
    impl_num!(Uint120; @lt i128, u128);
    impl_num!(Uint128; @eq u128; @lt i128);
    impl_num!(Uint136; @gt u128; @lt i128);
    impl_num!(Uint144; @gt u128; @lt i128);
    impl_num!(Uint152; @gt u128; @lt i128);
    impl_num!(Uint160; @gt u128; @lt i128);
    impl_num!(Uint168; @gt u128; @lt i128);
    impl_num!(Uint176; @gt u128; @lt i128);
    impl_num!(Uint184; @gt u128; @lt i128);
    impl_num!(Uint192; @gt u128; @lt i128);
    impl_num!(Uint200; @gt u128; @lt i128);
    impl_num!(Uint208; @gt u128; @lt i128);
    impl_num!(Uint216; @gt u128; @lt i128);
    impl_num!(Uint224; @gt u128; @lt i128);
    impl_num!(Uint232; @gt u128; @lt i128);
    impl_num!(Uint240; @gt u128; @lt i128);
    impl_num!(Uint248; @gt u128; @lt i128);
    impl_num!(Uint256; @gt u128; @lt i128);
};
