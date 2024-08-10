use std::fmt;

use super::DataValue;

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Number,
    String,
    Char,
    Boolean,
    True,
    False,
    Truly,
    Falsely,
    List(Box<ListType>),
    Null,
    Callable,
    Optional(Box<DataType>),
    Mixed(Vec<DataType>),
    Any,
    Never
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListType {
    /// A list with whatever type of data.
    Any(Option<usize>),

    /// A list with a type for each item
    Fixed(Vec<DataType>),

    /// A list full of the same type of data.
    Unique(DataType, Option<usize>)
}

impl Default for DataType {
    fn default() -> Self {
        DataType::Any
    }
}

impl Default for ListType {
    fn default() -> Self {
        ListType::Any(None)
    }
}

impl DataType {
    pub fn matched(&self, value: &DataValue) -> bool {
        match self {
            DataType::Number => matches!(value, DataValue::Number(_)),
            DataType::String => matches!(value, DataValue::String(_)),
            DataType::Char => matches!(value, DataValue::Char(_)),
            DataType::Boolean => matches!(value, DataValue::Boolean(_)),
            DataType::True => matches!(value, DataValue::Boolean(true)),
            DataType::False => matches!(value, DataValue::Boolean(false)),
            DataType::Truly => bool::from(value.clone()),
            DataType::Falsely => !bool::from(value.clone()),
            DataType::List(list_type) => {
                match list_type.as_ref() {
                    ListType::Any(list_len) => {
                        if let DataValue::List(list) = value {
                            if let Some(len) = list_len {
                                list.len() == *len
                            } else {
                                true
                            }
                        } else {
                            false
                        }
                    },
                    ListType::Fixed(types) => {
                        if let DataValue::List(list) = value {
                            for (item, data_type) in list.iter().zip(types.iter()) {
                                if !data_type.matched(item) {
                                    return false;
                                }
                            }

                            true
                        } else {
                            false
                        }
                    },
                    ListType::Unique(data_type, list_len) => {
                        if let DataValue::List(list) = value {
                            if let Some(len) = list_len {
                                if list.len() != *len {
                                    return false;
                                }
                            }

                            for item in list.iter() {
                                if !data_type.matched(item) {
                                    return false;
                                }
                            }

                            true
                        } else {
                            false
                        }
                    }
                }
            },
            DataType::Null => matches!(value, DataValue::Null),
            DataType::Callable => matches!(value, DataValue::Callable(_, _, _)),
            DataType::Optional(data_type) => if let DataValue::Null = value { true } else { data_type.matched(value) },
            DataType::Mixed(types) => {
                for data_type in types.iter() {
                    if data_type.matched(value) {
                        return true;
                    }
                }

                false
            },
            DataType::Any => true,
            DataType::Never => false
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Number => write!(f, "Number"),
            DataType::String => write!(f, "String"),
            DataType::Char => write!(f, "Char"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::True => write!(f, "True"),
            DataType::False => write!(f, "False"),
            DataType::Truly => write!(f, "Truly"),
            DataType::Falsely => write!(f, "Falsely"),
            DataType::List(list_type) => write!(f, "List<{}>", list_type),
            DataType::Null => write!(f, "Null"),
            DataType::Callable => write!(f, "Callable"),
            DataType::Optional(data_type) => write!(f, "Optional<{}>", data_type),
            DataType::Mixed(types) => {
                let mut types_str = String::new();

                for data_type in types {
                    types_str.push_str(&format!("{}, ", data_type));
                }

                // Remove the last comma and space
                types_str.pop();
                types_str.pop();

                write!(f, "Mixed<{}>", types_str)
            },
            DataType::Any => write!(f, "Any"),
            DataType::Never => write!(f, "Never")
        }
    }
}

impl fmt::Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListType::Any(list_len) => write!(f, "Any[{}]", list_len.map(|len| len.to_string()).unwrap_or("".to_string())),
            ListType::Fixed(types) => {
                let mut types_str = String::new();

                for data_type in types {
                    types_str.push_str(&format!("{}, ", data_type));
                }

                if types_str.len() > 1 {
                    // Remove the last comma and space
                    types_str.pop();
                    types_str.pop();
                }

                write!(f, "[{}]", types_str)
            },
            ListType::Unique(data_type, list_len) => write!(f, "{}[{}]", data_type, list_len.map(|len| len.to_string()).unwrap_or("".to_string()))
        }
    }
}
