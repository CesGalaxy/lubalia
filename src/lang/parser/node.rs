pub mod expression;
pub mod statement;

use super::{exception::ParserError, machine::ParsingMachine};

pub trait Node: std::fmt::Debug + std::fmt::Display {}

pub trait NodeFactory: Node {
    /// Parses a token stream into a node or an error.
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> where Self: Sized;
}

#[derive(Debug, Clone)]
pub enum TreeNode {
    Expression(expression::Expression),
    Statement(statement::Statement)
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeNode::Expression(expression) => write!(f, "[ {expression} ]"),
            TreeNode::Statement(statement) => write!(f, "{{ {statement} }}")
        }
    }
}

pub type AbstractSyntaxTree = Vec<TreeNode>;