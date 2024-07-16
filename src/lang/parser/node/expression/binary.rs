use crate::lang::parser::{error::ParserError, node::Node};

use super::{terminal::TerminalExpression, ASTExpression};

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

pub struct BinaryExpression {
    lhs: TerminalExpression,
    operator: Operator,
    rhs: ASTExpression
}

impl Node for BinaryExpression {
    fn transcribe(_cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> Result<Option<BinaryExpression>, ParserError> {
        let lhs = TerminalExpression::transcribe(cursor, initial_token)?.ok_or(ParserError::Expected("<expr:terminal>".to_string()));

        let operator = match cursor.consume() {
            Token::Keyword("add") => Operator::Add,
            Token::Keyword("sub") => Operator::Sub,
            Token::Keyword("mul") => Operator::Mul,
            Token::Keyword("div") => Operator::Div
        };

        let rhs = ASTExpression::transcribe(cursor, initial_token)?;

        Ok(Some(BinaryExpression { lhs, operator, rhs }))
    }
}