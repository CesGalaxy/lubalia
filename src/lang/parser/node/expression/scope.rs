use crate::{
    lang::{parser::{data::DataValue, error::ParserError, node::{ASTNode, Node}}, token::{Token, TokenSymbol}},
    utils::transcriber::cursor::TranscriberCursor,
    vm::{context::Context, VMTick}
};

use super::ExpressionNode;

#[derive(Debug, Clone)]
pub struct ScopeStruct {
    /// The nodes to execute inside the scope
    nodes: Vec<ASTNode>
}

impl Node for ScopeStruct {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ScopeStruct>, ParserError> {
        let mut nodes = vec![];

        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::BraceOpen)) {
            return Err(ParserError::Expected("start@scope/sym <sym:brace:open> '{'".to_string()));
        }

        while Some(&Token::Symbol(TokenSymbol::BraceClose)) != cursor.peek() {
            let initial_position = cursor.pos;

            if let Some(node) = ASTNode::transcribe(cursor)? {
                nodes.push(node);
            }

            // FIXME: This is a bad idea
            if cursor.pos == initial_position {
                cursor.next();
            }
        }

        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::BraceClose)) {
            return Err(ParserError::Expected("end@scope/sym <sym:brace:close> '}'".to_string()));
        }

        Ok(Some(ScopeStruct { nodes }))
    }
}

impl ExpressionNode for ScopeStruct {
    // TODO: This code is shit. But works!
    /// Run the scope (with it's own generated child context),
    /// it will return a value (NULL if not provided).
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        let mut result = None;

        let using_global_context = tick.context.is_none();

        tick.context = Some(Box::new(Context::with_parent(std::mem::take(tick.get_context()))));

        for node in &self.nodes {
            if let Some(value) = node.execute(tick) {
                result = Some(value);
                break;
            }
        }

        tick.context = if !using_global_context {
            if let Some(child) = std::mem::take(&mut tick.context) {
                if let Some(parent) = child.parent {
                    Some(parent)
                } else { None }
            } else { None }
        } else { None };

        result.unwrap_or_default()
    }
}