use lubalia_utils::transcriber::TranscriberTickResult;

use crate::parser::{error::ParserError, ParserCursor};

pub enum ASTRootItem {
    Node,
}

impl ASTRootItem {
    pub fn parse(cursor: &mut ParserCursor) -> TranscriberTickResult<ASTRootItem, ParserError> {
        unimplemented!()
    }
}