use crate::{
    lang::{parser::{data::DataValue, error::ParserError, node::{ASTNode, Node}}, token::{Token, TokenSymbol}},
    utils::transcriber::cursor::TranscriberCursor,
    vm::{context::Context, VMTick}
};

use super::ExpressionNode;

#[derive(Debug, Clone)]
pub struct ScopeStruct {
    code: Vec<ASTNode>
}

impl Node for ScopeStruct {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ScopeStruct>, ParserError> {
        let mut buffer = vec![];

        // TODO: Integrate this in the cursor
        if !cursor.consume().is_some_and(|t| t == &Token::Symbol(TokenSymbol::BraceOpen)) {
            return Err(ParserError::Expected("start@scope/sym <sym:brace:open> '{'".to_string()));
        }

        while Some(&Token::Symbol(TokenSymbol::BraceClose)) != cursor.peek() {
            let initial_position = cursor.pos;

            if let Some(node) = ASTNode::transcribe(cursor)? {
                buffer.push(node);
            }

            // FIXME: This is a bad idea
            if cursor.pos == initial_position {
                cursor.next();
            }
        }

        if !cursor.consume().is_some_and(|t| t == &Token::Symbol(TokenSymbol::BraceClose)) {
            return Err(ParserError::Expected("end@scope/sym <sym:brace:close> '}'".to_string()));
        }

        Ok(Some(ScopeStruct {
            code: buffer,
        }))
    }
}

impl ExpressionNode for ScopeStruct {
    // TODO: This code is shit. But works!
    /// Run the scope (with it's own generated child context),
    /// it will return a value (NULL if not provided).
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        let mut result = None;

        let using_global_context = tick.context.is_none();

        let tick_ctx = tick.get_context();

        let parent_ctx = std::mem::take(tick_ctx);

        let child_ctx = Context::with_parent(Some(parent_ctx));

        tick.context = Some(Box::new(child_ctx));

        for node in &self.code {
            result = node.execute(tick);
        }

        let final_child_ctx = std::mem::take(&mut tick.context);

        tick.context = if using_global_context {
            None
        } else {
            if let Some(child) = final_child_ctx {
                if let Some(parent) = child.parent {
                    Some(parent)
                } else {
                    None
                }
            } else {
                None
            }
        };

        result.unwrap_or_default()
    }
}