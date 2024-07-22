use crate::{
    engine::node::Node,
    lang::{parser::error::ParserError, token::Token},
    utils::transcriber::cursor::TranscriberCursor
};

use super::BinaryExpression;

impl Node for BinaryExpression {
    fn transcribe(_cursor: &mut TranscriberCursor<Token>) -> Result<Option<BinaryExpression>, ParserError> {
        Ok(None)
    }
}