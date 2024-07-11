pub mod variable_declaration;

use crate::{
    lang::{
        lexer::token::Token,
        parser::exception::{ExpectedToken, ParserError, ParserException}
    },
    vm::context::Context
};

use super::{Node, NodeFactory};

pub trait StatementNode: Node + NodeFactory {
    fn run(&self, scope: &mut Context);
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(variable_declaration::VariableDeclarationNode)
}

impl Node for Statement {}

impl NodeFactory for Statement {
    /// Try to get a statement given a vec of tokens
    fn from_tokens(m: &mut crate::lang::parser::machine::ParsingMachine) -> Result<Self, ParserError> where Self: Sized {
        match m.peek() {
            Some(Token::Keyword(keyword)) => match keyword.as_str() {
                "let" => Ok(Statement::VariableDeclaration(variable_declaration::VariableDeclarationNode::from_tokens(m)?)),
                _ => panic!("Invalid keyword"),
            },
            _ => Err(m.except(ParserException::TokenExpected(ExpectedToken::Keyword("<var name>"))))
        }
    }
}

impl StatementNode for Statement {
    fn run(&self, scope: &mut Context) {
        match self {
            Statement::VariableDeclaration(node) => node.run(scope)
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VariableDeclaration(node) => write!(f, "{}", node)
        }
    }
}