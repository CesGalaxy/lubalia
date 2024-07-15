pub mod terminal;

use crate::{lang::{parser::error::ParserError, token::Token}, utils::transcriber::cursor::TranscriberCursor};

use super::Node;

#[derive(Debug, Clone)]
pub enum ASTExpression {
    Terminal(terminal::TerminalExpression),
}

impl Node for ASTExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> Result<Option<ASTExpression>, ParserError> {
        terminal::TerminalExpression::transcribe(cursor, initial_token).map(|texpr| texpr.map(ASTExpression::Terminal))
    }
}