use crate::{
    lang::{
        lexer::token::{Token, TokenLiteral},
        parser::{
            data::DataValue,
            exception::{ExcpectedToken, ParserError, ParserException},
            machine::ParsingMachine,
            node::{Node, NodeFactory},
        },
    },
    vm::scope::Scope,
};

use super::ExpressionNode;

#[derive(Debug, Clone)]
pub struct LiteralExpresionNode(pub DataValue);

impl Node for LiteralExpresionNode {}

impl NodeFactory for LiteralExpresionNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        match m.consume() {
            Some(Token::Literal(TokenLiteral::Number(n))) => Ok(Self(DataValue::Number(n))),
            Some(Token::Literal(TokenLiteral::String(s))) => Ok(Self(DataValue::String(s))),
            Some(Token::Keyword(keyword)) => match keyword.as_str() {
                "true" => Ok(Self(DataValue::Boolean(true))),
                "false" => Ok(Self(DataValue::Boolean(false))),
                _ => Err(m.except(ParserException::TokenExpected(ExcpectedToken::Keyword("<literal@keyword>"))))
            },
            _ => Err(m.except(ParserException::TokenExpected(ExcpectedToken::Literal("<literal>")))),
        }
    }
}

impl ExpressionNode for LiteralExpresionNode {
    /// Returns the literal value
    fn evaluate(&self, _scope: &Scope) -> DataValue {
        self.0.clone()
    }
}

impl From<LiteralExpresionNode> for DataValue {
    fn from(node: LiteralExpresionNode) -> Self {
        node.0
    }
}

impl std::fmt::Display for LiteralExpresionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
