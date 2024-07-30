use std::fmt;

use super::node::ASTNode;

/// A script can contain multiple items,
/// in the case of nodes, they will be executed.
#[derive(Debug, Clone)]
pub enum ASTRootItem {
    /// A node that will be executed by the VM
    Node(ASTNode)
}

impl fmt::Display for ASTRootItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTRootItem::Node(node) => write!(f, "{}", node)
        }
    }
}