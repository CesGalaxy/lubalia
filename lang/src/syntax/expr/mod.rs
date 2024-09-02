use literal::LiteralExpression;
use lubalia_utils::cursor::CursorNavigation;

use crate::{parser::{error::ParserError, ParserCursor}, token::{symbol::TokenSymbol, Token}};

use super::node::NodeParsingResult;

pub mod literal;

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Literal(LiteralExpression),
    VarRef(String),
    LastValue,
    Binary,
}

impl ExpressionNode {
    pub fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        Self::parse_terminal(cursor).or_else(|_| {
            match cursor.peek() {
                Some(Token::Symbol(TokenSymbol::ParenOpen)) => {
                    cursor.consume();
                    let expr = Self::parse(cursor)?;
                    cursor.expect(&Token::Symbol(TokenSymbol::ParenClose), ParserError::Expected("end@paren <paren:close>"))?;
                    Ok(expr)
                },
                _ => Err(ParserError::Expected("expr")),
            }
        })
    }

    pub fn parse_terminal(cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        match cursor.peek() {
            // Reference to a variable
            Some(Token::Identifier(ident)) => {
                cursor.consume();
                Ok(Self::VarRef(ident.clone()))
            },

            // A literal value
            Some(Token::Literal(_)) | Some(Token::Keyword(_)) => LiteralExpression::parse(cursor).map(Self::Literal),

            // Reference to the last value computed
            Some(Token::Symbol(TokenSymbol::Underscore)) => {
                cursor.consume();
                Ok(Self::LastValue)
            },

            _ => Err(ParserError::Expected("expr:t")),
        }
    }
}
