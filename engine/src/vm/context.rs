use std::{collections::HashMap, fmt};

use crate::data::DataValue;

/// A context for running code, contains all variables.
/// Extends all the data from its parent
#[derive(Debug, Clone)]
pub struct Context{
    pub variables: HashMap<String, DataValue>,
    pub parent: Option<Box<Context>>
}

impl Context {
    /// Create a new empty context
    pub fn new(variables: HashMap<String, DataValue>) -> Self {
        Context { variables, parent: None }
    }

    /// Create a new context with a parent
    pub fn with_parent(variables: HashMap<String, DataValue>, parent: Option<Context>) -> Self {
        Context { variables, parent: parent.map(Box::new) }
    }

    /// Add a new variable to the current scope (or overwrite it if it already exists).
    /// If there is one with the same name in the parent scope, it won't be overwritten,
    /// but inaccessable from the current scope, as it will shadow the parent's variable.
    pub fn create(&mut self, name: String, value: DataValue) {
        if let Some(variable) = self.get_mut(name.clone()) {
            *variable = value;
        } else {
            self.variables.insert(name, value);
        }
    }

    /// Retrieve a mutable reference to a variable from the scope (or parent)
    pub fn get_mut(&mut self, name: String) -> Option<&mut DataValue> {
        if let Some(local) = self.variables.get_mut(&name){
            Some(local)
        } else {
            self.parent.as_mut().map(|scope| scope.get_mut(name)).flatten()
        }
    }

    /// Retrieve a recefrence to a variable from the scope,
    /// if there is no variable with the given name, it will look in the parent scope.
    /// If the variable is not found neither, it will return None.
    pub fn get(&self, name: String) -> Option<&DataValue> {
        // Check if the variable is in the local scope,
        // otherwise, check the parent scope.
        if let Some(local) = self.variables.get(&name) {
            Some(local)
        } else {
            // If the variable is not in the parent scope neither, return None (will end up with DataValue::default())
            self.parent.as_ref().map(|scope| scope.get(name)).flatten()
        }
    }
}

impl Default for Context {
    /// Geta an empty context
    fn default() -> Self {
        Context::new(HashMap::new())
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Context ================================\n")?;

        for (name, value) in &self.variables {
            write!(f, "\t{name:16} = {value}\n")?;
        }

        if let Some(parent) = &self.parent {
            write!(f, "Parent:\n{parent}")
        } else {
            write!(f, "")
        }
    }
}