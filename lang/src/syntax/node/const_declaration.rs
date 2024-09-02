use crate::parser::ParserCursor;

use super::{NodeFactory, NodeParsingResult};

#[derive(Debug, Clone)]
pub struct ConstDeclaration {
    pub name: String,
    pub value: i32,
}

impl NodeFactory for ConstDeclaration {
    fn parse(_cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        Ok(Self {
            name: "a".to_string(),
            value: 10,
        })
    }
}