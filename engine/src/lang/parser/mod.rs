pub mod error;
pub mod cursor;
pub mod manifest;
pub mod context;

use context::ParsingContext;
use error::ParserError;
use lubalia_utils::transcriber::{cursor::TranscriberCursor, result::TranscriptionResult, transcriber, TranscriberTickResult};
use manifest::ProgramManifest;

use crate::{
    node::{ASTNode, Node},
    root::ASTRootItem,
};

use super::token::{symbol::TokenSymbol, Token};

/// Transcribe a list of tokens into an AST (Abstract Syntax Tree).
pub fn parser(tokens: Vec<Token>) -> TranscriptionResult<Token, ASTRootItem, ParserError> {
    let mut manifest = ProgramManifest::default();
    let mut ctx = ParsingContext::new(&mut manifest);

    let transcription = transcriber(
        tokens,
        parser_tick(&mut ctx)
    );

    transcription
}

/// Each tick of the parser transcriber
fn parser_tick<'a>(ctx: &'a mut ParsingContext<'a>) -> impl FnMut(&mut TranscriberCursor<Token>, &Token) -> TranscriberTickResult<ASTRootItem, ParserError> + 'a {
    move |cursor: &mut TranscriberCursor<Token>, initial_token: &Token| {
        // TODO: This task should be for ASTRootItem
        match initial_token {
            Token::Symbol(TokenSymbol::EOF) => Ok(None),
            _ => ASTNode::transcribe(cursor, ctx).map(|astn| astn.map(ASTRootItem::Node))
        }
    }
}