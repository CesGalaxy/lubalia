mod operator;
mod transcription;

use std::{fmt, ops::Not};

use operator::BinaryOperator;

use crate::{data::{arithmetic::ArithmeticValue, DataValue}, vm::tick::VMTick};

use super::{ASTExpression, ExpressionNode};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BinaryExpression {
    lhs: Box<ASTExpression>,
    operator: BinaryOperator,
    rhs: Box<ASTExpression>
}

impl ExpressionNode for BinaryExpression {
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        let lhs = self.lhs.evaluate(tick);
        let rhs = self.rhs.evaluate(tick);

        match self.operator {
            BinaryOperator::Add => (ArithmeticValue::from(lhs) + ArithmeticValue::from(rhs)).into(),
            BinaryOperator::Sub => (ArithmeticValue::from(lhs) - ArithmeticValue::from(rhs)).into(),
            BinaryOperator::Mul => (ArithmeticValue::from(lhs) * ArithmeticValue::from(rhs)).into(),
            BinaryOperator::Div => (ArithmeticValue::from(lhs) / ArithmeticValue::from(rhs)).into(),
            BinaryOperator::Equal => DataValue::Boolean(lhs == rhs),
            BinaryOperator::NoEqual => DataValue::Boolean(lhs != rhs),
            BinaryOperator::Greater => DataValue::Boolean(lhs > rhs),
            BinaryOperator::GreaterOrEqual => DataValue::Boolean(lhs >= rhs),
            BinaryOperator::Less => DataValue::Boolean(lhs < rhs),
            BinaryOperator::LessOrEqual => DataValue::Boolean(lhs <= rhs),
            BinaryOperator::AND => DataValue::Boolean(bool::from(lhs) && bool::from(rhs)),
            BinaryOperator::OR => DataValue::Boolean(bool::from(lhs) || bool::from(rhs)),
            BinaryOperator::NAND => DataValue::Boolean(!bool::from(lhs) || !bool::from(rhs)),
            BinaryOperator::NOR => DataValue::Boolean(!bool::from(lhs) && !bool::from(rhs)),
            BinaryOperator::XOR => DataValue::Boolean(bool::from(lhs) ^ bool::from(rhs)),
            BinaryOperator::XNOR => DataValue::Boolean(bool::from(lhs) ^ bool::from(rhs).not())
        }
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {:?} {} )", self.lhs, self.operator, self.rhs)
    }
}