use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenSymbol {
    Equal,
    GreaterThan,
    LessThan,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Pipe,
    BraceOpen,
    BraceClose,
    Ampersand,
    Exclamation,
    Underscore,
    Semicolon,
    EOL,
    EOF,
}

impl std::fmt::Display for TokenSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "['{}']", match self {
            Self::EOL => "EOL".magenta(),
            Self::EOF => "EOF".magenta(),
            Self::Semicolon => ";".magenta(),
            _ => <&'static str>::from(self).bright_green().bold()
        })
    }
}

impl From<&TokenSymbol> for &'static str {
    fn from(value: &TokenSymbol) -> Self {
        match value {
            TokenSymbol::Equal => "=",
            TokenSymbol::GreaterThan => ">",
            TokenSymbol::LessThan => "<",
            TokenSymbol::Plus => "+",
            TokenSymbol::Minus => "-",
            TokenSymbol::Asterisk => "*",
            TokenSymbol::Slash => "/",
            TokenSymbol::Pipe => "|",
            TokenSymbol::BraceOpen => "{",
            TokenSymbol::BraceClose => "}",
            TokenSymbol::Ampersand => "&",
            TokenSymbol::Exclamation => "!",
            TokenSymbol::Underscore => "_",
            TokenSymbol::Semicolon => ";",
            TokenSymbol::EOL => "\n",
            TokenSymbol::EOF => "",
        }
    }
}

// TODO: TryForm
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
            '|' => Some(Self::Pipe),
            '{' => Some(Self::BraceOpen),
            '}' => Some(Self::BraceClose),
            '&' => Some(Self::Ampersand),
            '!' => Some(Self::Exclamation),
            '_' => Some(Self::Underscore),
            ';' => Some(Self::Semicolon),
            '\n' => Some(Self::EOL),
            _ => None,
        }
    }
}
