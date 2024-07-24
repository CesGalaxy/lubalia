/// This module contains the arithmetic operations for the language, with their corresponding data values.
pub mod arithmetic;

/// This module contains the logic operations for the language, with their corresponding data values.
pub mod logic;

/// This module contains the comparasion operations for the language data values.
pub mod comparasion;

use colored::Colorize;

use crate::lang::token::literal::TokenLiteral;

/// Represents a posible data value which the language can work with.
#[derive(Debug, Clone)]
pub enum DataValue {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<DataValue>),
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

impl std::fmt::Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValue::Number(number) => write!(f, "{}", number.to_string().bright_blue()),
            DataValue::String(string) => write!(f, "\"{}\"", string.yellow()),
            DataValue::Boolean(boolean) => write!(f, "{}", boolean.to_string().bright_green()),
            DataValue::List(list) => {
                let mut list_str = String::new();
                list_str.push_str("[");

                for item in list {
                    list_str.push_str(&format!("{item}, "));
                }

                list_str.push_str("]");
                write!(f, "{}", list_str)
            },
            DataValue::Null => write!(f, "{}", "NULL".bright_red()),
        }
    }
}
