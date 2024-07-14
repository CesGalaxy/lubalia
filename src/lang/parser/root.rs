use super::node::ASTNode;

#[derive(Debug, Clone)]
pub enum ASTRootItem {
    Node(ASTNode)
}