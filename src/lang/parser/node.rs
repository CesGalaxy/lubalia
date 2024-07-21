pub mod expression;
pub mod statement;

use expression::{ASTExpression, ExpressionNode};
use statement::{ASTStatement, StatementNode};

use crate::{lang::token::Token, utils::transcriber::cursor::TranscriberCursor, vm::VMTick};

use super::{data::DataValue, error::ParserError};

/// An instruction for the VM
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// An instruction that returns a value (doesn't modify the context, usually)
    Expression(ASTExpression),

    /// An instruction that works and manipulates the context and data
    Statement(ASTStatement)
}

pub trait Node {
    /// Transcribe a node from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized;
}

impl ASTNode {
    /// Execute the instruction og the node
    pub fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        match self {
            Self::Expression(expr) => Some(expr.evaluate(tick)),
            Self::Statement(statement) => {
                statement.execute(tick);
                None
            }
        }
    }
}

impl Node for ASTNode {
    /// Get a node from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTNode>, ParserError> {
        match cursor.peek() {
            Some(token) => match token {
                Token::EOL => Ok(None),
                Token::Keyword(_) => ASTStatement::transcribe(cursor).map(|stmt| stmt.map(ASTNode::Statement)),
                _ => ASTExpression::transcribe(cursor).map(|aste| aste.map(ASTNode::Expression))
            },
            None => Ok(None)
        }
    }
}