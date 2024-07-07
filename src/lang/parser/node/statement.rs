pub mod variable_declaration;

use super::{Node, NodeFactory};

pub trait StatementNode: Node + NodeFactory {
    
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(variable_declaration::VariableDeclarationNode)
}