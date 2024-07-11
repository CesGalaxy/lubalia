mod runtime;
pub mod context;

use context::Context;

use crate::lang::parser::node::AbstractSyntaxTree;

/// A virtual machine that executes a program in bytecode
pub struct VM {
    // The program to be executed in AST form
    program: AbstractSyntaxTree,
    // The position of the cursor in the root-node vector (AST)
    ip: usize,
    // The global scope (context) of the VM
    pub global: Context<'static>
}

impl VM {
    pub fn new(program: AbstractSyntaxTree) -> Self {
        VM {
            program,
            ip: 0,
            global: Context::new(),
        }
    }
}