use std::ops::{Add, Div, Mul, Sub};

use super::DataValue;

impl Add for DataValue {
    type Output = DataValue;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (DataValue::Number(lhs), DataValue::Number(rhs)) => DataValue::Number(lhs + rhs),
            (DataValue::String(lhs), DataValue::String(rhs)) => DataValue::String(lhs + &rhs),
            _ => Self::Null
        }
    }
}

impl Sub for DataValue {
    type Output = DataValue;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (DataValue::Number(lhs), DataValue::Number(rhs)) => DataValue::Number(lhs - rhs),
            _ => Self::Null
        }
    }
}

impl Mul for DataValue {
    type Output = DataValue;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (DataValue::Number(lhs), DataValue::Number(rhs)) => DataValue::Number(lhs * rhs),
            _ => Self::Null
        }
    }
}

impl Div for DataValue {
    type Output = DataValue;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (DataValue::Number(lhs), DataValue::Number(rhs)) => DataValue::Number(lhs / rhs),
            _ => Self::Null
        }
    }
}