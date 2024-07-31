use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenLiteral {
    /// A string containing any text
    String(String),

    /// A number (with decimals)
    Number(f64),

    /// A single character
    Character(char),

    /// A way of identificating things
    Identifier(String),
}

impl std::fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "[str:{}]", value.yellow().bold()),
            Self::Number(value) => write!(f, "[num:{}]", value.to_string().cyan().bold()),
            Self::Character(value) => write!(f, "[char:{}]", value.to_string().green().bold()),
            Self::Identifier(value) => write!(f, "[id:{}]", value.blue().bold()),
        }
    }
}