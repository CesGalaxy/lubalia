use super::DataValue;

impl From<DataValue> for bool {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Boolean(boolean) => boolean,
            DataValue::Number(number) => number != 0.0,
            DataValue::String(string) => !string.is_empty(),
            _ => false
        }
    }
}

impl From<bool> for DataValue {
    fn from(value: bool) -> Self {
        DataValue::Boolean(value)
    }
}