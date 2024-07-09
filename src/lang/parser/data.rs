use colored::Colorize;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub enum DataValue {
    String(String),
    Number(f64),
    Boolean(bool)
}

impl From<DataValue> for String {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::String(s) => s,
            DataValue::Number(n) => n.to_string(),
            DataValue::Boolean(b) => b.to_string()
        }
    }
}

impl std::fmt::Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValue::Number(n) => write!(f, "{}", n.to_string().cyan()),
            DataValue::String(s) => write!(f, "{}{}{}", "'".black(), s.yellow(), "'".black()),
            DataValue::Boolean(b) => write!(f, "{}", b.to_string().cyan())
        }
    }
}

// TODO: ArithmeticalDataValue
impl Add for DataValue {
    type Output = DataValue;

    fn add(self, other: DataValue) -> Self::Output {
        match (self, other) {
            // If both are numbers, sum them
            (DataValue::Number(lhs), DataValue::Number(rhs)) => DataValue::Number(lhs + rhs),

            // If at least one is not a number, convert all to string
            (v1, v2) => DataValue::String(format!("{}{}", <String>::from(v1.clone()), <String>::from(v2.clone()))),
        }
    }
}

impl Sub for DataValue {
    type Output = DataValue;

    fn sub(self, other: DataValue) -> Self::Output {
        match (self, other) {
            // If both are numbers, subtract them
            (DataValue::Number(lhs), DataValue::Number(rhs)) => DataValue::Number(lhs - rhs),

            // If at least one is not a number, convert all to string
            (v1, v2) => DataValue::String(format!("{}-{}", <String>::from(v1.clone()), <String>::from(v2.clone()))),
        }
    }
}

impl Mul for DataValue {
    type Output = DataValue;

    fn mul(self, other: DataValue) -> Self::Output {
        match (self, other) {
            // If both are numbers, multiply them
            (DataValue::Number(lhs), DataValue::Number(rhs)) => DataValue::Number(lhs * rhs),

            // If at least one is not a number, convert all to string
            (v1, v2) => DataValue::String(format!("{}*{}", <String>::from(v1.clone()), <String>::from(v2.clone()))),
        }
    }
}

impl Div for DataValue {
    type Output = DataValue;

    fn div(self, other: DataValue) -> Self::Output {
        match (self, other) {
            // If both are numbers, divide them
            (DataValue::Number(lhs), DataValue::Number(rhs)) => DataValue::Number(lhs / rhs),

            // If at least one is not a number, convert all to string
            (v1, v2) => DataValue::String(format!("{}/{}", <String>::from(v1.clone()), <String>::from(v2.clone()))),
        }
    }
}