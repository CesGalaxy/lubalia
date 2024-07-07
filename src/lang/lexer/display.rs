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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(value) => write!(f, "|k:{}|", value.red().bold()),
            Token::Literal(value) => write!(f, "|l:{}|", value),
            Token::Symbol(value) => write!(f, "|s:{}|", value),
            Token::Semicolon => write!(f, "|{}|", ";".magenta()),
            Token::EOL => write!(f, "|{}|", "EOL".magenta()),
            Token::EOF => write!(f, "|{}|", "END".magenta()),
        }
    }
}

impl std::fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenLiteral::String(value) => write!(f, "[str:{}]", value.yellow().bold()),
            TokenLiteral::Number(value) => write!(f, "[num:{}]", value.to_string().cyan().bold()),
        }
    }
}

impl std::fmt::Display for TokenSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "['{}']", <&str>::from(self).bright_green().bold())
    }
}
