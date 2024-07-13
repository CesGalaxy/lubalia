/// IMPORTANT NOTE
/// 
/// God knows how and why this works, I never did.
/// I'm too lazy for searching for a tutorial about operations order
/// If there is any weird combination that doesn't work, shut up and make a commit, I will NOT fix it.
/// 
/// César ~ 2024 (3h spent here, for now)

// TODO: This doesn't work

use std::ops::{Add, Div, Mul, Sub};

use crate::{
    lang::{
        token::{Token, TokenSymbol},
        parser::{
            arithmetic::ArithmeticDataValue, data::DataValue, exception::{ExpectedToken, ParserError, ParserException}, machine::ParsingMachine, node::{Node, NodeFactory}
        }
    },
    vm::context::Context
};

use super::{Expression, ExpressionNode};

pub type ArithmeticalExpression = (Expression, ArithmeticalOperation, Expression);
pub type ArithmeticalExpressionSecuence = (Vec<(Expression, ArithmeticalOperation)>, DataValue);

#[derive(Debug, PartialEq)]
pub enum ArithmeticalOperation {
    Add,
    Sub,
    Mul,
    Div
}

impl PartialOrd for ArithmeticalOperation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            match (self, other) {
                (ArithmeticalOperation::Mul, ArithmeticalOperation::Div) => Some(std::cmp::Ordering::Equal),
                (ArithmeticalOperation::Div, ArithmeticalOperation::Mul) => Some(std::cmp::Ordering::Equal),
                (ArithmeticalOperation::Add, ArithmeticalOperation::Sub) => Some(std::cmp::Ordering::Equal),
                (ArithmeticalOperation::Sub, ArithmeticalOperation::Add) => Some(std::cmp::Ordering::Equal),
                (ArithmeticalOperation::Mul, _) | (ArithmeticalOperation::Div, _) => Some(std::cmp::Ordering::Greater),
                (ArithmeticalOperation::Add, _) | (ArithmeticalOperation::Sub, _) => Some(std::cmp::Ordering::Less),
            }
        }
    }
}

pub fn get_expression_segment(m: &mut ParsingMachine) -> Result<Option<(Expression, ArithmeticalOperation)>, ParserError> {
    let n1 = Expression::get_value(m)?;

    if let Some(Token::Symbol(symbol)) = m.consume() {
        match symbol {
            TokenSymbol::Plus => Ok(Some((n1, ArithmeticalOperation::Add))),
            TokenSymbol::Minus => Ok(Some((n1, ArithmeticalOperation::Sub))),
            TokenSymbol::Asterisk => Ok(Some((n1, ArithmeticalOperation::Mul))),
            TokenSymbol::Slash => Ok(Some((n1, ArithmeticalOperation::Div))),
            _ => return Err(m.err(ParserException::TokenExpected(ExpectedToken::Symbol("<operand>")))),
        }
    } else {
        // TODO: Change this
        m.back();
        m.back();
        Ok(None)
    }
}

pub fn operation_from_segment(segment: (Expression, ArithmeticalOperation), end: Expression) -> OperationExpressionNode {
    match segment.1 {
        ArithmeticalOperation::Add => OperationExpressionNode::Add(Box::new(segment.0), Box::new(end)),
        ArithmeticalOperation::Sub => OperationExpressionNode::Sub(Box::new(segment.0), Box::new(end)),
        ArithmeticalOperation::Mul => OperationExpressionNode::Mul(Box::new(segment.0), Box::new(end)),
        ArithmeticalOperation::Div => OperationExpressionNode::Div(Box::new(segment.0), Box::new(end)),
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
        let segment = get_expression_segment(m)?
            .ok_or(m.err(ParserException::TokenExpected(ExpectedToken::Symbol("<operation first segment>"))))?;
        
        if let Some(segment2) = get_expression_segment(m)? {
            if segment.1 < segment2.1 { // a + b * ...
                // println!("HELLO F {:?} - {:?}", &segment, &segment2);
                let third_expr = Expression::from_tokens(m)?;
                let second_expr = Expression::Operation(operation_from_segment(segment2, third_expr));
                Ok(operation_from_segment(segment, second_expr))
            } else { // segment > segment2 || segment == segment2    a * b + ...
                let first_expr = Expression::Operation(operation_from_segment(segment, segment2.0));
                let operation = operation_from_segment((first_expr, segment2.1), Expression::from_tokens(m)?);

                Ok(operation)
            }
        } else {
            let a = Expression::from_tokens(m)?;
            Ok(operation_from_segment(segment, a))
        }
    }
}

impl ExpressionNode for OperationExpressionNode {
    /// Operates the values
    fn evaluate(&self, scope: &Context) -> DataValue {
        match self {
            OperationExpressionNode::Add(a, b) => ArithmeticDataValue::add(a.evaluate(scope).into(), b.evaluate(scope).into()),
            OperationExpressionNode::Sub(a, b) => ArithmeticDataValue::sub(a.evaluate(scope).into(), b.evaluate(scope).into()),
            OperationExpressionNode::Mul(a, b) => ArithmeticDataValue::mul(a.evaluate(scope).into(), b.evaluate(scope).into()),
            OperationExpressionNode::Div(a, b) => ArithmeticDataValue::div(a.evaluate(scope).into(), b.evaluate(scope).into()),
        }.into()
    }
}

impl std::fmt::Display for OperationExpressionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationExpressionNode::Add(a, b) => write!(f, "( {} + {} )", a, b),
            OperationExpressionNode::Sub(a, b) => write!(f, "( {} - {} )", a, b),
            OperationExpressionNode::Mul(a, b) => write!(f, "( {} * {} )", a, b),
            OperationExpressionNode::Div(a, b) => write!(f, "( {} / {} )", a, b),
        }
    }
}
