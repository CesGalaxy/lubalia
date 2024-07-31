use super::DataValue;

impl PartialEq for DataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Same type
            (DataValue::Number(a), DataValue::Number(b)) => a == b,
            (DataValue::String(a), DataValue::String(b)) => a == b,
            (DataValue::Char(a), DataValue::Char(b)) => a == b,
            (DataValue::Boolean(a), DataValue::Boolean(b)) => a == b,
            (DataValue::Null, DataValue::Null) => true,

            // String - Number
            (DataValue::Number(a), DataValue::String(b)) => &a.to_string() == b,
            (DataValue::String(a), DataValue::Number(b)) => a == &b.to_string(),

            // String - Boolean
            (DataValue::Boolean(a), DataValue::String(b)) => &a.to_string() == b,
            (DataValue::String(a), DataValue::Boolean(b)) => a == &b.to_string(),

            // Number - Boolean
            (DataValue::Number(a), DataValue::Boolean(b)) => a == &f64::from(*b),
            (DataValue::Boolean(a), DataValue::Number(b)) => &f64::from(*a) == b,

            // List - List
            (DataValue::List(a), DataValue::List(b)) => a == b,

            // Char - String
            (DataValue::Char(a), DataValue::String(b)) => a.to_string() == *b,
            (DataValue::String(a), DataValue::Char(b)) => *a == b.to_string(),

            // Char - Number
            (DataValue::Char(a), DataValue::Number(b)) => a.to_string() == b.to_string(),
            (DataValue::Number(a), DataValue::Char(b)) => a.to_string() == b.to_string(),

            // Char - Boolean
            (DataValue::Char(a), DataValue::Boolean(b)) => a == if *b { &'1' } else { &'0' },
            (DataValue::Boolean(a), DataValue::Char(b)) => (if *a { &'1' } else { &'0' }) == b,

            // Null and List always return false
            (DataValue::List(_), _) | (_, DataValue::List(_)) => false,
            (DataValue::Null, _) | (_, DataValue::Null) => false,
        }
    }
}

impl PartialOrd for DataValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // Same type
            (DataValue::Number(a), DataValue::Number(b)) => a.partial_cmp(b),
            (DataValue::String(a), DataValue::String(b)) => Some(a.cmp(b)),
            (DataValue::Char(a), DataValue::Char(b)) => Some(a.cmp(b)),
            (DataValue::Boolean(a), DataValue::Boolean(b)) => Some(a.cmp(b)),
            (DataValue::Null, DataValue::Null) => Some(std::cmp::Ordering::Equal),

            // String - Number
            (DataValue::Number(a), DataValue::String(b)) => Some(a.to_string().cmp(b)),
            (DataValue::String(a), DataValue::Number(b)) => Some(a.cmp(&b.to_string())),

            // String - Boolean
            (DataValue::Boolean(a), DataValue::String(b)) => Some(a.to_string().cmp(b)),
            (DataValue::String(a), DataValue::Boolean(b)) => Some(a.cmp(&b.to_string())),

            // Number - Boolean
            (DataValue::Number(a), DataValue::Boolean(b)) => Some(a.partial_cmp(&f64::from(*b)).unwrap_or(std::cmp::Ordering::Equal)),
            (DataValue::Boolean(a), DataValue::Number(b)) => Some(f64::from(*a).partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),

            // List - Number (use list length)
            (DataValue::List(a), DataValue::Number(b)) => Some(b.total_cmp(&(a.len() as f64))),
            (DataValue::Number(a), DataValue::List(b)) => Some(a.total_cmp(&(b.len() as f64))),

            // List - Boolean (use if list is empty)
            (DataValue::List(_), _) => None,
            (_, DataValue::List(_)) => None,

            // Null is always less than any other type
            (DataValue::Null, _) => Some(std::cmp::Ordering::Less),
            (_, DataValue::Null) => Some(std::cmp::Ordering::Greater),

            // TODO: WTF is this
            _ => None,
        }
    }
}

// EMPTY + TRUE = false
// llena + TRUE = TRUE
// EMPTY + false = TRUE
// llena + false = false