pub mod literal;
pub mod operation;
pub mod variable_reference;

use literal::LiteralExpresionNode;
use operation::OperationExpressionNode;
use variable_reference::VariableReferenceNode;

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
    VariableReference(VariableReferenceNode)
}

impl Expression {
    fn get_value(m: &mut ParsingMachine) -> Result<Expression, ParserError> {
        match m.peek() {
            Some(Token::Literal(_)) => Ok(Expression::Literal(LiteralExpresionNode::from_tokens(m)?)),
            Some(Token::Keyword(_)) => Ok(Expression::VariableReference(VariableReferenceNode::from_tokens(m)?)),
            _ => Err(m.except(ParserException::TokenExpected(ExcpectedToken::Literal("Number"))))
        }
    }
}

impl Node for Expression {}

impl NodeFactory for Expression {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        match m.peek() {
            Some(Token::Literal(_)) => if let Some(Token::Symbol(_)) = m.peek_next() {
                Ok(Self::Operation(OperationExpressionNode::from_tokens(m)?))
            } else {
                Ok(Self::Literal(LiteralExpresionNode::from_tokens(m)?))
            },
            Some(Token::Keyword(_)) => {
                Ok(Self::VariableReference(VariableReferenceNode::from_tokens(m)?))
            },
            _ => Err(m.except(ParserException::TokenExpected(ExcpectedToken::Literal("Number"))))
        }
    }
}

impl ExpressionNode for Expression {
    fn evaluate(&self, scope: &Scope) -> DataValue {
        match self {
            Expression::Literal(node) => node.evaluate(scope),
            Expression::Operation(node) => node.evaluate(scope),
            Expression::VariableReference(node) => node.evaluate(scope),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(node) => write!(f, "{}", node),
            Expression::Operation(node) => write!(f, "{}", node),
            Expression::VariableReference(node) => write!(f, "{}", node),
        }
    }
}
