pub mod expression;
pub mod structures;
pub mod statement;

use expression::ASTExpression;
use statement::ASTStatement;

use crate::{lang::token::Token, utils::transcriber::cursor::TranscriberCursor};

use super::error::ParserError;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Expression(ASTExpression),
    Statement(ASTStatement)
}

pub trait Node {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized;
}

impl Node for ASTNode {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTNode>, ParserError> {
        match cursor.peek().expect("Expected token") {
            Token::EOL => Ok(None),
            _ => ASTStatement::transcribe(cursor)
                .map(|stmt| stmt.map(ASTNode::Statement))
                .or_else(|_| {
                    ASTExpression::transcribe(cursor).map(|aste| aste.map(ASTNode::Expression))
                })
        }
    }
}