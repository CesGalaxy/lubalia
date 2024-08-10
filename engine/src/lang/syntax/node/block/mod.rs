pub mod meta;
pub mod transcription;

use std::{cell::RefCell, fmt};

use meta::BlockMetadata;

use crate::vm::{scope::Scope, VM};

use super::{statement::{StatementNode, StatementResult}, ASTNode};

/// A scope that will run a set of nodes in a new context (child of the current one)
#[derive(Debug, Clone)]
pub struct BlockStruct {
    /// The nodes to execute inside the scope
    nodes: Vec<ASTNode>,

    /// The ID the scope will be referenced by
    /// TODO: Implement this. How? This needs a new token
    #[allow(dead_code)]
    name: String,

    #[allow(dead_code)]
    meta: BlockMetadata
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