pub mod terminal;
pub mod binary;

use crate::{
    lang::{parser::error::ParserError, token::Token},
    utils::transcriber::cursor::TranscriberCursor
};

use super::Node;

#[derive(Debug, Clone)]
pub enum ASTExpression {
    Terminal(terminal::TerminalExpression),
    Binary(binary::BinaryExpression)
}

impl Node for ASTExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTExpression>, ParserError> {
        match cursor.peek_next() {
            Some(Token::Symbol(_)) => binary::BinaryExpression::transcribe(cursor).map(|bexpr| bexpr.map(ASTExpression::Binary)),
            _ => Ok(
                terminal::TerminalExpression::transcribe(cursor)
                    .unwrap_or(None)
                    .map(ASTExpression::Terminal)
            )
        }
    }
}