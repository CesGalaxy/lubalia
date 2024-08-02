use lubalia_utils::{loop_through::LoopThrough, transcriber::cursor::TranscriberCursor};

use crate::lang::token::{symbol::TokenSymbol, Token};

pub fn ignore_eols(cursor: &mut TranscriberCursor<Token>) -> usize {
    cursor.ignore_all(&Token::Symbol(TokenSymbol::EOL))
}

pub fn ignore_eols_but_last(cursor: &mut TranscriberCursor<Token>) -> usize {
    cursor.ignore_loop(LoopThrough::Before(1, Box::new(LoopThrough::WhileEq(&Token::Symbol(TokenSymbol::EOL)))))
}