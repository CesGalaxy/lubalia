use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::{
    data::DataValue,
    node::{ASTNode, Node},
    lang::{parser::error::ParserError, token::{symbol::TokenSymbol, Token}},
    vm::{context::Context, tick::VMTick}
};

use super::StatementNode;

/// A scope that will run a set of nodes in a new context (child of the current one)
#[derive(Debug, Clone)]
pub struct ScopeStruct {
    /// The nodes to execute inside the scope
    nodes: Vec<ASTNode>
}

impl Node for ScopeStruct {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ScopeStruct>, ParserError> {
        // Scopes should start with an opening brace
        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::BraceOpen)) {
            return Err(ParserError::Expected("start@scope/sym <sym:brace:open> '{'".to_string()));
        }

        let mut nodes = vec![];

        // Save all nodes found inside the scope until a closing brace is found (and ends the scope)
        // FIXME: When None it will loop forever
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

        // Scopes should end with a closing brace
        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::BraceClose)) {
            return Err(ParserError::Expected("end@scope/sym <sym:brace:close> '}'".to_string()));
        }

        Ok(Some(ScopeStruct { nodes }))
    }
}

impl StatementNode for ScopeStruct {
    // TODO: This code is shit. But works!
    /// Run the scope (with it's own generated child context),
    /// it will return a value (NULL if not provided).
    fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        let mut result = None;

        let using_global_context = tick.context.is_none();

        tick.context = Some(Box::new(Context::with_parent(std::mem::take(tick.get_context()))));

        for node in &self.nodes {
            if let Some(value) = node.execute(tick) {
                result = Some(value);
                break;
            }
        }

        if let Some(child) = std::mem::take(&mut tick.context) {
            if let Some(parent) = child.parent {
                if using_global_context {
                    tick.vm.global = *parent;
                } else {
                    tick.context = Some(parent)
                }
            }
        }

        // [SemiExpression] Return the value returned (if any) by the executed branch
        result
    }
}

impl std::fmt::Display for ScopeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{\n")?;

        for node in &self.nodes {
            write!(f, "\t{}\n", node)?;
        }

        write!(f, "}}")
    }
}