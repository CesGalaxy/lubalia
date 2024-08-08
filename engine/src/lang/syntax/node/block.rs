use std::{cell::RefCell, fmt};

use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::{
    lang::{parser::{context::ParsingContext, error::ParserError}, token::{symbol::TokenSymbol, Token}},
    vm::{scope::Scope, VM}
};

use super::{statement::{StatementNode, StatementResult}, ASTNode, Node, NodeParserTickResult};

/// A scope that will run a set of nodes in a new context (child of the current one)
#[derive(Debug, Clone)]
pub struct BlockStruct {
    /// The nodes to execute inside the scope
    nodes: Vec<ASTNode>,

    /// The ID the scope will be referenced by
    /// TODO: Implement this. How? This needs a new token
    #[allow(dead_code)]
    name: String
}

impl Node for BlockStruct {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // Blocks should start with an opening brace
        cursor.expect(&Token::Symbol(TokenSymbol::BraceOpen), ParserError::Expected("start@scope/sym <sym:brace:open> '{'".to_string()))?;

        let mut nodes = vec![];

        // Save all nodes found inside the scope until a closing brace is found (and ends the scope)
        // FIXME: When None it will loop forever
        while Some(&Token::Symbol(TokenSymbol::BraceClose)) != cursor.peek() {
            let initial_position = cursor.pos;

            if let Some(node) = ASTNode::transcribe(cursor, ctx)? {
                nodes.push(node);
            }

            // FIXME: This is a bad idea
            if cursor.pos == initial_position {
                cursor.next();
            }
        }

        // Blocks should end with a closing brace
        cursor.expect(&Token::Symbol(TokenSymbol::BraceClose), ParserError::Expected("end@scope/sym <sym:brace:close> '}'".to_string()))?;

        Ok(Some(BlockStruct { nodes, name: String::new() }))
    }
}

impl BlockStruct {
    // TODO: This code is shit. But works!
    /// Run the block (with its own generated child context)
    pub fn execute(&self, vm: &mut VM, scope: &RefCell<Scope>) -> Option<StatementResult> {
        let child = Scope::with_parent(scope.borrow());
        let child = RefCell::new(child);

        for node in &self.nodes {
            if let Some(value) = node.execute(vm, &child) {
                return Some(value);
            }
        }

        None
    }
}

impl fmt::Display for BlockStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n")?;

        for node in &self.nodes {
            write!(f, "\t{}\n", node)?;
        }

        write!(f, "}}")
    }
}