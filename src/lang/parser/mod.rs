pub mod error;
pub mod root;
pub mod node;
pub mod data;

use error::ParserError;
use node::{ASTNode, Node};
use root::ASTRootItem;

use crate::utils::transcriber::{cursor::TranscriberCursor, result::TranscriptionResult, transcriber};

use super::token::Token;

pub fn parser(tokens: Vec<Token>) -> TranscriptionResult<Token, ASTRootItem, ParserError> {
    // Source --> Token
    // Result --> Node (Expr/Stmnt)

    let transcription = transcriber(tokens, parser_tick)?;

    Ok(transcription)
}

pub fn parser_tick(cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> Result<Option<ASTRootItem>, ParserError> {
    // TODO: This task should be for ASTRootItem
    match initial_token {
        Token::EOF => Ok(None),
        _ => ASTNode::transcribe(cursor).map(|astn| astn.map(ASTRootItem::Node))
    }
}