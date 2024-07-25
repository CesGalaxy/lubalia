use lubalia_utils::transcriber::cursor::TranscriberCursor;

use crate::{
    node::Node,
    lang::{parser::error::ParserError, token::Token},
};

use super::BinaryExpression;

impl Node for BinaryExpression {
    fn transcribe(_cursor: &mut TranscriberCursor<Token>) -> Result<Option<BinaryExpression>, ParserError> {
        Ok(None)
    }
}