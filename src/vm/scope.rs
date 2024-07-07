use crate::lang::parser::data::DataValue;

pub struct Scope<'a> {
    pub variables: Vec<(String, DataValue)>,
    pub parent: Option<&'a Scope<'a>>
}

impl Scope<'static> {
    pub fn new() -> Self {
        Scope {
            variables: Vec::new(),
            parent: None
        }
    }
}

impl<'a> Scope<'a> {
    pub fn push(&mut self, name: String, value: DataValue) {
        self.variables.push((name, value));
    }

    pub fn get(&self, name: String) -> Option<&DataValue> {
        if let Some(local) = self.variables.iter().find(|v| v.0 == name).map(|v| &v.1) {
            Some(local)
        } else {
            if let Some(parent) = self.parent.map(|scope| scope.get(name)) {
                parent
            } else {
                None
            }
        }
    }
}