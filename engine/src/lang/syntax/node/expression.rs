pub mod terminal;
pub mod binary;
pub mod ufunc_constructor;
pub mod list;

use std::fmt;

use lubalia_utils::transcriber::cursor::TranscriberCursor;

use crate::{data::DataValue, lang::{parser::{context::ParsingContext, error::expected_token}, token::Token}, vm::tick::VMTick};

use super::{Node, NodeParserTickResult};

/// An expression that can be evaluated to a value
#[derive(Debug, Clone)]
pub enum ASTExpression {
    /// A terminal expression (literal, variable reference, scope, etc.) will always return a value,
    /// and doesn't require any manipulation of the referenced/provided value.
    Terminal(terminal::TerminalExpression),

    /// A expression that requires two values (binary expression) to be combined through an operation.
    Binary(binary::BinaryExpression)
}

pub trait ExpressionNode: Node {
    // TODO: Return an Option (for optional returns in statements)
    /// Evaluate the expression and return the result value
    fn evaluate(&self, tick: &mut VMTick) -> DataValue;
}

impl Node for ASTExpression {
    /// Transcribe any kind of expression (if possible)
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> {
        ctx.intent(cursor, binary::BinaryExpression::transcribe).map(ASTExpression::Binary)
            .alt_with_map(cursor, ctx, terminal::TerminalExpression::transcribe, ASTExpression::Terminal)
            .tag(expected_token!(<expr>))
    }
}

impl ExpressionNode for ASTExpression {
    /// Evaluate the expression and return the result value
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        let result = match self {
            ASTExpression::Terminal(expr) => expr.evaluate(tick),
            ASTExpression::Binary(expr) => expr.evaluate(tick)
        };

        // Save the result of the last evaluated expression for the `_` variable
        tick.vm.last_value = result.clone();

        result
    }
}

impl fmt::Display for ASTExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTExpression::Terminal(expr) => write!(f, "{expr}"),
            ASTExpression::Binary(expr) => write!(f, "{expr}")
        }
    }
}