pub mod expression;
pub mod statement;

use statement::Statement;

use super::{exception::ParsingMachineError, machine::ParsingMachine};

pub trait Node: std::fmt::Debug + std::fmt::Display {}

pub trait NodeFactory: Node {
    /// Parses a token stream into a node or an error.
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParsingMachineError> where Self: Sized;
}

#[derive(Debug)]
pub enum TreeNode {
    Expression(expression::Expression),
    Statement(statement::Statement)
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeNode::Expression(expression) => write!(f, "[ {} ]", expression),
            TreeNode::Statement(statement) => {
                write!(f, "{{ ")?;

                match statement {
                    Statement::VariableDeclaration(node) => node.fmt(f)
                }?;

                write!(f, " }}")
            }
        }
    }
}

pub type AbstractSyntaxTree = Vec<TreeNode>;