use std::fmt;

#[derive(Debug, Clone)]
pub enum ParserError {
    Expected(String),
    Unexpected(String)
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Expected(expected) => write!(f, "Expected {}", expected),
            ParserError::Unexpected(unexpected) => write!(f, "Unexpected {}", unexpected),
        }
    }
}
