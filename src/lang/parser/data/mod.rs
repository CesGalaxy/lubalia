/// This module contains the arithmetic operations for the language, with their corresponding data values.
pub mod arithmetic;
/// This module contains the logic operations for the language, with their corresponding data values.
pub mod logic;

use crate::lang::token::TokenLiteral;

/// Represents a posible data value which the language can work with.
#[derive(Debug, Clone)]
pub enum DataValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null
}

impl From<TokenLiteral> for DataValue {
    fn from(literal: TokenLiteral) -> Self {
        match literal {
            TokenLiteral::Number(number) => DataValue::Number(number),
            TokenLiteral::String(string) => DataValue::String(string),
        }
    }
}

impl Default for DataValue {
    fn default() -> Self {
        DataValue::Null
    }
}

impl PartialEq for DataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Same type
            (DataValue::Number(a), DataValue::Number(b)) => a == b,
            (DataValue::String(a), DataValue::String(b)) => a == b,
            (DataValue::Boolean(a), DataValue::Boolean(b)) => a == b,
            (DataValue::Null, DataValue::Null) => true,
            
            // Different types
            // String - Number
            (DataValue::Number(a), DataValue::String(b)) => &a.to_string() == b,
            (DataValue::String(a), DataValue::Number(b)) => a == &b.to_string(),

            // String - Boolean
            (DataValue::Boolean(a), DataValue::String(b)) => &a.to_string() == b,
            (DataValue::String(a), DataValue::Boolean(b)) => a == &b.to_string(),

            // Number - Boolean
            (DataValue::Number(a), DataValue::Boolean(b)) => a == &f64::from(*b),
            (DataValue::Boolean(a), DataValue::Number(b)) => &f64::from(*a) == b,

            // Null is always equal to Null
            (DataValue::Null, _) | (_, DataValue::Null) => false,
        }
    }
}

impl PartialOrd for DataValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            // Same type
            (DataValue::Number(a), DataValue::Number(b)) => a.partial_cmp(b).unwrap(),
            (DataValue::String(a), DataValue::String(b)) => a.cmp(b),
            (DataValue::Boolean(a), DataValue::Boolean(b)) => a.cmp(b),
            (DataValue::Null, DataValue::Null) => std::cmp::Ordering::Equal,

            // Different types
            // String - Number
            (DataValue::Number(a), DataValue::String(b)) => a.to_string().cmp(b),
            (DataValue::String(a), DataValue::Number(b)) => a.cmp(&b.to_string()),

            // String - Boolean
            (DataValue::Boolean(a), DataValue::String(b)) => a.to_string().cmp(b),
            (DataValue::String(a), DataValue::Boolean(b)) => a.cmp(&b.to_string()),

            // Number - Boolean
            (DataValue::Number(a), DataValue::Boolean(b)) => a.partial_cmp(&f64::from(*b)).unwrap_or(std::cmp::Ordering::Equal),
            (DataValue::Boolean(a), DataValue::Number(b)) => f64::from(*a).partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),

            // Null is always less than any other type
            (DataValue::Null, _) => std::cmp::Ordering::Less,
            (_, DataValue::Null) => std::cmp::Ordering::Greater,
        })
    }
}