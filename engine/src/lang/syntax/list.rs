use lubalia_utils::{loop_through::LoopThrough, transcriber::cursor::TranscriberCursor};

use crate::lang::{parser::context::ParsingContext, token::Token};

use super::node::{Node, NodeParserTickResult};

pub struct NodeList<T: Node> {
    _nodes: Vec<T>,
}

impl<T: Node> NodeList<T> {
    pub fn transcribe_list(
        _cursor: &mut TranscriberCursor<Token>,
        _ctx: &mut ParsingContext,
        _transcriber: impl Fn(&mut TranscriberCursor<Token>, &mut ParsingContext) -> NodeParserTickResult<T>,
        _loop_condition: LoopThrough<Token>
    ) -> NodeParserTickResult<Self> {
        todo!()
    }
}