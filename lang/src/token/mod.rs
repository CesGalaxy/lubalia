pub mod symbol;
pub mod literal;
pub mod keyword;
pub mod identifier;

use colored::Colorize;

/// A token is the smallest unit for building a program.
/// Can be obtained from a source code.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A keyword that is not build-in
    Identifier(String),

    /// A keyword that is reserved for the language grammar
    Keyword(keyword::TokenLangKeyword),

    /// A literal value provided in the code
    Literal(literal::TokenLiteral),

    /// A symbol part of the language grammar
    Symbol(symbol::TokenSymbol),

    /// For naming things
    Tag(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(value) => write!(f, "|k:{}|", value.red().bold()),
            Self::Keyword(value) => write!(f, "|k:{value}|"),
            Self::Literal(value) => write!(f, "|l:{value}|"),
            Self::Symbol(symbol::TokenSymbol::Semicolon) => write!(f, "|{}|", "SEMICOLON".magenta()),
            Self::Symbol(symbol::TokenSymbol::EOL) => write!(f, "|{}|", "EOL".magenta()),
            Self::Symbol(symbol::TokenSymbol::EOF) => write!(f, "|{}|", "END"),
            Self::Symbol(value) => write!(f, "|s:{value}|"),
            Self::Tag(value) => write!(f, "|t:{}|", value.blue().bold()),
        }
    }
}
