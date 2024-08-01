use colored::Colorize;

/// A keyword that is reserved for special uses and is part from the language grammar.
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
    Repeat,
    Switch,
    Case
}

impl TokenLangKeyword {
    /// Tells if a given string (should be a keyword) is representing a lang keyword.
    /// It will return None is there's no keyword that matches the provided value.
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
            "repeat" => Some(Self::Repeat),
            "switch" => Some(Self::Switch),
            "case" => Some(Self::Case),
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
            TokenLangKeyword::Repeat => "repeat",
            TokenLangKeyword::Switch => "switch",
            TokenLangKeyword::Case => "case",
        }
    }
}

impl std::fmt::Display for TokenLangKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self).green().bold())
    }
}