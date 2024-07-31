use std::fmt;

/// Errors that can happen during the tokenization of the source code
#[derive(Debug, Clone)]
pub enum TokenizerError {
    /// An unexpected symbol was found, can provide an expected symbol
    UnexpectedSymbol(char, Option<&'static str>),

    /// The end of the code was reached unexpectedly
    UnexpectedEnd,

    /// An unknown character was found
    UnknownCharacter(char),

    /// It's not possible for this to happen (or at least it shouldn't be),
    /// but this error wil be thrown if the, already checked, number can't be parsed
    ErrorParsingNumber(String),
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizerError::UnexpectedSymbol(symbol, expected) => if let Some(expected) = expected {
                write!(f, "Unexpected symbol '{symbol}', expected '{expected}'")
            } else {
                write!(f, "Unexpected symbol '{symbol}'")
            },
            TokenizerError::UnexpectedEnd => write!(f, "Unexpected end of the code"),
            TokenizerError::UnknownCharacter(c) => write!(f, "Unknown character '{}'", c),
            TokenizerError::ErrorParsingNumber(number) => write!(f, "Error parsing number '{}'", number),
        }
    }
}
