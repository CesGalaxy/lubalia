mod runtime;
pub mod context;

use context::Context;

use crate::lang::parser::{data::DataValue, root::ASTRootItem};

/// A virtual machine that executes a program in bytecode
#[derive(Debug)]
pub struct VM {
    /// The program to be executed in AST form
    program: Vec<ASTRootItem>,
    /// The position of the cursor in the root-node vector (AST)
    ip: usize,
    /// The global scope (context) of the VM
    pub global: Context,
    /// The last value returned by an expression (_)
    pub last_value: DataValue
}

impl VM {
    pub fn new(program: Vec<ASTRootItem>) -> Self {
        VM {
            program,
            ip: 0,
            global: Context::new(),
            last_value: DataValue::default()
        }
    }
}