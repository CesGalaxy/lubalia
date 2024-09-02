use const_declaration::ConstDeclarationNode;

use crate::parser::{ParserCursor, error::ParserError};

pub mod const_declaration;

#[derive(Debug)]
pub enum Node {
    ConstDeclaration(ConstDeclarationNode),
}

impl NodeFactory for Node {
    fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        ConstDeclarationNode::parse(cursor).map(|node| node.map(Node::ConstDeclaration))
    }
}

pub type NodeParsingResult<T> = Result<Option<T>, ParserError>;

pub trait NodeFactory {
    fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self> where Self: Sized;
}