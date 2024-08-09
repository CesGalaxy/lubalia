pub mod scope;

use std::{cell::RefCell, fmt::Debug};

use scope::Scope;

use crate::{data::DataValue, lang::syntax::{node::{statement::StatementNode, ASTNode}, root::ASTRootItem}};

/// A virtual machine that executes a program in bytecode
#[derive(Debug)]
pub struct VM {
    /// The global scope (context) of the VM
    pub global: Scope<'static>,

    /// The last value returned by an expression (_)
    pub last_value: DataValue,
}

impl VM {
    /// Create a new VM with a program
    pub fn new() -> Self {
        VM {
            global: Scope::default(),
            last_value: DataValue::default()
        }
    }

    /// Start running the emulation
    pub fn evaluate(&mut self, program: Vec<ASTRootItem>) -> Option<DataValue> {
        let scope = Scope::default();
        let scope = RefCell::new(scope);

        // Loop through all the nodes in the program
        for astri in program {
            // Execute all the nodes until a value is returned
            let ASTRootItem::Node(node) = astri;

            // If the node returned a value, return it
            if let Some(value) = self.tick(node, &scope) {
                return Some(value);
            }
        }

        None
    }

    /// Each tick corresponds to the execution of a single instruction/node.
    pub fn tick(&mut self, node: ASTNode, scope: &RefCell<Scope>) -> Option<DataValue> {
        node.execute(self, scope).map(|result| result.returned()).flatten()
    }
}
