use crate::{
    lang::{
        lexer::token::{Token, TokenSymbol},
        parser::{
            data::DataValue,
            exception::{ExcpectedToken, ParserError, ParserException},
            machine::ParsingMachine,
            node::{Node, NodeFactory}
        }
    },
    vm::scope::Scope
};

use super::{Expression, ExpressionNode};

pub type ArithmeticalExpression = (Vec<(Expression, ArithmeticalOperation)>, DataValue);

#[derive(Debug)]
pub enum ArithmeticalOperation {
    Add,
    Sub,
    Mul,
    Div
}

pub fn get_expression_segment(m: &mut ParsingMachine) -> Result<Option<(Expression, ArithmeticalOperation)>, ParserError> {
    let n1 = Expression::get_value(m)?;

    if let Some(Token::Symbol(symbol)) = m.consume() {
        match symbol {
            TokenSymbol::Plus => Ok(Some((n1, ArithmeticalOperation::Add))),
            TokenSymbol::Minus => Ok(Some((n1, ArithmeticalOperation::Sub))),
            TokenSymbol::Asterisk => Ok(Some((n1, ArithmeticalOperation::Mul))),
            TokenSymbol::Slash => Ok(Some((n1, ArithmeticalOperation::Div))),
            _ => return Err(m.except(ParserException::TokenExpected(ExcpectedToken::Symbol("<operand>")))),
        }
    } else {
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub enum OperationExpressionNode {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>)
}

impl Node for OperationExpressionNode {}

impl NodeFactory for OperationExpressionNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        let segment = get_expression_segment(m)?.ok_or(m.except(ParserException::TokenExpected(ExcpectedToken::Symbol("<operation first segment>"))))?;
        
        // let mut segments = Vec::new();

        // while let Some(segment) = get_expression_segment(m)? {
        //     segments.push(segment);
        // }

        // let end = Expression::from_tokens(m)?;

        Ok(match segment.1 {
            ArithmeticalOperation::Add => Self::Add(Box::new(segment.0), Box::new(Expression::from_tokens(m)?)),
            ArithmeticalOperation::Sub => Self::Sub(Box::new(segment.0), Box::new(Expression::from_tokens(m)?)),
            ArithmeticalOperation::Mul => Self::Mul(Box::new(segment.0), Box::new(Expression::from_tokens(m)?)),
            ArithmeticalOperation::Div => Self::Div(Box::new(segment.0), Box::new(Expression::from_tokens(m)?)),
        })
    }
}

impl ExpressionNode for OperationExpressionNode {
    /// Operates the values
    fn evaluate(&self, scope: &Scope) -> DataValue {
        match self {
            OperationExpressionNode::Add(a, b) => a.evaluate(scope) + b.evaluate(scope),
            OperationExpressionNode::Sub(a, b) => a.evaluate(scope) - b.evaluate(scope),
            OperationExpressionNode::Mul(a, b) => a.evaluate(scope) * b.evaluate(scope),
            OperationExpressionNode::Div(a, b) => a.evaluate(scope) / b.evaluate(scope),
        }
    }
}

impl std::fmt::Display for OperationExpressionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationExpressionNode::Add(a, b) => write!(f, "{} + {}", a, b),
            OperationExpressionNode::Sub(a, b) => write!(f, "{} - {}", a, b),
            OperationExpressionNode::Mul(a, b) => write!(f, "{} * {}", a, b),
            OperationExpressionNode::Div(a, b) => write!(f, "{} / {}", a, b),
        }
    }
}
