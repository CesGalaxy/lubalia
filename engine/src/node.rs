pub mod expression;
pub mod statement;

use std::fmt;

use expression::{ASTExpression, ExpressionNode};
use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException, TranscriberTickResult}};
use statement::{ASTStatement, StatementNode};

use crate::{lang::{parser::error::ParserError, token::{symbol::TokenSymbol, Token}}, vm::tick::VMTick};

use super::data::DataValue;

pub type NodeParserTickResult<T> = TranscriberTickResult<T, ParserError>;

/// An instruction for the VM
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// An instruction that ALWAYS returns a value (doesn't modify the context, usually)
    Expression(ASTExpression),

    /// An instruction that works and manipulates the context and data
    /// It can return a value SOMETIMES
    Statement(ASTStatement)
}

pub trait Node: fmt::Display {
    /// Transcribe a node from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> where Self: Sized;
}

impl ASTNode {
    /// Execute the instruction of the node
    pub fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        match self {
            Self::Expression(expr) => Some(expr.evaluate(tick)),
            Self::Statement(statement) => statement.execute(tick).map(|result| result.returned()).flatten()
        }
    }
}

impl Node for ASTNode {
    /// Get a node from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> {
        match cursor.peek() {
            Some(token) => match token {
                // Ignore EOLs (note: the trancriber will automatly move the cursor)
                Token::Symbol(TokenSymbol::EOL) => Ok(None),
                // Try (intent) to transcribe a statement
                _ => cursor.intent(ASTStatement::transcribe).map(|stmnt| stmnt.map(|stmnt| stmnt.map(Self::Statement)))
                        // if no statement was found, try to transcribe an expression (which won't be a statament-result).
                        // TODO: The expression will never be a statement, will it?
                        .alt(|| cursor.intent(ASTExpression::transcribe).map(|expr| expr.map(|expr| expr.map(ASTNode::Expression))))
                        // All nodes must end with a new line
                        .check(|_| if let Some(Token::Symbol(TokenSymbol::EOL)) = cursor.consume() {
                            None
                        } else {
                            Some(Err(TranscriptionException::Error(ParserError::Expected("end of line".to_string()))))
                        })
                        // Is no expression was found neither, no node was found
                        .tag("<node>".to_string())
            },
            None => Ok(None)
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression(expr) => write!(f, "<{expr}>"),
            Self::Statement(stmt) => write!(f, "[{stmt}]"),
        }
    }
}