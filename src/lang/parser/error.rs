#[derive(Debug, Clone)]
pub enum ParserError {
    Expected(String),
    Unexpected(String)
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Expected(expected) => write!(f, "Expected {}", expected),
            ParserError::Unexpected(unexpected) => write!(f, "Unexpected {}", unexpected),
        }
    }
}