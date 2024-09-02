use std::fmt;

#[derive(Debug, Clone)]
pub enum ParserError {
    Expected(&'static str),
    UnexpectedEnd,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Expected(expected) => write!(f, "Expected {}", expected),
            ParserError::UnexpectedEnd => write!(f, "Unexpected end of input"),
        }
    }
}
