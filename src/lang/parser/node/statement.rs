pub mod variable_declaration;

use crate::{
    lang::{parser::error::ParserError, token::Token},
    utils::transcriber::cursor::TranscriberCursor, vm::{context::Context, VM}
};

use super::Node;

#[derive(Debug, Clone)]
pub enum ASTStatement {
    VariableDeclaration(variable_declaration::VariableDeclaration),
}

pub trait StatementNode: Node {
    fn execute(&self, context: &mut Context, vm: &mut VM) -> Result<(), &'static str>;
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
    fn execute(&self, context: &mut Context, vm: &mut VM) -> Result<(), &'static str> {
        match self {
            ASTStatement::VariableDeclaration(vd) => vd.execute(context, vm),
        }
    }
}