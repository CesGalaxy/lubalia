use crate::lang::parser::{error::ParserError, node::Node};
use crate::lang::token::{Token, TokenSymbol};
use crate::utils::transcriber::cursor::TranscriberCursor;
use super::{terminal::TerminalExpression, ASTExpression};

#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    lhs: TerminalExpression,
    operator: Operator,
    /// Use box for recursive types
    rhs: Box<ASTExpression>
}

impl Node for BinaryExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> Result<Option<BinaryExpression>, ParserError> {
        let lhs = TerminalExpression::transcribe(cursor, initial_token)?.ok_or(ParserError::Expected("<expr:terminal>".to_string()))?;

        let operator = match cursor.consume() {
            Some(Token::Symbol(TokenSymbol::Plus)) => Operator::Add,
            Some(Token::Symbol(TokenSymbol::Minus)) => Operator::Sub,
            Some(Token::Symbol(TokenSymbol::Asterisk)) => Operator::Mul,
            Some(Token::Symbol(TokenSymbol::Slash)) => Operator::Div,
            _ => return Err(ParserError::Expected("<sym/operator>".to_string())),
        };

        let rhs = Box::new(ASTExpression::transcribe(cursor, initial_token)?.ok_or(ParserError::Expected("<expr>".to_string()))?);

        Ok(Some(BinaryExpression { lhs, operator, rhs }))
    }
}