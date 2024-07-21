pub mod display;
pub mod clasification;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Literal(TokenLiteral),
    Symbol(TokenSymbol),
    Semicolon,
    EOL,
    EOF
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenLiteral {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenSymbol {
    Equal,
    GreaterThan,
    LessThan,
    Plus,
    Minus,
    Asterisk,
    Slash,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Comma,
    Dot,
    At,
    Ampersand,
    Pipe,
    Exclamation,
    Underscore,
}

