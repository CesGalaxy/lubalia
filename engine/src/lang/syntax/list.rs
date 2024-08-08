use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::lang::{parser::context::ParsingContext, token::{symbol::TokenSymbol, Token}};

use super::node::{ASTNode, Node, NodeParserTickResult};

pub struct NodeList<T: Node>(Vec<T>);

impl Node for NodeList<ASTNode> {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> {
        let mut nodes = Vec::new();

        while let Some(node) = ASTNode::transcribe(cursor, ctx)? {
            nodes.push(node);

            if cursor.peek() == Some(&Token::Symbol(TokenSymbol::Semicolon)) {
                cursor.next();
            } else {
                break;
            }
        }

        Ok(Some(Self(nodes)))
    }
}

impl fmt::Display for NodeList<ASTNode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.0 {
            write!(f, "\t{}", node)?;
        }

        Ok(())
    }
}
