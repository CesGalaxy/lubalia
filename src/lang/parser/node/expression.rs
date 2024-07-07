pub mod literal;
pub mod operation;

use literal::LiteralExpresionNode;
use operation::OperationExpressionNode;

use crate::{lang::{lexer::token::Token, parser::{data::DataValue, exception::{ExcpectedToken, ParsingMachineError, ParsingMachineException}, machine::ParsingMachine}}, vm::scope::Scope};

use super::{Node, NodeFactory};

pub trait ExpressionNode: Node {
    /// Evaluates the expression and returns its value.
    fn evaluate(&self, scope: &Scope) -> DataValue;
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpresionNode),
    Operation(OperationExpressionNode),
}

impl ExpressionNode for Expression {
    fn evaluate(&self, scope: &Scope) -> DataValue {
        match self {
            Expression::Literal(node) => node.evaluate(scope),
            Expression::Operation(node) => node.evaluate(scope)
        }
    }
}

impl Node for Expression {}

impl NodeFactory for Expression {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParsingMachineError> {
        if let Some(Token::Literal(_)) = m.peek() {
            if let Some(Token::Symbol(_)) = m.peek_next() {
                Ok(Self::Operation(OperationExpressionNode::from_tokens(m)?))
            } else {
                Ok(Self::Literal(LiteralExpresionNode::from_tokens(m)?))
            }
        } else {
            Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Literal("Number"))))
        }
    }
}