pub mod error;
pub mod cursor;

use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::syntax::{node::NodeParsingResult, statement::list::StatementList};

use super::token::Token;

pub type ParserCursor<'a> = TranscriberCursor<'a, Token>;

/// Transcribe a list of tokens into an AST (Abstract Syntax Tree).
pub fn parser(tokens: Vec<Token>) -> NodeParsingResult<StatementList> {
    //let mut transcription = transcriber(tokens, parser_tick);
    let mut cursor = TranscriberCursor::new(&tokens);

    let nodes = StatementList::parse(&mut cursor);

    nodes
}