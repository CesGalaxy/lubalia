pub mod error;
pub mod root;
pub mod node;

use error::ParserError;
use node::ASTNode;
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
    match initial_token {
        Token::Literal(literal) => {
            println!("{literal}");
            // NOT REQUIRED, TRANSCRIBER WILL TAKE CARE OF IT: cursor.next();
            Ok(None)
        },
        Token::EOF => Ok(None),
        _ => ASTNode::transcribe(cursor, initial_token).map(|a| a.map(ASTRootItem::Node))
    }
}