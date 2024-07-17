pub mod expression;

use expression::ASTExpression;

use crate::{lang::token::Token, utils::transcriber::cursor::TranscriberCursor};

use super::error::ParserError;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Expression(ASTExpression),
    Statement
}

pub trait Node {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized;
}

impl Node for ASTNode {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTNode>, ParserError> {
        // ALL NODES ARE DEFINED HERE
        match cursor.peek().expect("Expected token") {
            Token::EOL => Ok(None),
            _ => ASTExpression::transcribe(cursor).map(|aste| aste.map(ASTNode::Expression))
        }
    }
}