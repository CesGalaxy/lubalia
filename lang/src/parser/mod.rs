pub mod error;
pub mod cursor;

use error::ParserError;
use lubalia_utils::transcriber::{cursor::TranscriberCursor, result::TranscriptionResult, transcriber, TranscriberTickResult};

use super::{syntax::root::ASTRootItem, token::Token};

pub type ParserCursor<'a> = TranscriberCursor<'a, Token>;

/// Transcribe a list of tokens into an AST (Abstract Syntax Tree).
pub fn parser(tokens: Vec<Token>) -> TranscriptionResult<Token, ASTRootItem, ParserError> {
    let transcription = transcriber(tokens, parser_tick);

    transcription
}

/// Each tick of the parser transcriber
fn parser_tick(cursor: &mut ParserCursor, _initial_token: &Token) -> TranscriberTickResult<ASTRootItem, ParserError> {
    ASTRootItem::parse(cursor)
}