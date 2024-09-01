pub enum DataValue {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Array(Vec<DataValue>),
    Object(HashMap<String, DataValue>),
}