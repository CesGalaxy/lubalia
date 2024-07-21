use super::TokenSymbol;

pub fn is_built_in_keyword(value: &str) -> bool {
    match value {
        "let" | "const" | "fn" | "if" | "else" | "return" | "true" | "false" | "null" | "undefined" => true,
        _ => false,
    }
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
            TokenSymbol::Ampersand => "&",
            TokenSymbol::Pipe => "|",
            TokenSymbol::Exclamation => "!",
            TokenSymbol::Underscore => "_",
        }
    }
}

impl TokenSymbol {
    pub fn from_char(value: char) -> Option<Self> {
        match value {
            '=' => Some(Self::Equal),
            '>' => Some(Self::GreaterThan),
            '<' => Some(Self::LessThan),
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Asterisk),
            '/' => Some(Self::Slash),
            '(' => Some(Self::ParenOpen),
            ')' => Some(Self::ParenClose),
            '{' => Some(Self::BraceOpen),
            '}' => Some(Self::BraceClose),
            '[' => Some(Self::BracketOpen),
            ']' => Some(Self::BracketClose),
            ',' => Some(Self::Comma),
            '.' => Some(Self::Dot),
            '@' => Some(Self::At),
            '&' => Some(Self::Ampersand),
            '|' => Some(Self::Pipe),
            '!' => Some(Self::Exclamation),
            '_' => Some(Self::Underscore),
            _ => None,
        }
    }
}
