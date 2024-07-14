pub mod node;
mod machine;
mod exception;
pub mod data;
mod arithmetic;

use exception::ParserError;
use machine::ParsingMachine;
use node::{scope::ScopeNode, AbstractSyntaxTree, NodeFactory};

use super::token::Token;

/// Generates an abstract syntax tree (AST) from a vector of tokens.
pub fn parse_tree(tokens: Vec<Token>) -> Result<AbstractSyntaxTree, ParserError> {
    let mut machine = ParsingMachine::new(tokens);

    Ok(ScopeNode::from_tokens(&mut machine)?.into())
}