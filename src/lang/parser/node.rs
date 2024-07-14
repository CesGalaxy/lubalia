use crate::{lang::token::Token, utils::transcriber::cursor::TranscriberCursor};

use super::error::ParserError;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Expression,
    Statement
}

pub trait Node {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> Result<Option<ASTNode>, ParserError>;
}

impl ASTNode {
    pub fn transcribe(_cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> Result<Option<ASTNode>, ParserError> {
        // ALL NODES ARE DEFINED HERE
        match initial_token {
            Token::EOL => Ok(None),
            _ => Ok(None)
        }
    }
}