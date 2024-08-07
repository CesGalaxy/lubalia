pub mod context;
pub mod tick;

use context::Context;
use tick::VMTick;

use crate::{data::DataValue, lang::syntax::{node::ASTNode, root::ASTRootItem}};

/// A virtual machine that executes a program in bytecode
#[derive(Debug)]
pub struct VM {
    /// The global scope (context) of the VM
    pub global: Context,

    /// The last value returned by an expression (_)
    pub last_value: DataValue,
}

impl VM {
    /// Create a new VM with a program
    pub fn new() -> Self {
        VM {
            global: Context::default(),
            last_value: DataValue::default()
        }
    }

    /// Start running the emulation
    pub fn evaluate(&mut self, program: Vec<ASTRootItem>) -> Option<DataValue> {
        // Loop through all the nodes in the program
        for astri in program {
            // Execute all the nodes until a value is returned
            let ASTRootItem::Node(node) = astri;

            if let Some(value) = self.tick(node) {
                return Some(value);
            }
        }

        None
    }

    /// Each tick corresponds to the execution of a single instruction/node.
    pub fn tick(&mut self, node: ASTNode) -> Option<DataValue> {
        let mut tick = VMTick {
            vm: self,
            context: None
        };

        node.execute(&mut tick)
    }
}
