use crate::lang::token::TokenLiteral;

#[derive(Debug, Clone)]
pub enum DataValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null
}

impl From<TokenLiteral> for DataValue {
    fn from(literal: TokenLiteral) -> Self {
        match literal {
            TokenLiteral::Number(number) => DataValue::Number(number),
            TokenLiteral::String(string) => DataValue::String(string),
        }
    }
}