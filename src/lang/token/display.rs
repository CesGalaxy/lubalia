use colored::Colorize;

use super::data::{TokenData, TokenLiteral, TokenSymbol};

impl std::fmt::Display for TokenData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenData::Keyword(value) => write!(f, "|k:{}|", value.red().bold()),
            TokenData::Literal(value) => write!(f, "|l:{}|", value),
            TokenData::Symbol(value) => write!(f, "|s:{}|", value),
            TokenData::Semicolon => write!(f, "|{}|", "SEMICOLON".magenta()),
            TokenData::EOL => write!(f, "|{}|", "EOL".magenta()),
            TokenData::EOF => write!(f, "|{}|", "END".magenta()),
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