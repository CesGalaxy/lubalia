pub mod error;
pub mod cursor;

use error::ParserError;
use lubalia_utils::transcriber::{cursor::TranscriberCursor, result::TranscriptionResult, transcriber, TranscriberTickResult};

use crate::{
    node::{ASTNode, Node},
    root::ASTRootItem,
};

use super::token::{symbol::TokenSymbol, Token};

/// Transcribe a list of tokens into an AST (Abstract Syntax Tree).
pub fn parser(tokens: Vec<Token>) -> TranscriptionResult<Token, ASTRootItem, ParserError> {
    let transcription = transcriber(tokens, parser_tick)?;

    Ok(transcription)
}

/// Each tick of the parser transcriber
fn parser_tick(cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> TranscriberTickResult<ASTRootItem, ParserError> {
    // TODO: This task should be for ASTRootItem
    match initial_token {
        Token::Symbol(TokenSymbol::EOF) => Ok(None),
        _ => ASTNode::transcribe(cursor).map(|astn| astn.map(ASTRootItem::Node))
    }
}