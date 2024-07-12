#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
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
    At
}

impl From<&TokenSymbol> for &str {
    fn from(value: &TokenSymbol) -> Self {
        match value {
            TokenSymbol::Equal => "=",
            TokenSymbol::GreaterThan => ">",
            TokenSymbol::LessThan => "<",
            TokenSymbol::Plus => "+",
            TokenSymbol::Minus => "-",
            TokenSymbol::Asterisk => "*",
            TokenSymbol::Slash => "/",
            TokenSymbol::ParenOpen => "(",
            TokenSymbol::ParenClose => ")",
            TokenSymbol::BraceOpen => "{",
            TokenSymbol::BraceClose => "}",
            TokenSymbol::BracketOpen => "[",
            TokenSymbol::BracketClose => "]",
            TokenSymbol::Comma => ",",
            TokenSymbol::Dot => ".",
            TokenSymbol::At => "@",
        }
    }
}

impl TokenSymbol {
    pub fn from_char(value: char) -> Option<Self> {
        match value {
            '=' => Some(TokenSymbol::Equal),
            '>' => Some(TokenSymbol::GreaterThan),
            '<' => Some(TokenSymbol::LessThan),
            '+' => Some(TokenSymbol::Plus),
            '-' => Some(TokenSymbol::Minus),
            '*' => Some(TokenSymbol::Asterisk),
            '/' => Some(TokenSymbol::Slash),
            '(' => Some(TokenSymbol::ParenOpen),
            ')' => Some(TokenSymbol::ParenClose),
            '{' => Some(TokenSymbol::BraceOpen),
            '}' => Some(TokenSymbol::BraceClose),
            '[' => Some(TokenSymbol::BracketOpen),
            ']' => Some(TokenSymbol::BracketClose),
            ',' => Some(TokenSymbol::Comma),
            '.' => Some(TokenSymbol::Dot),
            '@' => Some(TokenSymbol::At),
            _ => None,
        }
    }
}