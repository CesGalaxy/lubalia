use lubalia_utils::{cursor::CursorNavigation, transcriber::TranscriberTickResult};

use crate::{parser::{error::ParserError, ParserCursor}, token::Token};

#[derive(Debug, Clone)]
pub enum ASTRootItem {
    Const,
    Node,
}

impl ASTRootItem {
    pub fn parse(cursor: &mut ParserCursor) -> TranscriberTickResult<ASTRootItem, ParserError> {
        match cursor.peek() {
            Some(_) => todo!(),
            None => unreachable!(),
        }
    }
}
