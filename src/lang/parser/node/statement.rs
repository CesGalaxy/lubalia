pub mod variable_declaration;

use crate::{
    lang::{parser::error::ParserError, token::Token},
    utils::transcriber::cursor::TranscriberCursor
};

use super::Node;

#[derive(Debug, Clone)]
pub enum ASTStatement {
    VariableDeclaration(variable_declaration::VariableDeclaration),
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