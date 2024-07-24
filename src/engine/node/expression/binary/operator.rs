use crate::lang::token::{symbol::TokenSymbol, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    // TODO: I'm too lazy for implementing operators with 2 symbols
    // NoEqual,
    Greater,
    // GreaterOrEqual,
    Less,
    // LessOrEqual,
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    XNOR
}

// impl BinaryOperator {
//     pub fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Option<Self> {
//         cursor.consume().and_then(|token| {
//             match token {
//                 Token::Symbol(TokenSymbol::Plus) => Some(BinaryOperator::Add),
//                 Token::Symbol(TokenSymbol::Minus) => Some(BinaryOperator::Sub),
//                 Token::Symbol(TokenSymbol::Asterisk) => Some(BinaryOperator::Mul),
//                 Token::Symbol(TokenSymbol::Slash) => Some(BinaryOperator::Div),
//                 Token::Symbol(TokenSymbol::Equal) => Some(BinaryOperator::Equal),
//                 Token::Symbol(TokenSymbol::GreaterThan) => Some(BinaryOperator::Greater),
//                 Token::Symbol(TokenSymbol::LessThan) => Some(BinaryOperator::Less),
//                 Token::Symbol(TokenSymbol::Ampersand) => Some(BinaryOperator::AND),
//                 Token::Symbol(TokenSymbol::Pipe) => Some(BinaryOperator::OR),
//                 Token::Keyword(keyword) => match keyword.to_ascii_lowercase().as_str() {
//                     "and" => Some(BinaryOperator::AND),
//                     "or" => Some(BinaryOperator::OR),
//                     "nand" => Some(BinaryOperator::NAND),
//                     "nor" => Some(BinaryOperator::NOR),
//                     "xor" => Some(BinaryOperator::XOR),
//                     "xnor" => Some(BinaryOperator::XNOR),
//                     _ => None
//                 },
//                 _ => None
//             }
//         })
//     }
// }

impl From<&BinaryOperator> for u8 {
    fn from(value: &BinaryOperator) -> Self {
        match value {
            BinaryOperator::AND | BinaryOperator::OR | BinaryOperator::NAND | BinaryOperator::NOR | BinaryOperator::XOR | BinaryOperator::XNOR => 3,
            BinaryOperator::Equal | BinaryOperator::Greater | BinaryOperator::Less => 2,
            BinaryOperator::Mul | BinaryOperator::Div => 1,
            BinaryOperator::Add | BinaryOperator::Sub => 0,
        }
    }
}

impl From<&Token> for Option<BinaryOperator> {
    fn from(value: &Token) -> Self {
        match value {
            Token::Symbol(TokenSymbol::Plus) => Some(BinaryOperator::Add),
            Token::Symbol(TokenSymbol::Minus) => Some(BinaryOperator::Sub),
            Token::Symbol(TokenSymbol::Asterisk) => Some(BinaryOperator::Mul),
            Token::Symbol(TokenSymbol::Slash) => Some(BinaryOperator::Div),
            Token::Symbol(TokenSymbol::Equal) => Some(BinaryOperator::Equal),
            Token::Symbol(TokenSymbol::GreaterThan) => Some(BinaryOperator::Greater),
            Token::Symbol(TokenSymbol::LessThan) => Some(BinaryOperator::Less),
            Token::Symbol(TokenSymbol::Ampersand) => Some(BinaryOperator::AND),
            Token::Symbol(TokenSymbol::Pipe) => Some(BinaryOperator::OR),
            Token::CustomKeyword(keyword) => match keyword.to_ascii_lowercase().as_str() {
                "and" => Some(BinaryOperator::AND),
                "or" => Some(BinaryOperator::OR),
                "nand" => Some(BinaryOperator::NAND),
                "nor" => Some(BinaryOperator::NOR),
                "xor" => Some(BinaryOperator::XOR),
                "xnor" => Some(BinaryOperator::XNOR),
                _ => None
            },
            _ => None
        }
    }
}

impl Token {
    pub fn is_operator(&self) -> bool {
        match self {
            Token::Symbol(symbol) => match symbol {
                TokenSymbol::Plus | TokenSymbol::Minus | TokenSymbol::Asterisk | TokenSymbol::Slash | TokenSymbol::Equal
                | TokenSymbol::GreaterThan | TokenSymbol::LessThan | TokenSymbol::Ampersand | TokenSymbol::Pipe => true,
                _ => false
            },
            _ => false
        }
    }
}