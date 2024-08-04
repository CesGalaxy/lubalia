use std::{collections::HashMap, fmt};

use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::{
    lang::{parser::error::ParserError, token::{symbol::TokenSymbol, Token}},
    node::{ASTNode, Node, NodeParserTickResult},
    vm::{context::Context, tick::VMTick}
};

use super::{StatementNode, StatementResult};

/// A scope that will run a set of nodes in a new context (child of the current one)
#[derive(Debug, Clone)]
pub struct ScopeStruct {
    /// The nodes to execute inside the scope
    nodes: Vec<ASTNode>,

    /// The ID the scope will be referenced by
    /// TODO: Implement this. How? This needs a new token
    #[allow(dead_code)]
    name: String
}

impl Node for ScopeStruct {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> where Self: Sized {
        // Scopes should start with an opening brace
        cursor.expect(&Token::Symbol(TokenSymbol::BraceOpen), ParserError::Expected("start@scope/sym <sym:brace:open> '{'".to_string()))?;

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
        cursor.expect(&Token::Symbol(TokenSymbol::BraceClose), ParserError::Expected("end@scope/sym <sym:brace:close> '}'".to_string()))?;

        Ok(Some(ScopeStruct { nodes, name: String::new() }))
    }
}

impl StatementNode for ScopeStruct {
    // TODO: This code is shit. But works!
    /// Run the scope (with it's own generated child context)
    fn execute(&self, tick: &mut VMTick) -> Option<StatementResult> {
        let mut result = None;

        let parent_ctx = tick.context.clone().map(|c| *c);
        tick.context = Some(Box::new(Context::with_parent(HashMap::new(), parent_ctx)));

        for node in &self.nodes {
            if let Some(value) = node.execute(tick) {
                result = Some(value);
                break;
            }
        }

        tick.context = tick.context.clone().map(|child| child.parent.clone()).flatten();

        result.map(StatementResult::Return)
    }
}

impl fmt::Display for ScopeStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n")?;

        for node in &self.nodes {
            write!(f, "\t{}\n", node)?;
        }

        write!(f, "}}")
    }
}