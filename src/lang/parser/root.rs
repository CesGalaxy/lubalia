use super::node::ASTNode;

/// A script can contain multiple items,
/// in the case of nodes, they will be executed.
#[derive(Debug, Clone)]
pub enum ASTRootItem {
    Node(ASTNode)
}