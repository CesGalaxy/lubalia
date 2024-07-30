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
            BinaryOperator::Equal => (lhs == rhs).into(),
            BinaryOperator::Greater => (lhs > rhs).into(),
            BinaryOperator::Less => (lhs < rhs).into(),
            BinaryOperator::AND => (bool::from(lhs) && bool::from(rhs)).into(),
            BinaryOperator::OR => (bool::from(lhs) || bool::from(rhs)).into(),
            BinaryOperator::NAND => (!bool::from(lhs) || !bool::from(rhs)).into(),
            BinaryOperator::NOR => (!bool::from(lhs) && !bool::from(rhs)).into(),
            BinaryOperator::XOR => (bool::from(lhs) ^ bool::from(rhs)).into(),
            BinaryOperator::XNOR => (bool::from(lhs) ^ bool::from(rhs)).not().into()
        }
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {:?} {} )", self.lhs, self.operator, self.rhs)
    }
}