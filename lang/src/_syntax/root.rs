use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, TranscriberTickResult}};

use crate::{parser::{context::ParsingContext, error::ParserError}, token::{symbol::TokenSymbol, Token}};

use super::node::{ASTNode, Node};

/// A script can contain multiple items,
/// in the case of nodes, they will be executed.
#[derive(Debug, Clone)]
pub enum ASTRootItem {
    /// A node that will be executed by the VM
    Node(ASTNode)
}

impl ASTRootItem {
    /// Create a new root item from a node
    pub fn parse(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> TranscriberTickResult<Self, ParserError> {
        match cursor.peek() {
            Some(Token::Symbol(TokenSymbol::EOF)) => Ok(None),
            _ => ASTNode::transcribe(cursor, ctx).map(|astn| astn.map(ASTRootItem::Node))
        }
    }
}

impl fmt::Display for ASTRootItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTRootItem::Node(node) => write!(f, "> {}", node)
        }
    }
}