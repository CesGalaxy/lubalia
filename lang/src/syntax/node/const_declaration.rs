use crate::parser::ParserCursor;

use super::{NodeFactory, NodeParsingResult};

#[derive(Debug, Clone)]
pub struct ConstDeclarationNode {
    pub name: String,
    pub value: i32,
}

impl NodeFactory for ConstDeclarationNode {
    fn parse(_cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        Ok(Some(Self {
            name: "a".to_string(),
            value: 10,
        }))
    }
}