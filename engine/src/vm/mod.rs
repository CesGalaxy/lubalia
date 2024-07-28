pub mod context;

use context::Context;

use crate::{data::DataValue, node::ASTNode, root::ASTRootItem};

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
            global: Context::new(),
            last_value: DataValue::default()
        }
    }

    /// Start running the emulation
    pub fn evaluate(&mut self, program: Vec<ASTRootItem>) -> Option<DataValue> {
        // Loop through all the nodes in the program
        for astri in program {
            // Execute all the nodes until a value is returned
            if let ASTRootItem::Node(node) = astri {
                if let Some(value) = self.tick(node) {
                    return Some(value);
                }
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

pub struct VMTick<'a> {
    /// The VM running the tick
    pub vm: &'a mut VM,

    /// The smallest on which the tick is run
    pub context: Option<Box<Context>>,
}

impl VMTick<'_> {
    /// Gets the current context used ing the VM,
    /// if there's no custom context it returns the global
    pub fn get_context(&mut self) -> &mut Context {
        if let Some(context) = &mut self.context {
            context
        } else {
            &mut self.vm.global
        }
    }
}