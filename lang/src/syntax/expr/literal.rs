use lubalia_utils::cursor::CursorNavigation;

use crate::{parser::{error::ParserError, ParserCursor}, syntax::node::NodeParsingResult, token::{keyword::TokenLangKeyword, literal::TokenLiteral, Token}};

#[derive(Debug, Clone)]
pub enum LiteralExpression {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl LiteralExpression {
    pub fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        match cursor.consume() {
            Some(Token::Literal(TokenLiteral::Number(n))) => Ok(Self::Number(*n)),
            Some(Token::Literal(TokenLiteral::String(s))) => Ok(Self::String(s.clone())),
            Some(Token::Keyword(TokenLangKeyword::True)) => Ok(Self::Boolean(true)),
            Some(Token::Keyword(TokenLangKeyword::False)) => Ok(Self::Boolean(false)),
            None => Err(ParserError::UnexpectedEnd),
            _ => Err(ParserError::Expected("expr:lit")),
        }
    }
}
