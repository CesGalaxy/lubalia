use lubalia_utils::{cursor::CursorNavigation, transcriber::TranscriberTickResult};

use crate::parser::{error::ParserError, ParserCursor};

use super::node::{const_declaration::ConstDeclarationNode, NodeFactory};

#[derive(Debug, Clone)]
pub enum ASTRootItem {
    Const(ConstDeclarationNode),
    Node,
}

impl ASTRootItem {
    pub fn parse(cursor: &mut ParserCursor) -> TranscriberTickResult<ASTRootItem, ParserError> {
        match cursor.peek() {
            Some(_) => ConstDeclarationNode::parse(cursor).map(|a| a.map(ASTRootItem::Const)),
            None => unreachable!(),
        }
    }
}
