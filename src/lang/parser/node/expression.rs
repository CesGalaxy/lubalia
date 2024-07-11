pub mod literal;
pub mod operation;
pub mod variable_reference;

use literal::LiteralExpresionNode;
use operation::OperationExpressionNode;
use variable_reference::VariableReferenceNode;

use crate::{
    lang::{
        lexer::token::{Token, TokenSymbol},
        parser::{data::DataValue, exception::{ExpectedToken, ParserError, ParserException}, machine::ParsingMachine}
    },
    vm::context::Context
};

use super::{Node, NodeFactory};

pub trait ExpressionNode: Node {
    /// Evaluates the expression and returns its value.
    fn evaluate(&self, scope: &Context) -> DataValue;
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
            Some(Token::Keyword(_)) => if let Ok(_) = LiteralExpresionNode::from_tokens(&mut m.clone()) {
                Ok(Expression::Literal(LiteralExpresionNode::from_tokens(m)?))
            } else {
                Ok(Expression::VariableReference(VariableReferenceNode::from_tokens(m)?))
            },
            _ => Err(m.except(ParserException::TokenExpected(ExpectedToken::Literal("Number"))))
        }
    }
}

impl Node for Expression {}

impl NodeFactory for Expression {
    /// Try to get an expression from a vec of tokens
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        // TODO: This code is shit, fucking shit
        match m.peek() {
            Some(Token::Literal(_)) => match m.peek_next() {
                // TODO: Handle all the symbols
                Some(Token::Symbol(TokenSymbol::Plus)) => Ok(Self::Operation(OperationExpressionNode::from_tokens(m)?)),
                // TODO: If it's None it means there is no EOL/EOF
                _ => Ok(Self::Literal(LiteralExpresionNode::from_tokens(m)?))
            },
            // TODO: Variables and booleans also want to be operated :(
            Some(Token::Keyword(_)) => if let Ok(_) = LiteralExpresionNode::from_tokens(&mut m.clone()) {
                Ok(Expression::Literal(LiteralExpresionNode::from_tokens(m)?))
            } else {
                Ok(Expression::VariableReference(VariableReferenceNode::from_tokens(m)?))
            },
            _ => Err(m.except(ParserException::TokenExpected(ExpectedToken::Literal("<expr>"))))
        }
    }
}

impl ExpressionNode for Expression {
    /// Evaluate an expression
    fn evaluate(&self, scope: &Context) -> DataValue {
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
