pub mod expression;
pub mod statement;
pub mod block;
pub mod meta;

use std::{cell::RefCell, fmt};

use expression::{ASTExpression, ExpressionNode};
use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, TranscriberTickResult}};
use statement::{ASTStatement, StatementNode, StatementResult};

use crate::{data::DataValue, lang::{parser::{context::ParsingContext, error::{expected_token, ParserError}}, token::{symbol::TokenSymbol, Token}}, vm::{scope::Scope, VM}};

pub type NodeParserTickResult<T> = TranscriberTickResult<T, ParserError>;

/// An instruction for the VM
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// An instruction that ALWAYS returns a value (doesn't modify the context, usually).
    Expression(ASTExpression),

    /// An instruction that works and manipulates the context and data.
    /// It can return a value SOMETIMES.
    Statement(ASTStatement)
}

pub trait Node: fmt::Display {
    /// Transcribe a node from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized;
}

impl StatementNode for ASTNode {
    /// Execute the instruction of the node and return any result (wether it's returned or not)
    fn execute(&self, vm: &mut VM, scope: &RefCell<Scope>) -> Option<StatementResult> {
        match self {
            Self::Expression(expr) => Some(StatementResult::Return(expr.evaluate(vm, scope))),
            Self::Statement(statement) => statement.execute(vm, scope)
        }
    }
}

impl ExpressionNode for ASTNode {
    /// Evaluate the node and return the result
    fn evaluate(&self, vm: &mut VM, scope: &RefCell<Scope>) -> DataValue {
        match self {
            Self::Expression(expr) => expr.evaluate(vm, scope),
            Self::Statement(statement) => statement.execute(vm, scope).map(|result| result.value()).unwrap_or_default()
        }
    }
}

impl Node for ASTNode {
    /// Get a node from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> {
        match cursor.peek() {
            Some(token) => match token {
                // Ignore EOLs (note: the trancriber will automatly move the cursor)
                Token::Symbol(TokenSymbol::EOL) => Ok(None),
                // Try (intent) to transcribe a statement
                _ => ctx.intent(cursor, ASTStatement::transcribe).map(Self::Statement)
                    // if no statement was found, try to transcribe an expression (which won't be a statament-result).
                    .alt_with_map(cursor, ctx, ASTExpression::transcribe, ASTNode::Expression)
                    // TODO: Callables will be here?
                    // Is no expression was found neither, no node was found
                    .tag(expected_token!(<node>))
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