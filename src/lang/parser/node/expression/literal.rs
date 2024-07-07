use crate::{lang::{lexer::token::{Token, TokenLiteral}, parser::{data::DataValue, exception::{ExcpectedToken, ParsingMachineError, ParsingMachineException}, machine::ParsingMachine, node::{Node, NodeFactory}}}, vm::scope::Scope};

use super::ExpressionNode;

#[derive(Debug)]
pub struct LiteralExpresionNode(pub DataValue);

impl ExpressionNode for LiteralExpresionNode {
    fn evaluate(&self, _scope: &Scope) -> DataValue {
        self.0.clone()
    }
}

impl NodeFactory for LiteralExpresionNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParsingMachineError> {
        match m.consume() {
            Some(Token::Literal(TokenLiteral::Number(n))) => Ok(Self(DataValue::Number(n))),
            Some(Token::Literal(TokenLiteral::String(s))) => Ok(Self(DataValue::String(s))),
            _ => Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Literal("<any>"))))
        }
    }
}

impl Node for LiteralExpresionNode {}

impl From<LiteralExpresionNode> for DataValue {
    fn from(node: LiteralExpresionNode) -> Self {
        node.0
    }
}