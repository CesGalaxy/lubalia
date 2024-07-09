use colored::Colorize;

#[derive(Debug, Clone)]
pub enum DataValue {
    String(String),
    Number(f64),
    Boolean(bool)
}

impl From<DataValue> for String {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::String(s) => s,
            DataValue::Number(n) => n.to_string(),
            DataValue::Boolean(b) => b.to_string()
        }
    }
}

impl std::fmt::Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValue::Number(n) => write!(f, "{}", n.to_string().cyan()),
            DataValue::String(s) => write!(f, "{}{}{}", "'".black(), s.yellow(), "'".black()),
            DataValue::Boolean(b) => write!(f, "{}", b.to_string().cyan())
        }
    }
}
