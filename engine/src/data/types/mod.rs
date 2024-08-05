use super::DataValue;

#[derive(Debug, Clone)]
pub enum DataType {
    Number,
    String,
    Char,
    Boolean,
    List(Box<ListType>),
    Null,
    Callable,
    Optional(Box<DataType>),
    Mixed(Vec<DataType>),
    Any,
    Never
}

#[derive(Debug, Clone)]
pub enum ListType {
    /// A list with whatever type of data.
    Any(Option<usize>),

    /// Just allow certain types of data.
    Mixed(Vec<DataType>, Option<usize>),

    /// A list with a type for each item
    Fixed(Vec<DataType>),

    /// A list full of the same type of data.
    Unique(DataType, Option<usize>)
}

impl DataType {
    pub fn matched(&self, value: &DataValue) -> bool {
        match self {
            DataType::Number => matches!(value, DataValue::Number(_)),
            DataType::String => matches!(value, DataValue::String(_)),
            DataType::Char => matches!(value, DataValue::Char(_)),
            DataType::Boolean => matches!(value, DataValue::Boolean(_)),
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
                    ListType::Mixed(types, list_len) => {
                        if let DataValue::List(list) = value {
                            if let Some(len) = list_len {
                                if list.len() != *len {
                                    return false;
                                }
                            }

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
