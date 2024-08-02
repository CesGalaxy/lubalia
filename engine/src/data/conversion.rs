use super::DataValue;

impl From<DataValue> for f64 {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => number,
            DataValue::String(string) => string.parse().unwrap_or(0.0),
            DataValue::Char(char) => char as usize as f64,
            DataValue::Boolean(boolean) => if boolean { 1.0 } else { 0.0 },
            DataValue::List(list) => list.len() as f64,
            DataValue::Null => 0.0,
            DataValue::Callable(_, _, _) => 0.0
        }
    }
}

impl From<DataValue> for usize {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => number as usize,
            DataValue::String(string) => string.len(),
            // TODO: Please no
            // TODO: Error when stupid operations?
            // TODO: Organize this
            DataValue::Char(char) => char as usize,
            DataValue::List(list) => list.len(),
            DataValue::Boolean(boolean) => if boolean { 1 } else { 0 },
            DataValue::Null => 0,
            DataValue::Callable(_, _, _) => 0
        }
    }
}
