use std::{cell::Ref, collections::HashMap, fmt};

use crate::data::{types::DataType, DataValue};

pub type Variable = (DataValue, DataType);

/// A context for running code, contains all variables.
/// Extends all the data from its parent
#[derive(Debug)]
pub struct Scope<'a> {
    /// All the variables stored in the current scope
    pub variables: HashMap<String, Variable>,

    /// The parent scope of the actual one, this will be used to look for variables that are not in the current scope
    pub parent: Option<Ref<'a, Self>>
}

impl<'a> Scope<'a> {
    /// Create a new empty context
    pub fn new(variables: HashMap<String, Variable>) -> Self {
        Scope { variables, parent: None }
    }

    /// Provide a new scope with a parent
    pub fn with_parent(variables: HashMap<String, Variable>, parent: Ref<'a, Scope<'a>>) -> Self {
        let mut scope = Scope::new(variables);
        scope.parent = Some(parent);
        scope
    }

    /// Add a new variable to the current scope (or overwrite it if it already exists).
    /// If there is one with the same name in the parent scope, it won't be overwritten,
    /// but inaccessable from the current scope, as it will shadow the parent's variable.
    pub fn create(&mut self, name: String, value: Variable) {
        if let Some(_variable) = self.get(&name) {
            self.variables.remove(&name);
            self.variables.insert(name, value);
        } else {
            self.variables.insert(name, value);
        }
    }

    /// Retrieve a recefrence to a variable from the scope,
    /// if there is no variable with the given name, it will look in the parent scope.
    /// If the variable is not found neither, it will return None.
    pub fn get(&self, name: &String) -> Option<&Variable> {
        // Buscamos en el scope actual
        if let Some(value) = self.variables.get(name) {
            return Some(value);
        }

        // Try to search at the parent scope, otherwise return None
        self.parent.as_ref().map(|parent| parent.get(name)).flatten()
    }

    // Retrieve a mutable reference to a variable from the scope (or parent)
    #[allow(dead_code)]
    fn get_mut(&mut self, name: &String) -> Option<&mut Variable> {
        //Buscamos en el scope actual
        if let Some(value) = self.variables.get_mut(name) {
            return Some(value);
        }

        todo!("get_mut() can't be called for the parent yet");
        // TODO: idk what to do here
        // self.parent.as_mut().map(|parent| parent.get_mut(name)).flatten()
    }
}

impl Default for Scope<'_> {
    /// Geta an empty context
    fn default() -> Self {
        Scope::new(HashMap::new())
    }
}

impl fmt::Display for Scope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Context ================================\n")?;

        for (name, value) in &self.variables {
            write!(f, "\t{name:16}: {} = {}\n", value.1, value.0)?;
        }

        if let Some(parent) = &self.parent {
            write!(f, "Parent:\n{}", parent)
        } else {
            write!(f, "")
        }
    }
}