use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenLiteral {
    /// A string containing any text
    String(String),

    /// A number (with decimals)
    Number(f64),
}

impl std::fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "[str:{}]", value.yellow().bold()),
            Self::Number(value) => write!(f, "[num:{}]", value.to_string().cyan().bold()),
        }
    }
}