use colored::Colorize;

use super::{Token, TokenLiteral, TokenSymbol};

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(value) => write!(f, "|k:{}|", value.red().bold()),
            Self::Literal(value) => write!(f, "|l:{}|", value.to_string().black()),
            Self::Symbol(value) => write!(f, "|s:{}|", value),
            Self::Semicolon => write!(f, "|{}|", "SEMICOLON".magenta()),
            Self::EOL => write!(f, "|{}|", "EOL".magenta()),
            Self::EOF => write!(f, "|{}|", "END".magenta()),
        }
    }
}

impl std::fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "[str:{}]", value.yellow().bold()),
            Self::Number(value) => write!(f, "[num:{}]", value.to_string().cyan().bold()),
        }
    }
}

impl std::fmt::Display for TokenSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "['{}']", <&str>::from(self).bright_green().bold())
    }
}