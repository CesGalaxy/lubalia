use std::ops::{Add, Div, Mul, Sub};

use super::DataValue;

/// Arithmetic values are thoose which can be used in arithmetic operations.
#[derive(Debug, Clone)]
pub enum ArithmeticValue {
    Number(f64),
    String(String),
    List(Vec<ArithmeticValue>),
    Null
}

impl From<ArithmeticValue> for DataValue {
    fn from(value: ArithmeticValue) -> Self {
        match value {
            ArithmeticValue::Number(number) => Self::Number(number),
            ArithmeticValue::String(string) => Self::String(string),
            ArithmeticValue::List(list) => Self::List(list.into_iter().map(|item| item.into()).collect()),
            ArithmeticValue::Null => Self::Null
        }
    }
}

impl From<DataValue> for ArithmeticValue {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => Self::Number(number),
            DataValue::String(string) => Self::String(string),
            DataValue::Char(character) => Self::String(character.to_string()),
            DataValue::Boolean(boolean) => Self::Number(boolean.into()),
            DataValue::List(list) => Self::List(list.into_iter().map(|item| item.into()).collect()),
            DataValue::Null => Self::Null,
            // TODO: I'm not going to allow something as easy as this, I MUST fuck this up
            DataValue::Callable(_, _, _) => Self::Null
        }
    }
}

impl Add for ArithmeticValue {
    type Output = ArithmeticValue;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            // Same type
            (ArithmeticValue::Number(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::Number(lhs + rhs),
            (ArithmeticValue::String(lhs), ArithmeticValue::String(rhs)) => ArithmeticValue::String(lhs + &rhs),
            (ArithmeticValue::List(lhs), ArithmeticValue::List(rhs)) => ArithmeticValue::List(lhs.into_iter().chain(rhs.into_iter()).collect()),
            // TODO: Should this be here?
            (ArithmeticValue::Null, ArithmeticValue::Null) => ArithmeticValue::Null,

            // String
            (ArithmeticValue::String(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::String(lhs + &rhs.to_string()),
            (ArithmeticValue::Number(lhs), ArithmeticValue::String(rhs)) => ArithmeticValue::String(lhs.to_string() + &rhs),

            // List: Will add the item to the list (depending on the order)
            (ArithmeticValue::List(mut list), value) => {
                list.push(value);
                ArithmeticValue::List(list)
            }
            (value, ArithmeticValue::List(mut list)) => {
                list.insert(0, value);
                ArithmeticValue::List(list)
            },

            // Null will do nothing
            (Self::Null, value) | (value, Self::Null) => value,
        }
    }
}

impl Sub for ArithmeticValue {
    type Output = ArithmeticValue;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (ArithmeticValue::Number(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::Number(lhs - rhs),

            // List - Number (remove as items as the number, stop when the list is empty)
            (ArithmeticValue::List(mut list), ArithmeticValue::Number(number)) => {
                let mut new_list = Vec::new();
                let mut number = number as usize;

                while number > 0 && !list.is_empty() {
                    new_list.push(list.remove(0));
                    number -= 1;
                }

                ArithmeticValue::List(new_list)
            },

            _ => Self::Null
        }
    }
}

impl Mul for ArithmeticValue {
    type Output = ArithmeticValue;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (ArithmeticValue::Number(lhs), ArithmeticValue::Number(rhs)) => ArithmeticValue::Number(lhs * rhs),

            // value * number (repeat the value as many times as the number)
            (value, ArithmeticValue::Number(number)) => {
                let mut list = Vec::new();
                let number = number as usize;

                for _ in 0..number {
                    list.push(value.clone());
                }

                ArithmeticValue::List(list)
            },
            (ArithmeticValue::Number(number), value) => {
                let mut list = Vec::new();
                let number = number as usize;

                for _ in 0..number {
                    list.push(value.clone());
                }

                ArithmeticValue::List(list)
            },

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