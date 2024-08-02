use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{lang::{parser::error::expected_token, token::{symbol::TokenSymbol, Token}}, node::NodeParserTickResult};

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    // Basic maths
    Add, Sub, Mul, Div,

    // Comparison
    Equal, NoEqual,
    Greater, GreaterOrEqual,
    Less, LessOrEqual,

    // Logical
    AND, OR,
    NAND, NOR,
    XOR, XNOR
}

impl BinaryOperator {
    pub fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> {
        match cursor.consume() {
            Some(token) => match token {
                Token::Symbol(symbol) => match symbol {
                    TokenSymbol::Plus => Ok(Some(BinaryOperator::Add)),
                    TokenSymbol::Minus => Ok(Some(BinaryOperator::Sub)),
                    TokenSymbol::Asterisk => Ok(Some(BinaryOperator::Mul)),
                    TokenSymbol::Slash => Ok(Some(BinaryOperator::Div)),
                    TokenSymbol::Equal => Ok(Some(BinaryOperator::Equal)),
                    TokenSymbol::GreaterThan => Ok(Some(BinaryOperator::Greater)),
                    TokenSymbol::LessThan => Ok(Some(BinaryOperator::Less)),
                    TokenSymbol::Ampersand => Ok(Some(BinaryOperator::AND)),
                    TokenSymbol::Pipe => Ok(Some(BinaryOperator::OR)),
                    _ => match cursor.consume() {
                        Some(Token::Symbol(symbol2)) => match (symbol, symbol2) {
                            (TokenSymbol::Exclamation, TokenSymbol::Equal) => Ok(Some(BinaryOperator::NoEqual)),
                            (TokenSymbol::GreaterThan, TokenSymbol::Equal) => Ok(Some(BinaryOperator::GreaterOrEqual)),
                            (TokenSymbol::LessThan, TokenSymbol::Equal) => Ok(Some(BinaryOperator::LessOrEqual)),
                            (TokenSymbol::Ampersand, TokenSymbol::Ampersand) => Ok(Some(BinaryOperator::NAND)),
                            (TokenSymbol::Pipe, TokenSymbol::Pipe) => Ok(Some(BinaryOperator::NOR)),
                            _ => Err(TranscriptionException::NotFound(expected_token!(operator@bi_op | operator@bi <operator:sym>)))
                        },
                        _ => Err(TranscriptionException::NotFound(expected_token!(operator@bi_op | operator@bi <operator:sym>)))
                    }
                },
                // TODO: Keyword operators
                _ => Err(TranscriptionException::NotFound(expected_token!(operator@bi_op | operator@bi <operator>)))
            },
            _ => Err(TranscriptionException::NotFound(expected_token!(operator@bi_op | operator@bi <operator>)))
        }
    }
}

impl From<&BinaryOperator> for u8 {
    fn from(value: &BinaryOperator) -> Self {
        match value {
            BinaryOperator::Mul | BinaryOperator::Div => 3,
            BinaryOperator::Add | BinaryOperator::Sub => 2,
            BinaryOperator::Equal | BinaryOperator::NoEqual | BinaryOperator::Greater | BinaryOperator::GreaterOrEqual | BinaryOperator::Less | BinaryOperator::LessOrEqual => 1,
            BinaryOperator::AND | BinaryOperator::OR | BinaryOperator::NAND | BinaryOperator::NOR | BinaryOperator::XOR | BinaryOperator::XNOR => 0,
        }
    }
}
