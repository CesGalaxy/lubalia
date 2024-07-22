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

pub trait Node: std::fmt::Display {
    /// Transcribe a node from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized;
}

impl ASTNode {
    /// Execute the instruction og the node
    pub fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        match self {
            Self::Expression(expr) => Some(expr.evaluate(tick)),
            Self::Statement(statement) => statement.execute(tick)
        }
    }
}

impl Node for ASTNode {
    /// Get a node from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTNode>, ParserError> {
        match cursor.peek() {
            Some(token) => match token {
                Token::EOL => Ok(None),
                _ => Ok(
                    // Try to transcribe a statement (error handled with ControlFlow),
                    ASTStatement::transcribe(cursor)?.map(ASTNode::Statement)
                        // if no statement was found, try to transcribe an expression (which won't be a statament-result).
                        // The error is also handled with ControlFlow
                        .or(ASTExpression::transcribe(cursor)?.map(ASTNode::Expression))
                )
            },
            None => Ok(None)
        }
    }
}

impl std::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(expr) => write!(f, "{}{expr}{}", "<", ">"),
            Self::Statement(stmt) => write!(f, "{}{stmt}{}", "[", "]")
        }
    }
}