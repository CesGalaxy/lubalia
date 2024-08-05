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

impl From<DataValue> for String {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => number.to_string(),
            DataValue::String(string) => string,
            DataValue::Char(character) => character.to_string(),
            DataValue::Boolean(boolean) => boolean.to_string(),
            DataValue::List(list) => {
                let mut list_str = String::new();

                for item in list {
                    list_str.push_str(&format!("{}, ", item));
                }

                list_str
            },
            DataValue::Null => String::from("NULL"),
            DataValue::Callable(_, _, _) => String::from("CALL")
        }
    }
}

impl From<DataValue> for Option<char> {
    fn from(value: DataValue) -> Self {
        match value {
            DataValue::Number(number) => Some(number as u8 as char),
            DataValue::String(string) => string.chars().next(),
            DataValue::Char(character) => Some(character),
            DataValue::Boolean(boolean) => Some(if boolean { '1' } else { '0' }),
            DataValue::List(_) => None,
            DataValue::Null => Some('\0'),
            DataValue::Callable(_, _, _) => None
        }
    }
}

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
