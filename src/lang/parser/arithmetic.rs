use std::ops::{Add, Div, Mul, Sub};

use super::data::DataValue;

#[derive(Debug, Clone)]
pub enum ArithmeticDataValue {
    Number(f64),
    String(String)
}

impl From<DataValue> for ArithmeticDataValue {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(n) => ArithmeticDataValue::Number(n),
            DataValue::String(s) => ArithmeticDataValue::String(s),
            DataValue::Boolean(b) => ArithmeticDataValue::Number(if b { 1.0 } else { 0.0 }),
        }
    }
}

impl From<ArithmeticDataValue> for DataValue {
    fn from(value: ArithmeticDataValue) -> Self {
        match value {
            ArithmeticDataValue::Number(n) => DataValue::Number(n),
            ArithmeticDataValue::String(s) => DataValue::String(s),
        }
    }
}

impl From<ArithmeticDataValue> for String {
    fn from(value: ArithmeticDataValue) -> Self {
        match value {
            ArithmeticDataValue::Number(n) => n.to_string(),
            ArithmeticDataValue::String(s) => s,
        }
    }
}

impl Add for ArithmeticDataValue {
    type Output = ArithmeticDataValue;

    fn add(self, other: ArithmeticDataValue) -> Self::Output {
        match (self, other) {
            // If both are numbers, sum them
            (ArithmeticDataValue::Number(lhs), ArithmeticDataValue::Number(rhs)) => ArithmeticDataValue::Number(lhs + rhs),

            // If at least one is not a number, convert all to string
            (v1, v2) => ArithmeticDataValue::String(format!("{}{}", <String>::from(v1), <String>::from(v2))),
        }
    }
}

impl Sub for ArithmeticDataValue {
    type Output = ArithmeticDataValue;

    fn sub(self, other: ArithmeticDataValue) -> Self::Output {
        match (self, other) {
            // If both are numbers, subtract them
            (ArithmeticDataValue::Number(lhs), ArithmeticDataValue::Number(rhs)) => ArithmeticDataValue::Number(lhs - rhs),

            // If at least one is not a number, convert all to string
            (v1, v2) => ArithmeticDataValue::String(format!("{}-{}", <String>::from(v1), <String>::from(v2))),
        }
    }
}

impl Mul for ArithmeticDataValue {
    type Output = ArithmeticDataValue;

    fn mul(self, other: ArithmeticDataValue) -> Self::Output {
        match (self, other) {
            // If both are numbers, multiply them
            (ArithmeticDataValue::Number(lhs), ArithmeticDataValue::Number(rhs)) => ArithmeticDataValue::Number(lhs * rhs),

            // If at least one is not a number, convert all to string
            (v1, v2) => ArithmeticDataValue::String(format!("{}*{}", <String>::from(v1), <String>::from(v2))),
        }
    }
}

impl Div for ArithmeticDataValue {
    type Output = ArithmeticDataValue;

    fn div(self, other: ArithmeticDataValue) -> Self::Output {
        match (self, other) {
            // If both are numbers, divide them
            (ArithmeticDataValue::Number(lhs), ArithmeticDataValue::Number(rhs)) => ArithmeticDataValue::Number(lhs / rhs),

            // If at least one is not a number, convert all to string
            (v1, v2) => ArithmeticDataValue::String(format!("{}/{}", <String>::from(v1), <String>::from(v2))),
        }
    }
}