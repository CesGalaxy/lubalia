use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenLangKeyword {
    Let,
    Const,
    Fn,
    If,
    Else,
    Return,
    True,
    False,
    Null,
    Undefined,
}

impl TokenLangKeyword {
    pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "let" => Some(Self::Let),
            "const" => Some(Self::Const),
            "fn" => Some(Self::Fn),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "return" => Some(Self::Return),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            "null" => Some(Self::Null),
            "undefined" => Some(Self::Undefined),
            _ => None,
        }
    }
}

impl From<&TokenLangKeyword> for &'static str {
    fn from(value: &TokenLangKeyword) -> Self {
        match value {
            TokenLangKeyword::Let => "let",
            TokenLangKeyword::Const => "const",
            TokenLangKeyword::Fn => "fn",
            TokenLangKeyword::If => "if",
            TokenLangKeyword::Else => "else",
            TokenLangKeyword::Return => "return",
            TokenLangKeyword::True => "true",
            TokenLangKeyword::False => "false",
            TokenLangKeyword::Null => "null",
            TokenLangKeyword::Undefined => "undefined",
        }
    }
}

impl std::fmt::Display for TokenLangKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self).green().bold())
    }
}