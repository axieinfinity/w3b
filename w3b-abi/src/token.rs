use w3b_types_abi::{Address, Bytes, Int256, Uint256};

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Int(Int256),
    Uint(Uint256),
    Bool(bool),
    Address(Address),
    String(String),
    Bytes(Bytes),
    Array(Vec<Token>),
    FixedBytes(Bytes),
    FixedArray(Vec<Token>),
    Tuple(Vec<Token>),
}

impl Token {
    pub fn is_dynamic(&self) -> bool {
        match self {
            Token::String(_) | Token::Bytes(_) | Token::Array(_) => true,
            Token::FixedArray(tokens) => tokens.iter().any(Token::is_dynamic),
            _ => false,
        }
    }
}
