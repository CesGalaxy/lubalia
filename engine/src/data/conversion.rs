use super::DataValue;

impl From<DataValue> for f64 {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => number,
            _ => 0.0
        }
    }
}

impl From<DataValue> for usize {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => number as usize,
            DataValue::String(string) => string.len(),
            DataValue::List(list) => list.len(),
            DataValue::Boolean(boolean) => if boolean { 1 } else { 0 },
            // TODO: Please no
            // TODO: Error when stupid operations?
            // TODO: Organize this
            DataValue::Char(char) => char as usize,
            DataValue::Null => 0
        }
    }
}
