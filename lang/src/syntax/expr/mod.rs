#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Literal,
    Identifier,
    Binary,
}