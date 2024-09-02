use const_declaration::ConstDeclaration;

use crate::parser::{ParserCursor, error::ParserError};

pub mod const_declaration;
pub mod return_value;
pub mod var_declaration;

#[derive(Debug, Clone)]
pub enum Node {
    ConstDeclaration(ConstDeclaration),
}

impl NodeFactory for Node {
    fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        ConstDeclaration::parse(cursor).map(|node| node.map(Node::ConstDeclaration))
    }
}

pub type NodeParsingResult<T> = Result<Option<T>, ParserError>;

pub trait NodeFactory where Self: Sized {
    fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self>;
}
