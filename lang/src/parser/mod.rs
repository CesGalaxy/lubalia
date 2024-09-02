pub mod error;
pub mod cursor;

use error::ParserError;
use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::syntax::{list::NodeList, node::{Node, NodeFactory}};

use super::token::Token;

pub type ParserCursor<'a> = TranscriberCursor<'a, Token>;

/// Transcribe a list of tokens into an AST (Abstract Syntax Tree).
pub fn parser(tokens: Vec<Token>) -> Result<Vec<Node>, ParserError> {
    //let mut transcription = transcriber(tokens, parser_tick);
    let mut cursor = TranscriberCursor::new(&tokens);

    let list = NodeList::<Node>::parse(&mut cursor)?.unwrap();

    let nodes: Vec<Node> = list.into();

    Ok(nodes)
}