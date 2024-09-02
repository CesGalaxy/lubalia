#[derive(Debug)]
pub enum ExpressionNode {
    Literal,
    Identifier,
    Binary,
}