mod runtime;
pub mod scope;

use scope::Scope;

use crate::lang::parser::node::AbstractSyntaxTree;

pub struct VM {
    program: AbstractSyntaxTree,
    ip: usize,
    global: Scope<'static>
}

impl VM {
    pub fn new(program: AbstractSyntaxTree) -> Self {
        VM {
            program,
            ip: 0,
            global: Scope::new(),
        }
    }
}