pub mod literal;
pub mod operation;

use literal::LiteralExpresionNode;
use operation::OperationExpressionNode;

use crate::{
    lang::{
        lexer::token::Token,
        parser::{data::DataValue, exception::{ExcpectedToken, ParserError, ParserException}, machine::ParsingMachine}
    },
    vm::scope::Scope
};

use super::{Node, NodeFactory};

pub trait ExpressionNode: Node {
    /// Evaluates the expression and returns its value.
    fn evaluate(&self, scope: &Scope) -> DataValue;
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(LiteralExpresionNode),
    Operation(OperationExpressionNode),
}

impl Node for Expression {}

impl NodeFactory for Expression {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        if let Some(Token::Literal(_)) = m.peek() {
            if let Some(Token::Symbol(_)) = m.peek_next() {
                Ok(Self::Operation(OperationExpressionNode::from_tokens(m)?))
            } else {
                Ok(Self::Literal(LiteralExpresionNode::from_tokens(m)?))
            }
        } else {
            Err(m.except(ParserException::TokenExpected(ExcpectedToken::Literal("Number"))))
        }
    }
}

impl ExpressionNode for Expression {
    fn evaluate(&self, scope: &Scope) -> DataValue {
        match self {
            Expression::Literal(node) => node.evaluate(scope),
            Expression::Operation(node) => node.evaluate(scope)
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(node) => write!(f, "{}", node),
            Expression::Operation(node) => write!(f, "{}", node),
        }
    }
}
