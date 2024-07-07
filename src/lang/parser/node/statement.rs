pub mod variable_declaration;

use crate::{lang::lexer::token::Token, vm::scope::Scope};

use super::{Node, NodeFactory};

pub trait StatementNode: Node + NodeFactory {
    fn run(&self, scope: &mut Scope);
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(variable_declaration::VariableDeclarationNode)
}

impl Node for Statement {}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VariableDeclaration(node) => write!(f, "{}", node)
        }
    }
}

impl NodeFactory for Statement {
    fn from_tokens(m: &mut crate::lang::parser::machine::ParsingMachine) -> Result<Self, crate::lang::parser::exception::ParsingMachineError> where Self: Sized {
        match m.peek() {
            Some(Token::Keyword(keyword)) => match keyword.as_str() {
                "let" => Ok(
                    Statement::VariableDeclaration(
                        variable_declaration::VariableDeclarationNode::from_tokens(m)?
                    )
                ),
                _ => panic!("Invalid keyword"),
            },
            _ => Err(m.except(crate::lang::parser::exception::ParsingMachineException::TokenExpected(crate::lang::parser::exception::ExcpectedToken::Keyword("<var name>"))))
        }
    }
}

impl StatementNode for Statement {
    fn run(&self, scope: &mut Scope) {
        match self {
            Statement::VariableDeclaration(node) => node.run(scope)
        }
    }
}