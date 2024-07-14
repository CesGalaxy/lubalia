use crate::lang::old_parser::data::DataValue;

/// A context for running code, contains all variables.
/// Extends all the data from its parent
#[derive(Debug)]
pub struct Context<'a> {
    pub variables: Vec<(String, DataValue)>,
    pub parent: Option<&'a mut Context<'a>>
}

impl Context<'static> {
    pub fn new() -> Self {
        Context {
            variables: Vec::new(),
            parent: None
        }
    }
}

impl<'a> Context<'a> {
    /// Add a variable to the scope (or overwrite it if it already exists)
    pub fn set(&mut self, name: String, value: DataValue) {
        if let Some(variable) = self.get_mut(name.clone()) {
            *variable = value;
        } else {
            self.variables.push((name, value));
        }
    }

    /// Retrieve a mutable reference to a variable from the scope (or parent)
    pub fn get_mut(&mut self, name: String) -> Option<&mut DataValue> {
        if let Some(local) = self.variables.iter_mut().find(|v| v.0 == name) {
            Some(&mut local.1)
        } else {
            if let Some(parent) = self.parent.as_mut().map(|scope| scope.get_mut(name)) {
                parent
            } else {
                None
            }
        }
    }

    /// Retrieve a recefrence to a variable from the scope (or parent)
    pub fn get(&self, name: String) -> Option<&DataValue> {
        if let Some(local) = self.variables.iter().find(|v| v.0 == name).map(|v| &v.1) {
            Some(local)
        } else {
            if let Some(parent) = self.parent.as_ref().map(|scope| scope.get(name)) {
                parent
            } else {
                None
            }
        }
    }
}