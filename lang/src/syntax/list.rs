use lubalia_utils::cursor::CursorNavigation;

use crate::{parser::ParserCursor, token::Token};

pub fn parse_all<T>(
    cursor: &mut ParserCursor,
    parse: fn(&mut ParserCursor) -> Option<T>,
    divider: fn(&Token) -> bool,
) -> Vec<T> {
    let mut items = Vec::new();

    if let Some(node) = parse(cursor) {
        items.push(node);
    }

    while cursor.peek().is_some_and(divider) {
        if let Some(node) = parse(cursor) {
            items.push(node);
        }
    }

    items
}
