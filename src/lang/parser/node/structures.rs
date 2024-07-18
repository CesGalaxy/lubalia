pub mod scope;

use crate::{lang::{parser::error::ParserError, token::Token}, utils::transcriber::cursor::TranscriberCursor};

use super::Node;

#[derive(Debug, Clone)]
pub enum Structure {
    Scope(scope::ScopeStruct),
}

impl Node for Structure {
    fn transcribe(_cursor: &mut TranscriberCursor<Token>) -> Result<Option<Structure>, ParserError> {
        Ok(None)
    }
}