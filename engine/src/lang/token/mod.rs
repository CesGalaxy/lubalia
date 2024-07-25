pub mod symbol;
pub mod literal;
pub mod keyword;

use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    CustomKeyword(String),
    LangKeyword(keyword::TokenLangKeyword),
    Literal(literal::TokenLiteral),
    Symbol(symbol::TokenSymbol),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustomKeyword(value) => write!(f, "|k:{}|", value.red().bold()),
            Self::LangKeyword(value) => write!(f, "|k:{value}|"),
            Self::Literal(value) => write!(f, "|l:{value}|"),
            Self::Symbol(symbol::TokenSymbol::Semicolon) => write!(f, "|{}|", "SEMICOLON".magenta()),
            Self::Symbol(symbol::TokenSymbol::EOL) => write!(f, "|{}|", "EOL".magenta()),
            Self::Symbol(symbol::TokenSymbol::EOF) => write!(f, "|{}|", "END"),
            Self::Symbol(value) => write!(f, "|s:{value}|"),
        }
    }
}

pub fn is_built_in_keyword(value: &str) -> bool {
    match value {
        "let" | "const" | "fn" | "if" | "else" | "return" | "true" | "false" | "null" | "undefined" => true,
        _ => false,
    }
}
