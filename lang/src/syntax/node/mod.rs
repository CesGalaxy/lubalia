use lubalia_utils::transcriber::TranscriberTickResult;

use crate::parser::{ParserCursor, error::ParserError};

pub mod const_declaration;

pub enum Node {

}

pub type NodeParsingResult<T> = TranscriberTickResult<T, ParserError>;

pub trait NodeFactory {
    fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self> where Self: Sized;
}