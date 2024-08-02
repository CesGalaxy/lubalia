use lubalia_utils::transcriber::cursor::TranscriberCursor;

use crate::lang::token::{symbol::TokenSymbol, Token};

pub fn ignore_eols(cursor: &mut TranscriberCursor<Token>) -> usize {
    cursor.ignore_all(&Token::Symbol(TokenSymbol::EOL))
}