#[derive(PartialEq, Eq, Debug)]
pub enum ParamType {
    Int(usize),
    Uint(usize),
    Bool,
    Address,
    String,
    Bytes,
    Array(Box<ParamType>),
    FixedBytes(usize),
    FixedArray(Box<ParamType>, usize),
    Tuple(Vec<Box<ParamType>>),
}

impl ParamType {
    /// ```rust
    /// # use w3b_abi::ParamType;
    ///
    /// assert_eq!(
    ///     ParamType::parse("uint[100]").unwrap(),
    ///     ParamType::FixedArray(Box::new(ParamType::Uint(256)), 100),
    /// );
    ///
    /// assert_eq!(
    ///     ParamType::parse("string[100][]").unwrap(),
    ///     ParamType::Array(Box::new(ParamType::FixedArray(Box::new(ParamType::String), 100))),
    /// );
    ///
    /// assert_eq!(
    ///     ParamType::parse("uint100]").unwrap_err(),
    ///     String::from("no matching character [ in uint100]"),
    /// );
    ///
    /// assert_eq!(
    ///     ParamType::parse("uint[100").unwrap_err(),
    ///     String::from("invalid unsigned number [100"),
    /// );
    /// ```
    pub fn parse(ty: &str) -> Result<ParamType, String> {
        use ParamType::*;

        if let Some(']') = ty.chars().last() {
            let delimiter = ty
                .rfind('[')
                .ok_or(format!("no matching character [ in {}", ty))?;

            let size = &ty[delimiter + 1..ty.len() - 1];
            let subtype = Self::parse(&ty[..delimiter])?;

            return if size.is_empty() {
                Ok(Array(Box::new(subtype)))
            } else {
                Ok(FixedArray(Box::new(subtype), parse_num(size)?))
            };
        }

        Ok(match ty {
            "int" => Int(256),
            "uint" => Uint(256),
            "bool" => Bool,
            "address" => Address,
            "string" => String,
            "bytes" => Bytes,

            ty if ty.starts_with("int") => Int(parse_num(&ty[3..])?),
            ty if ty.starts_with("uint") => Uint(parse_num(&ty[4..])?),
            ty if ty.starts_with("bytes") => FixedBytes(parse_num(&ty[5..])?),

            _ => return Err(format!("invalid parameter type {}", ty)),
        })
    }
}

impl ToString for ParamType {
    fn to_string(&self) -> String {
        use ParamType::*;

        match self {
            Int(size) => format!("int{}", size),
            Uint(size) => format!("uint{}", size),
            Bool => "bool".to_owned(),
            Address => "address".to_owned(),
            String => "string".to_owned(),
            Bytes => "bytes".to_owned(),
            Array(subtype) => format!("{}[]", subtype.to_string()),
            FixedBytes(size) => format!("bytes{}", size),
            FixedArray(subtype, size) => format!("{}[{}]", subtype.to_string(), size),
            Tuple(_) => "tuple".to_owned(),
        }
    }
}

fn parse_num(s: &str) -> Result<usize, String> {
    usize::from_str_radix(s, 10).map_err(|_| format!("invalid unsigned number {}", s))
}
