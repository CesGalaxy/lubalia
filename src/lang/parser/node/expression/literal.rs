use crate::{
    lang::{
        token::{Token, TokenLiteral},
        parser::{
            data::DataValue,
            exception::{ExpectedToken, ParserError, ParserException},
            machine::ParsingMachine,
            node::{Node, NodeFactory},
        },
    },
    vm::context::Context,
};

use super::ExpressionNode;

/// A literal (constant and static) value, contains a DataValue that doesn't depend on anything else
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
                _ => Err(m.err(ParserException::TokenExpected(ExpectedToken::Keyword("<literal@keyword>"))))
            },
            _ => Err(m.err(ParserException::TokenExpected(ExpectedToken::Literal("<literal>")))),
        }
    }
}

impl ExpressionNode for LiteralExpresionNode {
    /// Returns the literal value
    fn evaluate(&self, _scope: &Context) -> DataValue {
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
