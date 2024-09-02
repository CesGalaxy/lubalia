use crate::parser::ParserCursor;

use super::NodeParsingResult;

#[derive(Debug, Clone)]
pub struct VarDeclaration {
    pub name: String,
    pub value: i32,
}

impl VarDeclaration {
    pub fn parse(_cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        todo!()
    }
}