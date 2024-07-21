pub mod variable_declaration;

use crate::{
    lang::{parser::error::ParserError, token::Token},
    utils::transcriber::cursor::TranscriberCursor, vm::VMTick
};

use super::Node;

/// An instruction the VM executes without returning a value
#[derive(Debug, Clone)]
pub enum ASTStatement {
    VariableDeclaration(variable_declaration::VariableDeclaration),
}

pub trait StatementNode: Node {
    fn execute(&self, tick: &mut VMTick) -> Result<(), &'static str>;
}

impl Node for ASTStatement {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTStatement>, ParserError> {
        match cursor.peek() {
            Some(Token::Keyword(keyword)) => {
                match keyword.as_str() {
                    "let" => variable_declaration::VariableDeclaration::transcribe(cursor).map(|vd| vd.map(ASTStatement::VariableDeclaration)),
                    _ => Ok(None)
                }
            },
            _ => Ok(None)
        }
    }
}

impl StatementNode for ASTStatement {
    fn execute(&self, tick: &mut VMTick) -> Result<(), &'static str> {
        match self {
            ASTStatement::VariableDeclaration(vd) => vd.execute(tick),
        }
    }
}