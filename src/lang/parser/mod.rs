pub mod error;
pub mod root;
pub mod node;
pub mod data;

use error::ParserError;
use node::{ASTNode, Node};
use root::ASTRootItem;

use crate::utils::transcriber::{cursor::TranscriberCursor, result::TranscriptionResult, transcriber};

use super::token::Token;

/// Transcribe a list of tokens into an AST (Abstract Syntax Tree).
pub fn parser(tokens: Vec<Token>) -> TranscriptionResult<Token, ASTRootItem, ParserError> {
    let transcription = transcriber(tokens, parser_tick)?;

    Ok(transcription)
}

/// Each tick of the parser transcriber
fn parser_tick(cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> Result<Option<ASTRootItem>, ParserError> {
    // TODO: This task should be for ASTRootItem
    match initial_token {
        Token::EOF => Ok(None),
        _ => ASTNode::transcribe(cursor).map(|astn| astn.map(ASTRootItem::Node))
    }
}