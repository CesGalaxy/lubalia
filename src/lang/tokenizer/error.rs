#[derive(Debug, Clone)]
pub enum TokenizerError {
    UnexcepedSymbolAtKeyword(String, char),
    UnknownCharacter(char),
    ErrorParsingNumber(String),
}

impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::UnexcepedSymbolAtKeyword(keyword, symbol) => write!(f, "Unexceped symbol '{}' at keyword '{}'", symbol, keyword),
            TokenizerError::UnknownCharacter(c) => write!(f, "Unknown character '{}'", c),
            TokenizerError::ErrorParsingNumber(number) => write!(f, "Error parsing number '{}'", number),
        }
    }
}
