use lubalia_utils::transcriber::cursor::TranscriberCursor;

use crate::{
    lang::token::Token,
    node::{Node, NodeParserTickResult}
};

use super::BinaryExpression;

impl Node for BinaryExpression {
    fn transcribe(_cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> {
        Ok(None)
    }
}