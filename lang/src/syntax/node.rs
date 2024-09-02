use crate::parser::{ParserCursor, error::ParserError};

pub mod const_declaration;
pub mod return_value;
pub mod var_declaration;

pub type NodeParsingResult<T> = Result<Option<T>, ParserError>;

pub trait NodeFactory where Self: Sized {
    fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self>;
}
