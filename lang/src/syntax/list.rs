use lubalia_utils::cursor::CursorNavigation;

use crate::{parser::ParserCursor, token::{symbol::TokenSymbol, Token}};

use super::node::{NodeFactory, NodeParsingResult};

pub struct NodeList<T: NodeFactory> {
    pub items: Vec<T>,
}

impl<T: NodeFactory> NodeFactory for NodeList<T> {
    fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self> where Self: Sized {
        let mut items = Vec::new();

        if let Ok(Some(node)) = T::parse(cursor) {
            items.push(node);
        }

        while cursor.peek().is_some_and(|token| token == &Token::Symbol(TokenSymbol::EOL) || token == &Token::Symbol(TokenSymbol::Semicolon)) {
            cursor.next();

            if let Ok(Some(node)) = T::parse(cursor) {
                items.push(node);
            }
        }

        Ok(Some(NodeList { items }))
    }
}

impl <T: NodeFactory> Into<Vec<T>> for NodeList<T> {
    fn into(self) -> Vec<T> {
        self.items
    }
}
