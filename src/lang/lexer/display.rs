use colored::Colorize;

use super::{linter::LinterError, token::{Token, TokenLiteral, TokenSymbol}, tokenizer::TokenizerError, LexerError};

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::TokenizerError(error) => write!(f, "TokenizerError >> {error}"),
            LexerError::LinterError(error) => write!(f, "LinterError >> {error}"),
        }
    }
}

impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::UnexcepedSymbolAtKeyword(keyword, c) => write!(f, "Unexcepted symbol '{c}' at keyword '{keyword}'"),
            TokenizerError::UnknownCharacter(c) => write!(f, "Unknown character '{c}'"),
            TokenizerError::ErrorParsingNumber(number) => write!(f, "Error parsing number '{number}'"),
        }
    }
}

impl std::fmt::Display for LinterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinterError::SemicolonNotAtEnd => write!(f, "Semicolon not at end"),
        }
    }
}


