pub mod variable_declaration;

use crate::vm::{scope::Scope, VM};

use super::{Node, NodeFactory};

pub trait StatementNode: Node + NodeFactory {
    fn run(&self, vm: &mut VM, scope: &mut Scope);
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(variable_declaration::VariableDeclarationNode)
}