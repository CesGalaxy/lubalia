use lubalia_utils::cursor::CursorNavigation;

use crate::{parser::{cursor::{ignore_eols, ignore_eols_but_last}, ParserCursor}, token::{symbol::TokenSymbol, Token}};

use super::StatementNode;

#[derive(Debug)]
pub struct StatementList(pub Vec<StatementNode>);

impl StatementList {
    pub fn parse(cursor: &mut ParserCursor) -> Self {
        let mut items = Vec::new();

        if cursor.peek().is_some() {
            if let Ok(Some(statement)) = StatementNode::parse(cursor) {
                items.push(statement);
            }

            ignore_eols_but_last(cursor);

            while cursor.peek().is_some_and(|token| token == &Token::Symbol(TokenSymbol::EOL) || token == &Token::Symbol(TokenSymbol::EOL)) {
                ignore_eols(cursor);

                if let Ok(Some(statement)) = StatementNode::parse(cursor) {
                    items.push(statement);
                }
            }
        }

        Self(items)
    }
}