use std::ops::{Add, Div, Mul, Sub};

use super::DataValue;

/// Arithmetic values are thoose which can be used in arithmetic operations.
pub enum ArithmeticValue {
    Number(f64),
    String(String),
    Null
}

impl From<ArithmeticValue> for DataValue {
    fn from(value: ArithmeticValue) -> Self {
        match value {
            ArithmeticValue::Number(number) => DataValue::Number(number),
            ArithmeticValue::String(string) => DataValue::String(string),
            ArithmeticValue::Null => DataValue::Null
        }
    }
}

impl From<DataValue> for ArithmeticValue {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => ArithmeticValue::Number(number),
            DataValue::String(string) => ArithmeticValue::String(string),
            DataValue::Boolean(boolean) => ArithmeticValue::Number(boolean.into()),
            DataValue::Null => ArithmeticValue::Null
        }
    }
}

impl Add for ArithmeticValue {
    type Output = ArithmeticValue;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (ArithmeticValue::Number(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::Number(lhs + rhs),
            (ArithmeticValue::String(lhs), ArithmeticValue::String(rhs)) => ArithmeticValue::String(lhs + &rhs),
            (ArithmeticValue::String(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::String(lhs + &rhs.to_string()),
            (ArithmeticValue::Number(lhs), ArithmeticValue::String(rhs)) => ArithmeticValue::String(lhs.to_string() + &rhs),
            _ => Self::Null
        }
    }
}

impl Sub for ArithmeticValue {
    type Output = ArithmeticValue;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (ArithmeticValue::Number(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::Number(lhs - rhs),
            _ => Self::Null
        }
    }
}

impl Mul for ArithmeticValue {
    type Output = ArithmeticValue;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (ArithmeticValue::Number(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::Number(lhs * rhs),
            _ => Self::Null
        }
    }
}

impl Div for ArithmeticValue {
    type Output = ArithmeticValue;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (ArithmeticValue::Number(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::Number(lhs / rhs),
            _ => Self::Null
        }
    }
}