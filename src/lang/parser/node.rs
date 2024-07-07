pub mod expression;
pub mod statement;

use super::{exception::ParsingMachineError, machine::ParsingMachine};

pub trait Node: std::fmt::Debug {}

pub trait NodeFactory: Node {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParsingMachineError> where Self: Sized;
}

#[derive(Debug)]
pub enum TreeNode {
    Expression(expression::Expression),
    Statement(statement::Statement)
}

pub type AbstractSyntaxTree = Vec<TreeNode>;