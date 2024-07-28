pub mod terminal;
pub mod binary;

use lubalia_utils::transcriber::cursor::TranscriberCursor;

use crate::{
    data::DataValue,
    lang::{parser::error::ParserError, token::Token},
    vm::tick::VMTick
};

use super::Node;

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
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTExpression>, ParserError> {
        //* Expressions shouldn't return an Err if nothing could be transcribed, should them?
        if cursor.peek_next().is_some_and(Token::is_operator) {
            binary::BinaryExpression::transcribe(cursor).map(|bexpr| bexpr.map(ASTExpression::Binary))
        } else {
            Ok(
                terminal::TerminalExpression::transcribe(cursor)?
                    //.unwrap_or(None)
                    .map(ASTExpression::Terminal)
            )
        }
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

impl std::fmt::Display for ASTExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTExpression::Terminal(expr) => write!(f, "{}", expr),
            ASTExpression::Binary(expr) => write!(f, "{}", expr)
        }
    }
}