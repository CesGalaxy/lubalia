use std::fmt;

/// Errors that can happen during the tokenization of the source code
#[derive(Debug, Clone)]
pub enum TokenizerError {
    // TODO: Error only when the symbol isn't a TokenSymbol
    /// There was an unexcepted symbol during the tokenization of a keyword
    UnexcepedSymbolAtKeyword(String, char),

    /// An unknown character was found
    UnknownCharacter(char),

    /// It's not possible for this to happen (or at least it shouldn't be),
    /// but this error wil be thrown if the, already checked, number can't be parsed
    ErrorParsingNumber(String),
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizerError::UnexcepedSymbolAtKeyword(keyword, symbol) => write!(f, "Unexceped symbol '{}' at keyword '{}'", symbol, keyword),
            TokenizerError::UnknownCharacter(c) => write!(f, "Unknown character '{}'", c),
            TokenizerError::ErrorParsingNumber(number) => write!(f, "Error parsing number '{}'", number),
        }
    }
}
