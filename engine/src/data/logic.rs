use super::DataValue;

impl From<DataValue> for bool {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => number != 0.0,
            DataValue::String(string) => !string.is_empty(),
            DataValue::Char(character) => character != '\0',
            DataValue::Boolean(boolean) => boolean,
            DataValue::List(list) => !list.is_empty(),
            DataValue::Null => false,
            DataValue::Callable(_, _, _) => true,
        }
    }
}

impl From<bool> for DataValue {
    fn from(value: bool) -> Self {
        DataValue::Boolean(value)
    }
}