/// This module contains the arithmetic operations for the language, with their corresponding data values.
pub mod arithmetic;

/// This module contains the comparasion operations for the language data values.
pub mod comparsion;

/// This module contains converters between different (language and machine) data types.
pub mod conversion;

/// This module contains the typing system.
pub mod types;

use std::fmt;

use colored::Colorize;

use crate::{lang::token::literal::TokenLiteral, node::ASTNode};

/// Represents a posible data value which the language can work with.
#[derive(Debug, Clone)]
pub enum DataValue {
    Number(f64),
    String(String),
    Char(char),
    Boolean(bool),
    List(Vec<DataValue>),
    Null,
    Callable(Vec<String>, Vec<(String, DataValue)>, Box<ASTNode>),
}

impl From<TokenLiteral> for DataValue {
    fn from(literal: TokenLiteral) -> Self {
        match literal {
            TokenLiteral::Number(number) => DataValue::Number(number),
            TokenLiteral::String(string) => DataValue::String(string),
            TokenLiteral::Character(character) => DataValue::Char(character),
            // TODO: This shouldn't even be a literal
            TokenLiteral::Identifier(_) => DataValue::Null,
        }
    }
}

impl Default for DataValue {
    fn default() -> Self {
        DataValue::Null
    }
}

impl fmt::Display for DataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataValue::Number(number) => write!(f, "{}", number.to_string().bright_blue()),
            DataValue::String(string) => write!(f, "\"{}\"", string.yellow()),
            DataValue::Char(character) => write!(f, "'{}'", character.to_string().bright_green()),
            DataValue::Boolean(boolean) => write!(f, "{}", boolean.to_string().bright_green()),
            DataValue::List(list) => {
                let mut list_str = String::new();

                for item in list {
                    list_str.push_str(&format!("{item}, "));
                }

                // Remove the last comma and space
                list_str.pop();
                list_str.pop();

                write!(f, "[ {} ]", list_str)
            },
            DataValue::Null => write!(f, "{}", "NULL".bright_red()),
            DataValue::Callable(required_args, optional_args, body) =>
                // TODO: This should be a lot more detailed
                write!(f, "{}{required_args:?}{optional_args:?} {body}", "callable".bright_red().bold()),
        }
    }
}
