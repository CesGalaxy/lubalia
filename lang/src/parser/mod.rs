pub mod error;
pub mod cursor;

use error::ParserError;
use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::syntax::statement::list::StatementList;

use super::token::Token;

pub type ParserCursor<'a> = TranscriberCursor<'a, Token>;

/// Transcribe a list of tokens into an AST (Abstract Syntax Tree).
pub fn parser(tokens: Vec<Token>) -> Result<StatementList, ParserError> {
    //let mut transcription = transcriber(tokens, parser_tick);
    let mut cursor = TranscriberCursor::new(&tokens);

    let nodes = StatementList::parse(&mut cursor);

    Ok(nodes)
}