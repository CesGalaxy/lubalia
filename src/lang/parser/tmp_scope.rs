use crate::{lang::{old_parser::node::TreeNode, token::Token}, utils::transcriber::cursor::TranscriberCursor};

use super::{error::ParserError, node::ASTNode};

pub struct ScopeNode {
    pub name: Option<String>,
    pub children: Vec<TreeNode>
}

impl ScopeNode {
    pub fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTNode>, ParserError> {
        Ok(None)
    }
}