use std::ops::Add;

#[derive(Debug, Clone)]
pub enum DataValue {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

impl PartialEq for DataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataValue::Int(a), DataValue::Int(b)) => a == b,
            (DataValue::Float(a), DataValue::Float(b)) => a == b,
            (DataValue::String(a), DataValue::String(b)) => a == b,
            (DataValue::Bool(a), DataValue::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Add for &DataValue {
    type Output = DataValue;

    fn add(self, other: Self) -> DataValue {
        match (self, other) {
            (DataValue::Int(a), DataValue::Int(b)) => DataValue::Int(a + b),
            (DataValue::Float(a), DataValue::Float(b)) => DataValue::Float(a + b),
            (DataValue::String(a), DataValue::String(b)) => DataValue::String(format!("{}{}", a, b)),
            (a, b) => panic!("Cannot add {a:?} and {b:?}"),
        }
    }
}
